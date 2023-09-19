from typing import Dict, Iterable
from coldmod_py.code.parsed_trace_src import ParsedTraceSrc
from coldmod_py.files import find_src_files_in
import coldmod_py.config
from . import sender, settrace

from coldmod_msg.proto.tracing_pb2 import TraceSrc
import os
import logging

LOG=logging.getLogger(__name__)

def _trace_src_by_abs_loc(parsed_trace_srcs: Dict[str, Iterable[ParsedTraceSrc]]) -> Dict[str, TraceSrc]:
    return { f"{os.path.abspath(path)}:{parsed_trace_src.position.line}":parsed_trace_src.trace_src for path, parsed_trace_srcs in parsed_trace_srcs.items() for parsed_trace_src in parsed_trace_srcs }

def start():
    # try hard to set up for tracing, but dont stop the app from starting if we fail
    root_marker = coldmod_py.root_marker.load()
    absolute_paths = find_src_files_in(root_marker.dir(), root_marker.ignore_files())
    relative_paths = [os.path.relpath(p, root_marker.dir()) for p in absolute_paths]
    parsed_trace_srcs = coldmod_py.code.find_trace_srcs_by_relative_paths(relative_paths)
    srcs_by_location = _trace_src_by_abs_loc(parsed_trace_srcs)
    settrace.install(srcs_by_location)
    sender.start(srcs_by_location.values())
