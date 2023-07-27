from typing import List, Dict, Tuple, Iterable, Any
from glob import glob
import os
import sys
import logging
import tomllib

def find_src_files_in(path: str, ignore: List[str] = []) -> Iterable[str]:
    exclude = set()

    for pattern in ignore:
        exclude = exclude.union(glob(os.path.join(path, pattern), recursive=True))

    candidates = set(glob(os.path.join(path, "**/*.py"), recursive=True))

    return candidates - exclude
