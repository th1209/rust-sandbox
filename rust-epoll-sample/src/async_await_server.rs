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
                    // 一度登録したファイルディスクリプタの再設定. 上記のEPOLLONESHOTによる最適化(登録済みのファイルディスクリプタを使い回す)意図.
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
