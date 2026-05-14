FROM rust:1.76-slim AS builder

WORKDIR /usr/src/escrowflow_backend

RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
RUN mkdir src
RUN echo "fn main() { println!(\"initialize\"); }" > src/main.rs
RUN cargo build --release

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/escrowflow_backend/target/release/escrowflow_backend /usr/local/bin/escrowflow_backend
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/escrowflow_backend"]
