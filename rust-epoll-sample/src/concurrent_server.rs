use nix::sys::epoll::{
    epoll_create1, epoll_ctl, epoll_wait, EpollCreateFlags, EpollEvent, EpollFlags, EpollOp,
};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
use std::os::unix::io::{AsRawFd, RawFd};

pub fn start() {
    // localhostの10000番ポートでリッスン
    let listener = TcpListener::bind("127.0.0.1:10000").unwrap();

    // epoll用のオブジェクトを生成
    // ※ epoll用のファイルディスクリプタをそのままオブジェクトっぽく扱っている. C言語的なハンドルでのやり取りをイメージすると分かりやすいか.
    let epfd = epoll_create1(EpollCreateFlags::empty()).unwrap();

    // epollに､リッスン用のソケットを監視対象として追加
    // ※ epoll_ctlは監視対象の追加・削除・修正
    let listen_fd = listener.as_raw_fd();
    let mut ev = EpollEvent::new(EpollFlags::EPOLLIN, listen_fd as u64);
    epoll_ctl(epfd, EpollOp::EpollCtlAdd, listen_fd, &mut ev).unwrap();

    let mut fd2buf: HashMap<i32, (BufReader<TcpStream>, BufWriter<TcpStream>)> = HashMap::new();
    let mut events = vec![EpollEvent::empty(); 1024];

    // ※epoll_waitで､イベントの発生を監視(イベントが発生したら､その分のファイルディスクリプタが返る)
    while let Ok(nfds) = epoll_wait(epfd, &mut events, -1) {
        for n in 0..nfds {
            if events[n].data() == listen_fd as u64 {
                process_listen_socket(&listener, epfd, &mut fd2buf);
            } else {
                // クライアントソケットの場合
                let fd = events[n].data() as RawFd;
                process_client_socket(fd, epfd, &mut fd2buf);
            }
        }
    }
}

fn process_listen_socket(
    listener: &TcpListener,
    epfd: i32,
    fd2buf: &mut HashMap<i32, (BufReader<TcpStream>, BufWriter<TcpStream>)>,
) {
    if let Ok((stream, _)) = listener.accept() {
        let fd = stream.as_raw_fd();

        let stream0 = stream.try_clone().unwrap();
        let reader = BufReader::new(stream0);
        let writer = BufWriter::new(stream);
        fd2buf.insert(fd, (reader, writer));

        println!("accept: fd = {}", fd);

        // acceptしたファイルディスクリプタをepollの監視対象に追加
        let mut ev = EpollEvent::new(EpollFlags::EPOLLIN, fd as u64);
        epoll_ctl(epfd, EpollOp::EpollCtlAdd, fd, &mut ev).unwrap();
    }
}

fn process_client_socket(
    fd: i32,
    epfd: i32,
    fd2buf: &mut HashMap<i32, (BufReader<TcpStream>, BufWriter<TcpStream>)>,
) {
    // let fd = events[n].data() as RawFd;
    let (reader, writer) = fd2buf.get_mut(&fd).unwrap();

    let mut buf = String::new();
    let n = reader.read_line(&mut buf).unwrap();

    if n == 0 {
        // ※コネクションがクローズしていた場合(read_lineの値が0で判定できる)

        // epollの監視対象からファイルディスクリプタを削除
        let mut ev = EpollEvent::new(EpollFlags::EPOLLIN, fd as u64);
        epoll_ctl(epfd, EpollOp::EpollCtlDel, fd, &mut ev).unwrap();

        fd2buf.remove(&fd);

        println!("closed fd = {}", fd);

        return;
    }

    // 受け取ったバッファを自身にも出力
    print!("read: fd = {} buf = {}", fd, buf);

    // TCPストリームにwriteして､相手に返す
    writer.write(buf.as_bytes()).unwrap();
    writer.flush().unwrap();
}
