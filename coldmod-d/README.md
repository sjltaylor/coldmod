# README

This is the Coldmod server component. It provides:
* Storage via redis 7.2
* A gRPC interface for
    * runtime data collection
    * CLI <-> Web UI interop
* A websocket server for the web UI

This package contains the following binary crates:
* `coldmod-d` gRPC, websockets and static file server
* `demo` a cli enabling simulation and diagnostics of data collection

Top level instructions to get this component running at [here](../README.md).

## Run

* `cargo run --bin coldmod-d` to run the server (from the root)
* `cargo run --bin demo` to run the demo cli (from the root)

Each binary crate is a self-documenting cli developed with [argh](https://github.com/google/argh)


## Tests

Run with `cargo test`.

The integration tests are destructive. To avoid data loss they are `#[ignore]`ed. To run them, uncomment the `#[ignore]` and set `COLDMOD_OPS=on`.

`COLDMOD_OPS` enables some gRPC operations which can be used by the test suite to manage test data.
