FROM rust:1.90.0-slim-trixie
ENV DEBIAN_FRONTEND=noninteractive

LABEL authors="wi"

### Dependencies (as of 11/12/2025) ###
# + sqlite3             Provides the SQLite3 library       (needed by Diesel ORM)
# + libsqlite3-dev      Provides header files for SQLite3  (needed by Diesel ORM)
# + mold                Linker
# + clang               Compiler

RUN apt update && \
    apt install --no-install-recommends -y sqlite3 libsqlite3-dev && \
    apt install --no-install-recommends -y clang mold && \
    rm -rf /var/lib/apt/lists/*

ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=clang
ENV RUSTFLAGS="-C link-arg=-fuse-ld=mold"

RUN cargo install diesel_cli --no-default-features --features sqlite

### Setup ###

RUN groupadd --gid 1000 crabdrive && useradd --uid 1000 --gid 1000 -m crabdrive

RUN mkdir -p /storage/ && chown -R crabdrive:crabdrive /storage/

USER crabdrive

WORKDIR /app/crabdrive/