from coldmod_py.files import find_src_files_in
from coldmod_py.code import key_by_location, find_trace_srcs_in
import coldmod_py.config
from . import sender, settrace
from .connect import register_trace_srcs
from coldmod_msg.proto.tracing_pb2 import TraceSrc
import logging

LOG=logging.getLogger(__name__)

def start(path: str|None = None):
    # try hard to set up for tracing, but dont stop the app from starting if we fail
    try:
        config = coldmod_py.config.load(path)
        srcs = find_src_files_in(config.srcs_root_dir, config.ignore_patterns)
        trace_srcs = find_trace_srcs_in(config.srcs_root_dir, srcs)
        trace_srcs_by_location = key_by_location(config.srcs_root_dir, trace_srcs)
        register_trace_srcs(config.srcs_root_dir, trace_srcs)
    except Exception as e:
        LOG.error(f"Failed to register trace srcs: {e}")
        return

    settrace.install(trace_srcs_by_location)
    sender.start()
