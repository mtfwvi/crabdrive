# Requirements
[docs/submission/requirements.md](docs/submission/requirements.md) 

# Deployment
### Podman/Docker
To start the container run `podman compose up --build` / `docker compose up --build`

### Manual
Build the client and server on you own machine

Prerequisites:
- Rust 1.85.0 or later installed.
- trunk installed (install with `cargo install trunk`)
- available `wasm32-unknown-unknown` target (add with `rustup target add wasm32-unknown-unknown`)
- TCP port `2722` (or port set by `CRABDRIVE_ADDR`) available on the host.
```
git clone https://github.com/mtfwvi/crabdrive
cd ./client
trunk build --release
cd ..
cargo run --bin crabdrive-server --release
```

# Development
### Native
Prerequisites:
- Rust 1.85.0 or later installed.
- trunk installed (install with `cargo install trunk`)
- available `wasm32-unknown-unknown` target (add with `rustup target add wasm32-unknown-unknown`)
- TCP port `2722` (or port set by `CRABDRIVE_ADDR`) available on the host.
- (optional) `leptosfmt` (install with `cargo install leptosfmt`) to more easily format the view macros inside leptos.
- (optional) `diesel_cli` (install with `cargo install diesel_cli`, requires a local sqlite installation) to manage db migrations

To run the client on its own run `trunk server --open` from the `./client` folder - to build it for serving via the server run `trunk build --release`.

The server can be run/built with `cargo run --bin crabdrive-server --release` / `cargo build --bin crabdrive-server --release`

The server will try to serve the files under `./client/target` (relative to the working directory)

### Devcontainer (extremely slow and not recommended)
A dev container for vs code is provided in this repository

# Server

The default address is http://127.0.0.1:2722/ or http://localhost:2722/, however you can configure it using the
`CRABDRIVE_ADDR` / `CRABDRIVE_PORT` environment variables (`server.addr` / `server.port` resp.):

Pressing `Crtl + C` will gracefully shut down the server.

### Developement Resources

- [Logging](./docs/developement/server/logging.md)
- [Shared State Management](./docs/developement/server/state.md)

# Configuration
You can configure the server by using a TOML file or Environment overrides.
By default, the server uses a development-sensitive configuration.

By default, the server attempts to load the configuration from `./crabdrive.toml`. To pass in a custom path to a
configuration file, use: `./crabdrive-server -C <PATH>`.

**generation**: You can also generate a template using `./crabdrive-server --generate-config-template <PATH>`.
