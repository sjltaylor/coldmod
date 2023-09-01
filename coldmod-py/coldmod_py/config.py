from typing import List, Dict, Tuple, Any
import os
import sys
import logging
import tomllib
import pathlib

class Config:
    """
    absolute path to a config file
    """
    config_file: str
    srcs_root_dir: str
    ignore_patterns: List[str]

def load(path: str|None) -> Config:
    """
        Takes an optional path, assumes CWD if not specified.
        Can also take a directory, assumes a filename: coldmod.toml.
        A relative srcs.root value in config is resolved relative to the config file directory.
    """
    if path is None:
        path = os.getcwd()
        logging.debug(f"No path specificed.")

    logging.debug(f"Loading config from: {path}")

    if os.path.isdir(path):
        path = os.path.join(path, "coldmod.toml")
        logging.debug(f"Assuming config filename: {path}")

    path = os.path.abspath(path)

    with open(path, 'rb') as file:
        config = tomllib.load(file)

    srcs = config["srcs"] or {}
    root = srcs["root"] or "."
    ignore = srcs["ignore"] or []

    root = os.path.abspath(os.path.join(os.path.dirname(path), root))

    config = Config()

    config.config_file = path
    config.srcs_root_dir = root
    config.ignore_patterns = ignore

    return config

class RootMarker:
    ignore_patterns: List[str]

    def __init__(self, ignore_patterns: List[str]):
        self.ignore_patterns = ignore_patterns


def rootmarker() -> RootMarker:
    path = pathlib.Path(os.getcwd()).joinpath("coldmod.rootmarker")

    if not path.exists():
        sys.stderr.write("coldmod.rootmarker not found in working directory\n")
        sys.exit(-1)

    if not path.is_file():
        sys.stderr.write(f"{path} is not a file\n")
        sys.exit(-1)

    with path.open('rb') as file:
        config = tomllib.load(file)

    ignore = config["ignore"] or []

    return RootMarker(ignore)

class Env:
    def grpc_host(self) -> str:
        host = os.getenv("COLDMOD_GRPC_HOST")
        if not host:
            raise Exception("COLDMOD_GRPC_HOST not set")
        return host

    def ca(self) -> str:
        ca = os.getenv("COLDMOD_CA")
        if not ca:
            raise Exception("COLDMOD_CA not set")
        return ca

    def web_host(self) -> str:
        web_host = os.getenv("COLDMOD_WEB_HOST")
        if not web_host:
            raise Exception("COLDMOD_WEB_HOST not set")
        return web_host

    def api_key(self) -> str:
        api_key = os.getenv("COLDMOD_API_KEY")
        if not api_key:
            raise Exception("COLDMOD_API_KEY not set")
        return api_key

    def insecure(self) -> bool:
        insecure = os.getenv("COLDMOD_INSECURE") == "on"
        return insecure

env = Env()
