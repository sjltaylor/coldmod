from typing import Iterable, Dict
from coldmod_py.files import read_all
from .parse import _parse_all
from .visitor import _visit_all
from coldmod_msg.proto.tracing_pb2 import TraceSrc

def find_trace_srcs_in(srcs_root_dir: str, src_paths: Iterable[str]) -> Iterable[TraceSrc]:
    return _visit_all(srcs_root_dir, _parse_all(read_all(src_paths)))

def key_by_location(trace_srcs: Iterable[TraceSrc]) -> Dict[str,TraceSrc]:
    return {f"{ts.path}:{ts.lineno}" : ts for ts in trace_srcs}

def key_by_digest(trace_srcs: Iterable[TraceSrc]) -> Dict[str,TraceSrc]:
    return {ts.digest : ts for ts in trace_srcs}