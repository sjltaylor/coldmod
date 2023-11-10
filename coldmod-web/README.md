# README

This is the Coldmod web app, built with [Leptos](https://github.com/leptos-rs/leptos/).

See instructions in the root [README](../README.md).

It provides a realtime view of tracing data collection with the ability to filter the heatmap for Hot/Cold keys, and supports a "connected" mode where the UI can be used to issue mod commands to a local development environment.


## Setup

Install [Trunk](https://trunkrs.dev/)

For distribution or TLS setups, use `trunk build [--release]`. This will create a `dist` (configured in `Trunk.toml`). Note that build output depends on configuration, specifically `COLDMOD_INSECURE`. Therefore the app will need to be built with the configuration for the target environment.


Run tests with: `cargo test`.


## TLS

The `leptos` web app is run during development with `trunk.rs`, which doesn't support TLS - so configuration supports setups with and without TLS.

To run the setup with TLS you will need certs. For local dev with TLS enabled, you might want to use https://github.com/FiloSottile/mkcert to create certs.

With TLS enabled, obviously you won't be able to use `trunk.rs`. So, build the apps and host it in `coldmod-d/dist`. `trunk build` in `coldmod-web` creates a `dist` directory in `coldmod-web`. `coldmod-d` runs a static file server that looks in `dist/index.html` from wherever it is run, so you can move the `coldmod-web` `dist` to `coldmod-d` to serve the app via TLS.
