FROM rust:1.66

WORKDIR /workspace

COPY Cargo.toml Cargo.toml

# 予め空のソースファイルでビルドしておくことで､Cargo.tomlに変更がない場合はキャッシュする
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
RUN cargo build --release

# TODO: 転送というより､ホストのマウントにしたほうがよさそう
COPY ./src ./src

# ビルドしたもののうち､アプリケーションに関するものだけ削除
RUN rm -f target/release/deps/*

RUN cargo build --release
