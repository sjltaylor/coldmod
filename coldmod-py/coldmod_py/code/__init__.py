from typing import Iterable, Dict
from coldmod_py.files import read_all
from .parse import _parse_all
from .visitor import _visit_all
from .tracing_src import TracingSrc

def find_heat_srcs_in(srcs_root_dir: str, src_paths: Iterable[str]) -> Iterable[TracingSrc]:
    return _visit_all(srcs_root_dir, _parse_all(read_all(src_paths)))

def key_by_location(tracing_srcs: Iterable[TracingSrc]) -> Dict[str,TracingSrc]:
    return {f"{ts.path}:{ts.lineno}" : ts for ts in tracing_srcs}

def key_by_digest(tracing_srcs: Iterable[TracingSrc]) -> Dict[str,TracingSrc]:
    return {ts.digest : ts for ts in tracing_srcs}
