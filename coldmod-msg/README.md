# README

This directory contains protocol buffer and gRPC definitions in `proto`, as well as some shared client-server Rust types in `rs/src/web`.

You will need to install the `protoc` compiler to generate code from the definitions.

To generate (or update) Python code from protocol buffer definitions, run `py/gen.sh` from the `py` directory.

Rust code uses `tonic_build` to generate code from protocol buffer definitions. See `rs/build.rs` for how this is configured.
