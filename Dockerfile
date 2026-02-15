FROM rust:alpine as build

# add dev dependencies
RUN apk update && apk add sqlite-dev trunk
run rustup install 1.85.0
RUN rustup default 1.85.0
RUN rustc --version

WORKDIR /usr/src/crabdrive/
# https://oneuptime.com/blog/post/2026-02-08-how-to-containerize-an-axum-rust-application-with-docker/view
# Cache dependencies by building a dummy project first
COPY Cargo.toml Cargo.lock ./
COPY server/Cargo.toml server/Cargo.toml
COPY client/Cargo.toml client/Cargo.toml
COPY common/Cargo.toml common/Cargo.toml
RUN mkdir server/src && echo "fn main() {}" > server/src/main.rs
RUN mkdir client/src && echo "fn main() {}" > client/src/main.rs
RUN mkdir common/src && echo "" > common/src/lib.rs
RUN cargo check --release

RUN rustup target add wasm32-unknown-unknown

RUN rm client server common Cargo.toml Cargo.lock -rf

#add wasm target for client
RUN rustup target add wasm32-unknown-unknown

# copy source
WORKDIR /usr/src/crabdrive/
COPY . .

# build server
RUN cargo build --bin crabdrive-server --release

# build client
WORKDIR /usr/src/crabdrive/client
RUN trunk build --release

FROM alpine

#add runtime dependencies
RUN apk update && apk add sqlite

COPY --from=build /usr/src/crabdrive/client/dist/ /usr/bin/crabdrive/client/dist/
COPY --from=build /usr/src/crabdrive/target/release/crabdrive-server/ /usr/bin/crabdrive/crabdrive-server 

EXPOSE 2722

RUN mkdir /var/log/crabdrive
WORKDIR /usr/bin/crabdrive/
ENTRYPOINT ["/usr/bin/crabdrive/crabdrive-server"]
