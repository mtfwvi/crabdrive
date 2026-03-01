# Test strategy
We did not find any tooling to do integration tests with leptos. Instead we chose the following test strategies:

## Manual testing
Since a big part of our project was about communication between server and client, a big part of the functionality can be tested by starting the client and testing the functions manually. We created the checklist in [docs/submission/manual_testing.md](./manual_testing.md) .

## Unit tests
#### Rust tests 
Wherever possible, we wrote unit tests to test small parts of our code.

#### Wasm tests
Some tests in the client required a browser environment (especially integration with browser apis). We used `wasm_bindgen_test` to execute those tests in the browser

## Server tests

### Request Handler

The request handlers in the server were tested using `axum-test` in `server/src/test/`. The tests make requests against a mock server, and we compare the expected result manually. The payload for requests is (as far as possible) randomly generated.

### Database Functions

Because Diesel aims to provide strong compile-time safety, we test database operations primarily through integration via the request handlers.

### Virtual File System

Dedicated storage backend tests for the VFS are maintained in `server/src/test/storage/`.
