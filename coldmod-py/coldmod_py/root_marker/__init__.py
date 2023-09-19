from typing import Dict, Any, List, Self
import os
import sys
import logging
import tomli
import tomli_w
from pathlib import Path

class RootMarker:
    _config: Dict[str, Any]
    _path: Path

    def __init__(self, *, path: Path, config: Dict[str, Any]):
        self._config = config
        self._path = path

    def dir(self) -> str:
        return str(self._path.parent)

    def ignore(self) -> Dict[str, List[str]]:
        return self._config.get("ignore") or {}

    def ignore_files(self) -> List[str]:
        return self.ignore().get("files") or []

    def ignore_keys(self) -> List[str]:
        return self.ignore().get("keys") or []

    def add_ignore_key(self, key: str) -> Self:
        if "ignore" not in self._config:
            self._config["ignore"] = {}

        if "keys" not in self._config["ignore"]:
            self._config["ignore"]["keys"] = []

        self._config["ignore"]["keys"].append(key)
        self._config["ignore"]["keys"] = list(set(self._config["ignore"]["keys"]))

        return self

    def dump(self):
        with self._path.open('wb') as file:
            tomli_w.dump(self._config, file)



def load() -> RootMarker:
    path = Path().cwd().joinpath("coldmod.rootmarker")
    if not path.exists():
        sys.stderr.write("coldmod.rootmarker not found in working directory\n")
        sys.exit(-1)

    if not path.is_file():
        sys.stderr.write(f"{path} is not a file\n")
        sys.exit(-1)

    with path.open('rb') as file:
        config = tomli.load(file)

    return RootMarker(path=path, config=config)
