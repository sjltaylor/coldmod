# README

This directory contains `flatbuffers` which define protocols between coldmod components.

The `rs` and `py` directorys are intended to contain libraries imports for Rust and Python respectively.
The source client implementations are generated:

from the root:

* `flatc --rust -o coldmod-msg/rs/src coldmod-msg/trace.fbs`
* `flatc --python -o coldmod-msg/py/src coldmod-msg/trace.fbs`
