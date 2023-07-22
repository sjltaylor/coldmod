from typing import Iterable
from coldmod_py.files import read_all
from .parse import _parse_all
from .visitor import _visit_all
from .tracing_src import TracingSrc

def scan(src_paths: Iterable[str]) -> Iterable[TracingSrc]:
    return _visit_all(_parse_all(read_all(src_paths)))
