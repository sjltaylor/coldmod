# README

TODO

* document the main cli
* document the demo cli
* document the COLDMOD_OPS environment variable ("on" to turn on)
* document generating tls certs (rsa 2048 because rustls)
* document how using mkcert for local dev

ENV VARS


COLDMOD_OPS
COLDMOD_TLS_CA (demo client)
COLDMOD_WEB_HOST
COLDMOD_GRPC_HOST
COLDMOD_REDIS_HOST
COLDMOD_API_KEY
COLDMOD_TLS_CERT
COLDMOD_TLS_KEY
COLDMOD_INSECURE (local basically turns off auth and tls, becuase trunk RS doesnt do TLS and browser don't allow you to mix )


## TLS

The `leptos` web app is run during development with `trunk.rs`, which doesn't support TLS - so configuration supports setups with and without TLS.

To run the setup with TLS you will need certs. For local dev with TLS enabled, you might want to use https://github.com/FiloSottile/mkcert to create certs.

With TLS enabled, obviously you won't be able to use `trunk.rs`. So, build the apps and host it in `coldmod-d/dist`. `trunk build` in `coldmod-web` creates a `dist` directory in `coldmod-web`. `coldmod-d` runs a static file server that looks in `dist/index.html` from wherever it is run, so you can move the `coldmod-web` `dist` to `coldmod-d` to serve the app via TLS.
