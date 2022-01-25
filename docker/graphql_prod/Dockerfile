FROM ekidd/rust-musl-builder:latest as builder

COPY Cargo.toml .
COPY Cargo.lock .
COPY src src

RUN cargo build --release

FROM alpine as prod

ENV PORT 8080

WORKDIR /app

COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/rust-juniper-playground /app/main
EXPOSE 8080

CMD ["/app/main"]
