use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpListener;

// 反復サーバ(クライアントの文字列を返すだけのechoサーバ)
//
// 使い方:
// こちらのアプリケーションを起動後､別ターミナルからtcpでlocalhost:10000にアクセス
//
// 実行例:
// $ socat stdio tcp:localhost:10000
// Hello, Rust!
// Hello, Rust!
pub fn start() {
    // 10000番のポートをリッスンする
    let listener = TcpListener::bind("127.0.0.1:10000").unwrap();

    // コネクション要求をアクセプトする
    while let Ok((stream, _)) = listener.accept() {
        // Read用にストリームをコピー
        let stream0 = stream.try_clone().unwrap();

        let mut reader = BufReader::new(stream0);
        let mut writer = BufWriter::new(stream);

        // 一行リードし
        let mut buffer = String::new();
        reader.read_line(&mut buffer).unwrap();

        // やまびこ的な感じでそのまま返す
        writer.write(buffer.as_bytes()).unwrap();
        writer.flush().unwrap();
    }
}
