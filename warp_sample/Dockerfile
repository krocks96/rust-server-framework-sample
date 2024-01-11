FROM rust:latest

WORKDIR /usr/src/warp_sample

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -f target/release/deps/warp_sample

COPY src ./src

RUN cargo install --path .

CMD ["warp_sample"]