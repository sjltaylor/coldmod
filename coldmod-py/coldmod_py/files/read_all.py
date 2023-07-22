from typing import Iterable, Dict, Tuple

def _read_file(path: str) -> Tuple[str, str]:
    with open(path, "r") as f:
        return (path, f.read())

def read_all(paths: Iterable[str]) -> Dict[str, str]:
    return dict(map(_read_file, paths))
