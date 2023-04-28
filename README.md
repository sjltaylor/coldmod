# README

Notes about the monorepo setup

## Python

* single virtualenv created with poetry in the root
  * individual projects have a `poetry.toml` with `virtualenvs.create = false`
* editable installs done with `poetry add -e <path>` in the root
* dont forget to activate the shell (`poetry shell`)
* not sure how building and distributing packages will work yet
* component deps go in their own pyproject.toml, dev deps go in the root pyproject.toml

## Rust

* Root level Cargo workspace
* flat namesspaces for libraries, e.g `coldmod_msg_rs` not `coldmod::msg_rs`