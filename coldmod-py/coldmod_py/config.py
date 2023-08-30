from typing import List, Dict, Tuple, Any
import os
import logging
import tomllib

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


def env():
    host = os.getenv("COLDMOD_GRPC_HOST")
    if not host:
        raise Exception("COLDMOD_GRPC_HOST not set")

    ca = os.getenv("COLDMOD_CA")
    if not ca:
        raise Exception("COLDMOD_CA not set")

    web_host = os.getenv("COLDMOD_WEB_HOST")
    if not web_host:
        raise Exception("COLDMOD_WEB_HOST not set")

    api_key = os.getenv("COLDMOD_API_KEY")
    if not api_key:
        raise Exception("COLDMOD_API_KEY not set")

    insecure = os.getenv("COLDMOD_INSECURE") == "on"

    return (host, ca, web_host, api_key, insecure)


(COLDMOD_GRPC_HOST, COLDMOD_CA, COLDMOD_WEB_HOST, COLDMOD_API_KEY, INSECURE) = env()
