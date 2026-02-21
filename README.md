# Development

Prerequisites:

- Rust 1.85.0 or later installed.
- TCP port `2722` (or port set by `CRABDRIVE_ADDR`) available on the host.
## Docker
To start the docker container run `UID_GID="$(id -u):$(id -g)" docker compose up --build`

## Server

You can start the server by entering `cargo run server`. Once the server has started successfully, a message indicating
the address on which the server is listening should appear on the console.

The default address is http://127.0.0.1:2722/ or http://localhost:2722/, however you can configure it using the
`CRABDRIVE_ADDR` / `CRABDRIVE_PORT` environment variables (`server.addr` / `server.port` resp.):

- On Linux (ZSH):
  ```
  $ export CRABDRIVE_ADDR="127.0.0.1"
  $ export CRABDRIVE_PORT=1234
  $ cargo run server
      Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
      Running `target/debug/crabdrive-server server`
  Server running on http://127.0.0.1:1234
  ^CExiting
  $ unset CRABDRIVE_ADDR CRABDRIVE_PORT
  ```

- On Windows (PowerShell):
  ```
  $ $env:CRABDRIVE_ADDR = "127.0.0.1"
  $ $env:CRABDRIVE_PORT = 1234
  $ cargo run server
    Compiling crabdrive-server v0.1.0
      Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.52s
      Running `target\debug\crabdrive-server.exe server`
  Server running on http://127.0.0.1:1234
  Exiting
  $ Remove-Variable [Environment]:CRABDRIVE_ADDR
  $ Remove-Variable [Environment]:CRABDRIVE_PORT
  ```

Pressing `Crtl + C` will gracefully shut down the server.

### Developement Resources

- [Logging](./docs/developement/server/logging.md)
- [Shared State Management](./docs/developement/server/state.md)

# Configuration

You can configure the server by using a TOML file or Environment overrides.
By default, the server uses a development-sensitive configuration.

By default, the server attempts to load the configuration from `./crabdrive.toml`. To pass in a custom path to a
configuration file, use: `./crabdrive-server -C <PATH>`.

## Configuration Template

You can also generate a template using `./crabdrive-server --generate-config-template <PATH>`.

```toml
# The server configuration.

# The environment the application runs in.
# If this is set to DEV, the application may f.e. log sensitive information.
#
# Possible values:
#  - `DEV`
#  - `PROD`
#
# Default: Derived from Build Type
#
# Can also be specified via environment variable `CRABDRIVE_ENV`.
#
#env =

[server]
# The address the TCP listener binds to. Can be a IPv4 or IPv6.
#
# Default: `127.0.0.1`
#
# Can also be specified via environment variable `CRABDRIVE_ADDR`.
#
#address =

# The port the TCP listener binds to.
#
# Default: `2722`
#
# Can also be specified via environment variable `CRABDRIVE_PORT`.
#
#port =

[db]
# The path to the database file. It can be one of the following formats:
# - `/path/to/db.sqlite` or `file:///path/to/db.sqlite`
# - `:memory:`
#
# Notes:   If the file does not exist, it is created.
#
# Default: `:memory:`
#
# Can also be specified via environment variable `CRABDRIVE_DB_PATH`.
#
#path =

# Number of connections opened to the database and stored in a connection pool.
#
# Notes:   This will open a corresponding number of file handles.
#
# Default: 15
#
# Can also be specified via environment variable `CRABDRIVE_DB_POOLSIZE`.
#
#pool_size =

[storage]
# The path to the storage directory. Can be of the following formats:
# - `/path/to/directory/`
# - `:memory:`
#
# Notes:   The directory is not automatically created.
#
# Default: `:memory:`
#
# Can also be specified via environment variable `CRABDRIVE_STORAGE_DIR`.
#
#dir =

# The storage limit for ALL files, in Bytes.
#
# Notes:   When [`AppConfig::storage_dir`] is set to `:memory:`, this will limit the memory
#          used by the application for storage.
#
# Default: `500_000_000` (500MB)
#
# Can also be specified via environment variable `CRABDRIVE_STORAGE_LIMIT`.
#
#limit =

[log]
# The minimum log level for log messages. All messages below this level will be discarded.
# If this is set to `None`, nothing will be logged. Possible values are:
# - `NONE`
# - `TRACE`
# - `DEBUG`
# - `INFO`
# - `WARN`
# - `ERROR`
#
# Default: `TRACE` when `ENV` is set to `DEV`, otherwise `WARN`
#
# Can also be specified via environment variable `CRABDRIVE_MINIMUM_LOG_LEVEL`.
#
#minimum_level =

# The targets, where logs are piped into. If `env` is set to `DEV` or
# `logs.minimum_level` is set to `NONE`, this is ignored. It may be one of the following
# formats:
# - `/path/to/directory/` (note the trailing slash!) creates rolling, daily logs inside the folder
# - `/path/to/my_log.log` or `/path/to/my_log.json` writes into a file
# - `:stdout:` or `:stderr:`
#
# Notes:   Directories are not automatically created.
#
# Default: `[":stdout:"]` when `env` is set to `DEV`, otherwise `["/var/log/crabdrive/"]`
#
# Can also be specified via environment variable `CRABDRIVE_LOG_TARGETS`.
#
#targets =

```
## Client

To run the client on its own run `trunk server --open` from `/client` - to build it for serving via the server run `trunk build --release`.

Make sure `trunk` is installed (if not install it with `cargo install trunk`) and add the `wasm32-unknown-unknown` target with `rustup target add wasm32-unknown-unknown`.

Optionally, you can install `leptosfmt` with `cargo install leptosfmt` to more easily format the view macros inside
leptos.