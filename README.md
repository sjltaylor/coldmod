# README

Coldmod is a set of tools for finding and removing "cold" code.

N.B. Where "dead" code can be found through static analysis, "cold" code is code that is never executed at runtime even if its callpath is reachable.

## Components

### `coldmod-d`

This server component provides gRPC, websocket and static file services. It stores tracing data, serves a realtime UI via websockets and can server the web app bundle.

### `coldmod-web`

This is the [leptos](https://leptos.dev/) web UI. It provides realtime visibility of data being collected and, when the CLI is connected, allows codemods to be applied.

### `coldmod-py`

This is a the Python 3 library that provides support for measuring python apps at runtime. It also provides a CLI, which can be connect to the web UI to enable the web UI to apply codemods.


### `coldmod-msg`

This is a shared library of types, protocol buffers and gRPC definitions.

## Prerequisites

Setup assumes local development on Mac OS.
Ubuntu has been used for production environments.

Coldmod uses:
* Redis 7.2: `brew install redis`
* protobuf: `brew install protobuf`

## Setup

[Trunk](https://trunkrs.dev/), which is used for web UI development, doesn't support TLS. This is a shame because otherwise configuration would only need to support TLS. We need to support both TLS and non-TLS setups. This section details how to setup for development (without TLS). Additional setup required for TLS is in the next section.

We need to export some env vars. This _can_ be done manually; I use [direnv](https://direnv.net/) for convenience.

For non-TLS setup export `COLDMOD_INSECURE=on`. This also disables API authentication (see below). Any other value than `"on"` will enable TLS and authentication.

We need to setup up three server enpdoints: gRPC, websockets and the webapp. Also, we need to configure the redis server host.

Export `COLDMOD_REDIS_HOST=` to the redis server host, typically `redis://localhost/`.
Export `COLDMOD_WEB_HOST=` to the web server host, I use `localhost:8088`.
Export `COLDMOD_GRPC_HOST=` to the gRPC server host, I use `localhost:8089`.

Note: no protocol specification for `COLDMOD_WEB_HOST`/`COLDMOD_GRPC_HOST`.

The web app can be built and served from `coldmod-d`, but during development I use [Trunk](https://trunkrs.dev/). Configuration in `coldmod-web/Trunk.toml`  determines the value needed to export `COLDMOD_APP_ORIGIN`, if unchanged this is `http://localhost:8080`.

Having exported those environment variables, we can start the server and web UI.
* From the repo root run `cargo build`
* From the repo root run: `cargo run --bin coldmod-d`
* From `coldmod-web` run `trunk serve --open`

The web UI should open in your browser and you should successfully load an empty page (stay with me, it gets more exciting).

At this point you _could_ see the UI working by simulating a client using some CLI commands provided by other crates in `coldmod-d`. See the [README](coldmod-d/README) in `coldmod-d` for how to do that. Here, we'll continue setting up to trace and modify a Python app.

Next we can setup up a Django application with `coldmod-py` for tracing. There are two options: install from pypi to use the published version (add it to your `requirements.txt` and `pip install` , or use an editable install.

After adding either:
* pypi: `coldmod-py[==<version>]`
* editable install: `-e path/to/coldmod/coldmod-py`

Run `pip install -r requirements.txt`.
A virtual env is recommended. I use [`pyenv`](https://github.com/pyenv/pyenv) and [`virtualenv`](https://virtualenv.pypa.io/en/latest/).

In the `manage.py` of the Django app, add the following:

```python
import coldmod_py

if sys.argv[1] == 'runserver' and os.environ.get("ENV") == "production":
    coldmod_py.tracing.start()
```
See [coldmod-py/README.md](coldmod-py/README.md) for more on what happens when tracing starts.


This will start tracing when the Django app is run with `manage.py runserver`.

In the same directory as `manage.py` create a file called `coldmod.rootmarker`.
This file denotes the root of the tracing namespace and provides configuration. Here as an example configuration, modify as required:

```toml

[ignore]
files = [ "**/migrations/**", "**/tests/**", "**/wsgi.py", "**/test_*.py", "**/tests.py",]
keys = [ ]

```

For more about `keys` see (more on this later).

Now we will begin parsing source code to look for functions to trace at runtime. Note the first time parsing happens it can take a while, but will cache the data it collects in `.coldmod-cache` in the same directory as `coldmod.rootmarker` to speed up subsequent runs. If startup time is a concern, the cache can be warmed at build time with a CLI (see `coldmod-py` README for CLI usage).

Before we run the Django application and start tracing, we can sense check the `ignore` configuration to see what files and functions will be included:
* `python -m coldmod_py.cm files` to list the files that will be included
* `python -m coldmod_py.cm srcs` to see all fully qualify names of functions which will be traced. Note this starts parsing the source code.

If that's all okay, start your Django app and create some traffic. The Web UI you started earlier should be populated with a Heatmap: all of the functions which _might_ be executed and a count of how many times each function has been called.

Idenitfy "cold" functions by selecting the "Cold" filter.

Next, back at the command line run `python -m coldmod_py.cm connect [URL]` where `URL` is the URL of the existing Web UI. If you omit it, it will launch a new Web UI.

The Web UI is now in "connected" mode and new controls are available. Don't forget to commit to version control before making any changes.
For each function, the functions can be ignored if you don't want to trace it, or removed from source because it's cold code.

When a function is ignored, the key is added to the `ignore.keys` in `coldmod.rootmarker`. Next time to Django application starts the heatmap will be truncated, until then it will show in the heatmap as greyed when the CLI is connected.

When a function is removed from source, the function itself is deleted and any references are left with a comment. The key will continue to appear in the UI until a version of the application with the funtion rmeoved is started. Until then, it can be Ignored.

When the CLI is connected, any keys in the UI which are not present in the source will not show the Remove option.

For more details on the available CLI commands see `coldmod-py/README.md`.
For more details on server capabilities see `coldmod-d/README.md`.


## TLS and Authentication

TLS support is available. For development and testing with TLS enabled, you'll need to generate some certs. I used [mkcert](https://github.com/FiloSottile/mkcert)

For TLS support, export `COLDMOD_INSECURE=off` (or omit the env var, or set it to anything other than `"on"`.

You will need to specify values for these environment variables:
* `COLDMOD_TLS_CA=...`
* `COLDMOD_TLS_CERT=...`
* `COLDMOD_TLS_KEY=...`

Also, when `COLDMOD_INSECURE!=on` authentication is enabled:
* `coldmod-web`-> `coldmod-d` by HTTP Basic Auth (username: `coldmod`)
* `coldmod-py` (CLI and tracing) -> `coldmod-d` via API key

You'll need to export an API key (also the basic auth password): `COLDMOD_API_KEY=...`.

Also, the protocols on other host environment variables will be need to be upgraded to `https://`:

Since you won't be able to serve the application with TLS using Trunk it will need hosting somewhere else. Also, the build output depends on env vars. See `coldmod-web/README.md` for more details.

See [About](./About.md) for more about goals, technology choices and limitations.


## Help and Contributing

There's plenty lots to improved and explore with this tooling.

If you'd like help using it, or contributing, please get in touch via GitHub issues or the contact information available on Github.

Please note the [code of conduct](CODE_OF_CONDUCT.md), please follow it in all your interactions.
