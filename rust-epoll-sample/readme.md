#### Dockerイメージのビルド・コンテナの実行

```sh
docker build -t rust-epoll-sample .

docker run -it \
--name rust-epoll-sample \
--mount type=bind,source="$(pwd)"/src,target=/workspace/src \
-p 10000:10000 \
rust-epoll-sample \
/bin/bash

```