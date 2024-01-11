FROM rust:latest

WORKDIR /usr/src/salvo_sample

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -f target/release/deps/salvo_sample

COPY src ./src

RUN cargo install --path .

CMD ["salvo_sample"]