FROM rust:1.66

WORKDIR /workspace

# socatコマンドのインストール(echoサーバの動作確認用)
RUN apt-get update
RUN apt-get -y install socat

# Rust関連の環境設定
RUN rustup component add rustfmt
RUN rustup component add clippy
RUN cargo install cargo-watch

COPY Cargo.toml Cargo.toml

# 予め空のソースファイルでビルドしておくことで､Cargo.tomlに変更がない場合はキャッシュする
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY ./src ./src

# ビルドしたもののうち､アプリケーションに関するものだけ削除
RUN rm -f target/release/deps/*

RUN cargo build --release
