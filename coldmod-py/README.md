# README

Coldmod-py is a python tracing library and CLI which.
Its for trace configuration and code modding.

This is a quick and diiirrrty CLI built using google Fire.
Minimimal input validation is done.
For example, we make use of assert to cause runtime exceptions.




* Get into the poetry env: `poetry shell`
* Run a sample: `python -m coldmod_py.samples.trace_target_1`
* Test command: `pytest[-watch]`
* Run the cli: `python -m coldmod <tracefile> <sourcepath>`
* to debug just add `breakpoint()`

## Development

### Editable install


## CLI

The CLI is used to apply code mods and check trace configuration.

## Installation

### Man page:
```
insert generated man page here
```

## pyright

run it with brew install, it uses node... so I don't want to make every install it as a dep


## setting up in an app via editable install of local checkout

* checkout the __coldmod branch (has a pyproject.toml which has coldmod-py as an editable install)
* activate an env for the app (see __coldmod branch)
* in that shell, go to the root of the app
* poetry install
* pip install -r requirements.txt
* add a line to the entrypoint `import coldmod_py; coldmod_py.tracing.start()`
* add a coldmod.toml
* start the app
* if coldmod-d isn't running the app should carry on and start but with no tracing
* start coldmod-d
* restart the app
