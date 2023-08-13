from typing import Iterable, Dict, List
from coldmod_py.files import read_all
from .parse import parse_modules
from .visitor import _visit_all
import coldmod_py.code.digest
from .parsed_trace_src import ParsedTraceSrc
import os

def parse_trace_srcs_in(srcs_root_dir: str, src_paths: Iterable[str]) -> List[ParsedTraceSrc]:
    return list(_visit_all(srcs_root_dir, parse_modules(read_all(src_paths))))

def key_by_location(srcs_root_dir: str, trace_srcs: Iterable[ParsedTraceSrc]) -> Dict[str,ParsedTraceSrc]:
    return {f"{os.path.join(srcs_root_dir, ts.trace_src.path)}:{ts.trace_src.lineno}" : ts for ts in trace_srcs}

def key_by_digest(trace_srcs: Iterable[ParsedTraceSrc]) -> Dict[str,ParsedTraceSrc]:
    return {pts.trace_src.digest : pts for pts in trace_srcs}

def duplicates(parsed_trace_srcs: Iterable[ParsedTraceSrc]) -> Dict[str,List[ParsedTraceSrc]]:
    by_digest = {}

    for parsed_trace_src in parsed_trace_srcs:
        digest = parsed_trace_src.trace_src.digest
        if digest not in by_digest:
            by_digest[digest] = []
        by_digest[digest].append(parsed_trace_src)

    return {digest: by_digest[digest] for digest, ts in by_digest.items() if len(ts) > 1}
