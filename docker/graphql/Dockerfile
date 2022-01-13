FROM rust:1.57-alpine

ARG uid

RUN apk update && apk add shadow \
    && [ -z "${uid}" ] \
      && echo -e "\n$(printf '\033')[31;1m   abort: empty Dockerfile argument 'uid' $(printf '\033')[m\n" && exit 1 \
      || true \
    && useradd -m rust_app -u ${uid} \
#   copy root user env
    && env | grep -e RUST -e PATH -e CARGO >> /home/rust_app/.profile \
    && mkdir -p /cargo_target \
    && chown rust_app /cargo_target

RUN apk add musl-dev \
    && su - rust_app -c " \
      set -a \
      && source ~/.profile \
      && set +a \
      && cargo install cargo-watch"

USER rust_app
