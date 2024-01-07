FROM rust:latest

WORKDIR /usr/src/axum_sample

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -f target/release/deps/axum_sample

# 実際のソースコードをコピー
COPY src ./src

# 本番ビルド
RUN cargo install --path .

CMD ["axum_sample"]