# README

Coldmod-py is a tracing library for collecting runtime data from python code, and a CLI for configuration and codemods.

It emits events when each application function is called, enabling a heatmap of _functions_ to be computed.

It is develop in and for the Python version in `.python-version`, though it may work with other versions.

Tracing is implemented using  [sys.settrace](https://docs.python.org/3/library/sys.html?highlight=settrace#sys.settrace) - built into CPython.

The CLI, which is implemented with [Python Fire](https://github.com/google/python-fire), can be used to inspect configuration and apply code mods.

Parsing and modify code is implement using [LibCST](https://github.com/Instagram/LibCST).


## Installation

`pip install coldmod-py`

## Test

Test with `python -m pytest`.

## Setup

The CLI and tracing require these environment variables to be set

```
COLDMOD_GRPC_HOST
COLDMOD_TLS_CA
COLDMOD_WEB_HOST
COLDMOD_API_KEY
COLDMOD_INSECURE
```

See Env.md for details on how these environment variables are used.

## Rootmarker

A file marks the root of the application code being traced. This is used to determine which files to trace and to determine the fully qualified name of functions. The file also contains configuration allow files and functions to be ignored.

### Samples
```toml

[ignore]
files = [ ]
keys = [ ]

```

`keys` is the fully qualified name of a function. `files` is paths or glob patterns.


## Tracing

This library can trace function calls in application code (not deps).

It parses source code to find functions - `"trace_srcs"` and emits a runtime event for each function call. Each time the process starts, the source code is parsed and the set of tracing srcs is `_set_` on the server. Soem of the work on finding srcs is cached in `.coldmod-cache` for speed.

N.B. important to note is that `coldmod_py.tracing.start()` is potentially destructive - it can result in the heatmap on the server being truncated. For example if you ignore a file, all of the tracing srcs in the file will be removed from the heatmap when the process starts. This is how the Ignore functionality results in keys eventually being removed from the heatmap.

This also means you might want to wrap the coldmod tracing code in a conditional so that it only starts in production to avoid local source code changes affecting the heatmap (in the case that configuration pointed at production).

```
import coldmod_py
if os.environ.get("ENV") == "production":
    coldmod_py.tracing.start()
```

Function calls in each thread are traced and sent to the server via a dedicated sender thread. If connectivity to the server is lost, the sender appliest a backoff strategy and retries. A maximum of 65536 events are queued, additional events are dropped when the queue is full.

A configuration file is used to specify what to trace and denote the root of the source code being traced. See show to create a `coldmod.rootmarker` in the [Setup README](../README.md).

The tracing library can ignore `files` and `keys` (fully qualified function names). The effective configuration can be inspected using the CLI.


## CLI

Usage `python -m coldmod_py.cm <command>` in the same directory as `coldmod.rootmarker`.

* `cache warm` - warm the cache
* `cache clear` - clear the cache
* `files` - show the files to be parsed
* `srcs` - show keys and locations of trace srcs
* `rm` remove the function corresponding to the given key (fully qualified name)
* `ignore` - ignore the given key by adding it to the `coldmod.rootmarker` file
* `fetch` - retrieve the heatmap data from the server, print as text or json. Any keys on the server which are not found in the local source are skipped.
* `connect` pass a URL of an open web app instance, or omit it to open a new one. Once connected the CLI and provide the UI with metadata from local source code and enable the web app to apply code mods and ignore keys.

## Version Drift

Since the code being traced in production is likely to be an older version than the code being parsed locally, functions may have moved or been removed.
Since fully qualified function names are used as the key, parsing can tolerate changes to files, but not changes to package structure or function name.
