FROM rust:latest

WORKDIR /usr/src/rocket_sample

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -f target/release/deps/rocket_sample

COPY src ./src

RUN cargo install --path .

CMD ["rocket_sample"]