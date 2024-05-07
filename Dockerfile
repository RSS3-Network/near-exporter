FROM messense/rust-musl-cross:x86_64-musl as builder

WORKDIR /usr/src

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/target \
    cargo install --path .


FROM alpine as runner

RUN apk --no-cache add ca-certificates

COPY --from=builder /root/.cargo/bin/near-exporter /usr/local/bin/near-exporter

USER 1000

ENTRYPOINT ["/usr/local/bin/near-exporter"]