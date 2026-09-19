FROM rust:1.98.1-alpine3.24

LABEL authors="wi"

RUN apk update && apk add trunk bash git sqlite-dev sqlite-libs sqlite-static

RUN adduser crabdrive -s /bin/bash -D

RUN mkdir -p /storage/ && chown -R crabdrive:crabdrive /storage/

USER crabdrive

RUN cargo install diesel_cli --no-default-features --features sqlite
RUN rustc --version
RUN rustup target add wasm32-unknown-unknown

### Setup ###

WORKDIR /home/crabdrive/crabdrive/
