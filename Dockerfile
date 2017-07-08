FROM ubuntu:xenial

ENV PATH "/root/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"

RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y build-essential curl libmysqlclient-dev libpq-dev libsqlite3-dev && \
    curl https://sh.rustup.rs -sSf | bash -s -- -y && \
    rustup toolchain install nightly-x86_64-unknown-linux-gnu && \
    rustup default nightly && \
    cargo install cargo-make && \
    cargo install diesel_cli && \
    mkdir -p /usr/src/rust-blog-api

WORKDIR /usr/src/rust-blog-api
VOLUME ["/usr/src/rust-blog-api"]
COPY . /usr/src/rust-blog-api

EXPOSE 80

CMD ["cargo", "make", "run"]
