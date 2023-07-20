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
        Can also take a directory, assumes a filename: Coldmod.toml.
        A relative srcs.root value in config is resolved relative to the config file directory.
    """
    if path is None:
        path = os.getcwd()
        logging.debug(f"No path specificed.")

    logging.debug(f"Loading config from: {path}")

    if os.path.isdir(path):
        path = os.path.join(path, "Coldmod.toml")
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
