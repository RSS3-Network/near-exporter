FROM rust:1.77-slim-bullseye as builder

WORKDIR /usr/src/app

RUN apt update && apt install -y pkg-config libssl-dev

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt update && apt install -y libssl1.1 openssl ca-certificates

COPY --from=builder /usr/src/app/target/release/near-exporter /usr/local/bin/near-exporter
ENTRYPOINT [ "/usr/local/bin/near-exporter" ]