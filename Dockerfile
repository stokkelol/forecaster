FROM rust:1.76.0-bullseye as builder

COPY src/dummy.rs src/main.rs src/
COPY Cargo.toml .
COPY Cargo.lock .

RUN apt-get update
RUN apt-get -y install musl-tools pkg-config libssl-dev

RUN rustup target add x86_64-unknown-linux-musl

## hack for dependencies caching
RUN sed -i 's#src/main.rs#src/dummy.rs#' Cargo.toml
RUN cargo build --release

RUN sed -i 's#src/dummy.rs#src/main.rs#' Cargo.toml
RUN PKG_CONFIG_ALLOW_CROSS=1 cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:latest

COPY --from=builder /target/x86_64-unknown-linux-musl/release/forecaster /
COPY docker-entrypoint.sh /

RUN chmod +x /forecaster

CMD ["./docker-entrypoint.sh"]