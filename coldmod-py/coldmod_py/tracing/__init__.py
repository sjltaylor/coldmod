from coldmod_py.files import find_src_files_in
from coldmod_py.code import key_by_location, parse_trace_srcs_in
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
        parsed_trace_srcs = parse_trace_srcs_in(config.srcs_root_dir, srcs)
        srcs_by_location = key_by_location(config.srcs_root_dir, parsed_trace_srcs)
        register_trace_srcs(config.srcs_root_dir, [e.trace_src for e in parsed_trace_srcs])
    except Exception as e:
        LOG.error(f"Failed to register trace srcs: {e}")
        return

    srcs_by_location = {k: v.trace_src for k,v in srcs_by_location.items()}

    settrace.install(srcs_by_location)
    sender.start()
