# we need to start with a newer rust version because diesel cli needs rust 1.86
FROM rust:1.93.1-alpine3.23

LABEL authors="wi"

RUN apk update && apk add trunk bash git sqlite-dev sqlite-libs sqlite-static

RUN adduser crabdrive -s /bin/bash -D

RUN mkdir -p /storage/ && chown -R crabdrive:crabdrive /storage/

USER crabdrive

RUN cargo install diesel_cli --no-default-features --features sqlite
RUN rustup install 1.85.0
RUN rustup default 1.85.0
RUN rustc --version
RUN rustup target add wasm32-unknown-unknown

### Setup ###

WORKDIR /home/crabdrive/crabdrive/