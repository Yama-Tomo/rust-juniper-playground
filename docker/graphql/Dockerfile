FROM rust:1.57

ARG uid

RUN apt-get update \
    && [ -z "${uid}" ] \
      && echo -e "\n$(printf '\033')[31;1m   abort: empty Dockerfile argument 'uid' $(printf '\033')[m\n" && exit 1 \
      || true \
    && useradd -m rust_app -u ${uid} \
    && mkdir -p /cargo_target \
    && chown rust_app /cargo_target

# install mold
RUN apt-get install -y --no-install-recommends build-essential git clang-11 cmake libstdc++-10-dev libssl-dev libxxhash-dev zlib1g-dev \
    && ln -s /usr/bin/clang-11 /usr/bin/clang \
    && ln -s /usr/bin/clang++-11 /usr/bin/clang++ \
    && cd $HOME \
    && git clone https://github.com/rui314/mold.git \
    && cd mold \
    && git checkout $(git describe --tags --abbrev=0) \
    && make -j$(nproc) \
    && make install

RUN su rust_app -c " \
      rustup component add clippy rustfmt && \
      cargo install cargo-watch sqlx-cli"

USER rust_app
