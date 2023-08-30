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
