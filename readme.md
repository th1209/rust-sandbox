### 概要

- Rustの練習用に､Rustで色々やってみるレポジトリ

### 各レポジトリ

#### rust-async-programming

- Rustによる非同期プログラミングのコード色々
- Executor/Task/Wakerモデルによる非対称コルーチン
    - [並行プログラミング入門 5章](https://www.oreilly.co.jp/books/9784873119595/)

#### rust-epoll-sample

- epollを使ったIO多重化の例
    - [並行プログラミング入門 5章](https://www.oreilly.co.jp/books/9784873119595/)
- epollなので､Dockerを使ったLinux環境で実行する

#### rust-recursion

- Rustによる再帰処理の練習
- 以下の再帰の練習記事を､Rustで実装してみたもの.
    - [ぐるぐる〜 再帰で考える](https://bleis-tift.hatenablog.com/entry/20120119/1326944722)
- 末尾再帰最適化については､以下記事を参照
    - [Qiita 末尾再帰による最適化](https://qiita.com/pebblip/items/cf8d3230969b2f6b3132)

#### simple-cli

- CLIプログラムのサンプル(単純な逆ポーランド記法の計算機)
    - [実践Rustプログラミング入門 4章](https://www.shuwasystem.co.jp/book/9784798061702.html)