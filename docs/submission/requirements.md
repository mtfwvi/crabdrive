# Requirements
### (G6): here are at least two server side quality gates for pull requests
The following checks are run on every pull request
- clippy (called `Check build`)
- Rust tests (called `Run tests`)
- build client (called `Check Client build`)
  - this check is required because there was case where clippy passed but the client did not compile because of a library (`diesel` imported by mistake) that was not available for the wasm architecture
- Wasm tests (called `Run WASM tests`)
  - tests in the client that require a browser environment are run in the browser using `wasm_bindgen_test`
- format check (called `Check code formatting`)
  - `cargo fmt`
  - `leptosfmt` (checks the formatting of code inside the `view!` macro)

### (C2): The project is executable
project can be built/run using 

1. running it directly:

   Prerequisites:
    - Rust 1.85.0 or later installed.
    - trunk installed (install with `cargo install trunk`, requires a newer rust version than 1.85)
    - available `wasm32-unknown-unknown` target (add with `rustup target add wasm32-unknown-unknown`)
    - TCP port `2722` (or port set by `CRABDRIVE_ADDR`) available on the host.
   
    Instructions:
   - `cd client && trunk build --release && cd .. && cargo run --bin crabdrive-server --release`

    
2. container: `podman compose up --build` / `docker compose up --build`
3. (not recommended) dev container: the container should be usable in vs code. It does not work on windows 50% of the time and may require multiple attempts to run it

    The steps to run the server in the container should be the same as for running it directly

### (C8) he code is tested with a sensible testing method
see [docs/submission/testing.md](./testing.md) 

### (D1) The documentation contains all steps to run the project
see C2 for running it.

After compiling the client and running the server, the frontend should be available on `http://localhost:2722`. To use client one needs to register an account. The default invite code is `crabdrive`. After registering you can log in.

### (D2) The documentation lists all implemented features and how to discover them if non obvious
see [docs/features.md](../features.md) 

### (D4) IFI Rule: State all used AI/LLMs and for what purpose they were used
see [docs/submission/ai-usage](./ai-usage.md)

### (D6) The documentation describes how the testing was done and explains why the testing method is well founded.
see [docs/submission/testing.md](./testing.md) 
