from typing import List, Dict, Tuple
from glob import glob
import os

def find_in(path: str) -> List[str]:
    # Find all Python files in path
    return glob(os.path.join(path, "**/*.py"), recursive=True)

def _read_file(path: str) -> Tuple[str, str]:
    with open(path, "r") as f:
        return (path, f.read())

def read_all(paths: List[str]) -> Dict[str, str]:
    return dict(map(_read_file, paths))
