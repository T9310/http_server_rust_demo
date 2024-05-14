FROM rust:latest as builder

WORKDIR /app/http_server_demo

COPY Cargo.toml Cargo.lock ./
RUN mkdir src \
    && echo "fn main() {println!(\"dummy\")}" > src/main.rs \
    && cargo build --release

COPY . .

COPY html_files/ ./html_files/

RUN cargo install --path .

FROM ubuntu:22.04

RUN apt-get update \
    && apt-get install -y libc6 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/http_server_demo .

COPY --from=builder /app/http_server_demo/html_files ./html_files

EXPOSE 7878

CMD ["./http_server_demo"]
