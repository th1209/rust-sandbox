use futures::{
    future::{BoxFuture, FutureExt},
    task::{waker_ref, ArcWake},
};
use nix::{
    errno::Errno,
    sys::{
        epoll::{
            epoll_create1, epoll_ctl, epoll_wait, EpollCreateFlags, EpollEvent, EpollFlags, EpollOp,
        },
        // eventfd: Linux固有のイベント通知インタフェース. 値が0より大きい場合に読み込みイベントが発生.
        eventfd::{eventfd, EfdFlags},
    },
    unistd::write,
};
use std::{
    collections::{HashMap, VecDeque},
    future::Future,
    io::{BufRead, BufReader, BufWriter, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    os::unix::io::{AsRawFd, RawFd},
    pin::Pin,
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc, Mutex,
    },
    task::{Context, Poll, Waker},
};

pub fn start() {
    let executor = Executor::new();
    let spawner = executor.get_spawner();
    let selector = IOSelector::new();

    let server = async move {
        let listener = AsyncListener::listen("127.0.0.1:10000", selector.clone());

        loop {
            let (mut reader, mut writer, addr) = listener.accept().await;
            println!("accept: {}", addr);

            spawner.spawn(async move {
                while let Some(buf) = reader.read_line().await {
                    println!("read: {}, {}", addr, buf);
                    writer.write(buf.as_bytes()).unwrap();
                    writer.flush().unwrap();
                }
                println!("close: {}", addr);
            });
        }
    };

    executor.get_spawner().spawn(server);
    executor.run();
}

fn write_eventfd(fd: RawFd, n: usize) {
    // &usize -> *const u8 へのキャストが一発でできないので､二度キャストする必要がある
    let ptr = &n as *const usize as *const u8;
    let val = unsafe { std::slice::from_raw_parts(ptr, std::mem::size_of_val(&n)) };
    write(fd, &val).unwrap();
}

enum IOOps {
    ADD(EpollFlags, RawFd, Waker),
    REMOVE(RawFd),
}

struct IOSelector {
    wakers: Mutex<HashMap<RawFd, Waker>>,
    queue: Mutex<VecDeque<IOOps>>,
    epfd: RawFd,  // epollのfd
    event: RawFd, // eventfdのfd
}

impl IOSelector {
    pub fn new() -> Arc<Self> {
        let selector = IOSelector {
            wakers: Mutex::new(HashMap::new()),
            queue: Mutex::new(VecDeque::new()),
            epfd: epoll_create1(EpollCreateFlags::empty()).unwrap(),
            event: eventfd(0, EfdFlags::empty()).unwrap(),
        };
        let result = Arc::new(selector);
        let selector = result.clone();
        // メインスレッドとは別スレッドで､epoll監視(ループ)
        std::thread::spawn(move || selector.select());
        return result;
    }

    // ファイルディスクリプタ(とWaker)をepollの監視対象に登録
    fn add_event(
        &self,
        flag: EpollFlags,
        fd: RawFd,
        waker: Waker,
        wakers: &mut HashMap<RawFd, Waker>,
    ) {
        // ※ EPOLLONESHOT: 一度イベントが発生すると､再設定するまではそのファイルディスクリプタへのイベントは通知されない(epollへの登録自体は残る)
        let mut ev = EpollEvent::new(flag | EpollFlags::EPOLLONESHOT, fd as u64);
        if let Err(err) = epoll_ctl(self.epfd, EpollOp::EpollCtlAdd, fd, &mut ev) {
            match err {
                nix::Error::Sys(Errno::EEXIST) => {
                    // 一度登録したファイルディスクリプタの再設定. 上記のEPOLLONESHOTによる最適化の意図(登録済みのファイルディスクリプタを使い回す).
                    epoll_ctl(self.epfd, EpollOp::EpollCtlMod, fd, &mut ev).unwrap();
                }
                _ => {
                    panic!("epoll_ctl: {}", err);
                }
            }
        }
        assert!(!wakers.contains_key(&fd));
        wakers.insert(fd, waker);
    }

    // ファイルディスクリプタ(とWaker)をepollの監視対象から登録解除
    fn rm_event(&self, fd: RawFd, wakers: &mut HashMap<RawFd, Waker>) {
        let mut ev = EpollEvent::new(EpollFlags::empty(), fd as u64);
        epoll_ctl(self.epfd, EpollOp::EpollCtlDel, fd, &mut ev).ok();
        wakers.remove(&fd);
    }

    // epollの監視ループ
    pub fn select(&self) {
        // eventfdも､epollの監視対象としておく
        let mut ev = EpollEvent::new(EpollFlags::EPOLLIN, self.event as u64);
        epoll_ctl(self.epfd, EpollOp::EpollCtlAdd, self.event, &mut ev).unwrap();

        let mut events = vec![EpollEvent::empty(); 1024];
        while let Ok(nfds) = epoll_wait(self.epfd, &mut events, -1) {
            let mut wakers = self.wakers.lock().unwrap();
            for n in 0..nfds {
                if events[n].data() == self.event as u64 {
                    // eventfdの場合.
                    // ファイルディスクリプタとWakerの登録/削除を行う.
                    let mut queue = self.queue.lock().unwrap();
                    while let Some(op) = queue.pop_front() {
                        match op {
                            IOOps::ADD(flag, fd, waker) => {
                                self.add_event(flag, fd, waker, &mut wakers);
                            }
                            IOOps::REMOVE(fd) => self.rm_event(fd, &mut wakers),
                        }
                    }
                } else {
                    // ただのファイルディスクリプタの場合.
                    // wakeし実行キューに追加する.
                    let data = events[n].data() as i32;
                    let waker = wakers.remove(&data).unwrap();
                    waker.wake_by_ref();
                }
            }
        }
    }

    // ファイルディスクリプタとWakerをIOSelectorに登録
    pub fn register(&self, flags: EpollFlags, fd: RawFd, waker: Waker) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(IOOps::ADD(flags, fd, waker));
        write_eventfd(self.event, 1);
    }

    // ファイルディスクリプタ(とWaker)をIOSelectorから登録解除
    pub fn unregister(&self, fd: RawFd) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(IOOps::REMOVE(fd));
        write_eventfd(self.event, 1);
    }
}

// Async(ノンブロッキング)なTcpListenerクラス
struct AsyncListener {
    listener: TcpListener,
    selector: Arc<IOSelector>,
}

impl AsyncListener {
    fn listen(addr: &str, selector: Arc<IOSelector>) -> AsyncListener {
        let listener = TcpListener::bind(addr).unwrap();
        // ここで､ノンブロッキングに設定する
        listener.set_nonblocking(true).unwrap();
        return AsyncListener {
            listener: listener,
            selector: selector,
        };
    }

    // AsyncListnerは､コネクションのアクセプト時にFutureを実装した型を返す -> ノンブロッキングを表現している
    fn accept(&self) -> Accept {
        return Accept { listener: self };
    }
}

impl Drop for AsyncListener {
    fn drop(&mut self) {
        self.selector.unregister(self.listener.as_raw_fd());
    }
}

struct Accept<'a> {
    listener: &'a AsyncListener,
}

impl<'a> Future for Accept<'a> {
    type Output = (AsyncReader, BufWriter<TcpStream>, SocketAddr);
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.listener.listener.accept() {
            Ok((stream, addr)) => {
                let stream0 = stream.try_clone().unwrap();
                return Poll::Ready((
                    AsyncReader::new(stream0, self.listener.selector.clone()),
                    BufWriter::new(stream),
                    addr,
                ));
            }
            Err(err) => {
                if err.kind() == std::io::ErrorKind::WouldBlock {
                    self.listener.selector.register(
                        EpollFlags::EPOLLIN,
                        self.listener.listener.as_raw_fd(),
                        cx.waker().clone(),
                    );
                    return Poll::Pending;
                } else {
                    panic!("accept: {}", err);
                }
            }
        }
    }
}

struct AsyncReader {
    fd: RawFd,
    reader: BufReader<TcpStream>,
    selector: Arc<IOSelector>,
}

impl AsyncReader {
    fn new(stream: TcpStream, selector: Arc<IOSelector>) -> AsyncReader {
        stream.set_nonblocking(true).unwrap();
        return AsyncReader {
            fd: stream.as_raw_fd(),
            reader: BufReader::new(stream),
            selector: selector,
        };
    }

    fn read_line(&mut self) -> ReadLine {
        return ReadLine { reader: self };
    }
}

impl Drop for AsyncReader {
    fn drop(&mut self) {
        self.selector.unregister(self.fd);
    }
}

struct ReadLine<'a> {
    reader: &'a mut AsyncReader,
}

impl<'a> Future for ReadLine<'a> {
    type Output = Option<String>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut line = String::new();
        match self.reader.reader.read_line(&mut line) {
            Ok(0) => return Poll::Ready(None), // 0が返ってきた場合はコネクションクローズ
            Ok(_) => return Poll::Ready(Some(line)),
            Err(err) => {
                if err.kind() == std::io::ErrorKind::WouldBlock {
                    self.reader.selector.register(
                        EpollFlags::EPOLLIN,
                        self.reader.fd,
                        cx.waker().clone(),
                    );
                    return Poll::Pending;
                } else {
                    return Poll::Ready(None);
                }
            }
        }
    }
}

// ↓↓↓↓ 以下､非対称コルーチンで用いたクラス群 ↓↓↓↓

pub struct Task {
    future: Mutex<BoxFuture<'static, ()>>,
    sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let self0 = arc_self.clone();
        arc_self.sender.send(self0).unwrap();
    }
}

pub struct Executor {
    // ※MPSCチャンネルを実行キューに見立てている
    sender: SyncSender<Arc<Task>>,
    receiver: Receiver<Arc<Task>>,
}

impl Executor {
    pub fn new() -> Self {
        let (sender, receiver) = sync_channel(1024);
        return Executor {
            sender: sender.clone(),
            receiver,
        };
    }

    pub fn get_spawner(&self) -> Spawner {
        return Spawner {
            sender: self.sender.clone(),
        };
    }

    pub fn run(&self) {
        while let Ok(task) = self.receiver.recv() {
            let mut future = task.future.lock().unwrap();

            let waker = waker_ref(&task);
            let mut context = Context::from_waker(&waker);

            let poll = future.as_mut().poll(&mut context);
            match poll {
                Poll::Pending => {}
                Poll::Ready(()) => break,
            }
        }
    }
}

pub struct Spawner {
    sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    pub fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(future),
            sender: self.sender.clone(),
        });
        self.sender.send(task).unwrap();
    }
}
