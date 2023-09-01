from typing import List, Dict, Tuple, Iterator

from . import parsed_trace_src
from .parsed_trace_src import ParsedTraceSrc
from .function_finder import FunctionFinder

from libcst.metadata import FullRepoManager, FullyQualifiedNameProvider

def trace_srcs(paths_relative_to_cwd: List[str]) -> Dict[str, List[parsed_trace_src.ParsedTraceSrc]]:
    frm = FullRepoManager(".", {*paths_relative_to_cwd}, {FullyQualifiedNameProvider})
    trace_srcs_by_relative_path = {}
    for rp in paths_relative_to_cwd:
        wrapper = frm.get_metadata_wrapper_for_path(rp)
        function_finder = FunctionFinder()
        wrapper.visit(function_finder)
        trace_srcs_by_relative_path[rp] = function_finder.trace_srcs

    return trace_srcs_by_relative_path
