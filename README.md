# Development

Prerequisites:

- Rust 1.85.0 or later installed.
- TCP port `2722` (or port set by `CRABDRIVE_ADDR`) available on the host.

## Server

You can start the server by entering `cargo run server`. Once the server has started successfully, a message indicating
the address on which the server is listening should appear on the console.

The default address is http://127.0.0.1:2722/ or http://localhost:2722/, however you can configure it using the
`CRABDRIVE_ADDR` environment variable:

- On Linux (ZSH):
  ```
  $ export CRABDRIVE_ADDR="127.0.0.1:1234"
  $ cargo run server
      Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
      Running `target/debug/crabdrive-server server`
  Server running on http://127.0.0.1:1234
  ^CExiting
  $ unset CRABDRIVE_ADDR
  ```

- On Windows (PowerShell):
  ```
  $ $env:CRABDRIVE_ADDR = "127.0.0.1:1234"
  $ cargo run server
    Compiling crabdrive-server v0.1.0
      Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.52s
      Running `target\debug\crabdrive-server.exe server`
  Server running on http://127.0.0.1:1234
  Exiting
  $ Remove-Variable [Environment]:CRABDRIVE_ADDR
  ```

Pressing `Crtl + C` will gracefully shut down the server.

## Client

To run the client on its own run `trunk server --open` from `/client`.
Make sure `trunk` is installed - if not install it with `cargo install trunk` - and add the `wasm32-unknown-unknown` target with `rustup target add wasm32-unknown-unknown`.

Optionally, you can install `leptosfmt` with `cargo install leptosfmt` to more easily format the view macros inside
leptos.