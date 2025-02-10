FROM rust:1.84 AS builder
COPY Cargo.lock .
COPY Cargo.toml .
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim AS runner
COPY --from=builder /target/release/rust-sql-exporter /rust-sql-exporter
# RUN apt-get update && apt install -y openssl
CMD ["/rust-sql-exporter"]
