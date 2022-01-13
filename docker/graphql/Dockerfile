FROM rust:1.57

ARG uid

RUN apt-get update \
    && [ -z "${uid}" ] \
      && echo -e "\n$(printf '\033')[31;1m   abort: empty Dockerfile argument 'uid' $(printf '\033')[m\n" && exit 1 \
      || true \
    && useradd -m rust_app -u ${uid} \
    && mkdir -p /cargo_target \
    && chown rust_app /cargo_target

RUN su rust_app -c " \
      cargo install cargo-watch"

USER rust_app
