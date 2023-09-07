from typing import Iterable, Dict, Tuple, Iterator, List

from . import parsed_trace_src
from .parsed_trace_src import ParsedTraceSrc
from .function_finder import FunctionFinder
from .parse import parse_modules
from hashlib import blake2b
import coldmod_py.cache as cache

from libcst.metadata import FullRepoManager, FullyQualifiedNameProvider

def find_trace_srcs(paths_relative_to_cwd: Iterable[str]) -> Dict[str, Iterable[parsed_trace_src.ParsedTraceSrc]]:
    frm = FullRepoManager(".", {*paths_relative_to_cwd}, {FullyQualifiedNameProvider})
    trace_srcs_by_relative_path = {}
    for rp in paths_relative_to_cwd:
        wrapper = frm.get_metadata_wrapper_for_path(rp)

        def _origin() -> List[ParsedTraceSrc]:
            function_finder = FunctionFinder()
            wrapper.visit(function_finder)
            return function_finder.trace_srcs

        trace_srcs = cache.parsed_trace_srcs(rp, _origin)
        trace_srcs_by_relative_path[rp] = trace_srcs

    return trace_srcs_by_relative_path
