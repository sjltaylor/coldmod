from coldmod_py.files import find_src_files_in
import coldmod_py.config
from . import sender, settrace
from .connect import register_trace_srcs
from coldmod_msg.proto.tracing_pb2 import TraceSrc
import os
import logging

LOG=logging.getLogger(__name__)

def start():
    # try hard to set up for tracing, but dont stop the app from starting if we fail
    try:
        root_marker = coldmod_py.config.root_marker()
        absolute_paths = find_src_files_in(root_marker.dir, root_marker.ignore_patterns)
        relative_paths = [os.path.relpath(p, root_marker.dir) for p in absolute_paths]
        parsed_trace_srcs = coldmod_py.code.find_trace_srcs(relative_paths)
        srcs_by_location = { f"{os.path.abspath(path)}:{parsed_trace_src.position.line}":parsed_trace_src.trace_src for path, parsed_trace_srcs in parsed_trace_srcs.items() for parsed_trace_src in parsed_trace_srcs }
        register_trace_srcs(srcs_by_location.values())
    except Exception as e:
        LOG.error(f"Failed to register trace srcs: {e}")
        return

    settrace.install(srcs_by_location)
    sender.start()
