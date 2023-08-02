import sys
import os
import threading
from coldmod_py.files import find_src_files_in
from coldmod_py.code import key_by_location, find_trace_srcs_in
import coldmod_py.config
from typing import Dict
from . import sender
from .connect import register_trace_srcs
from .settrace import fn
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

    _install(trace_srcs_by_location)
    sender.start()

def _install(trace_srcs_by_location: Dict[str, TraceSrc]):
    settrace.trace_srcs_by_location = trace_srcs_by_location
    threading.settrace(fn)
    sys.settrace(fn)

def _uninstall():
    if sys.gettrace() == fn:
        sys.settrace(None)
    if threading.gettrace() == fn:
        threading.settrace(None) # type: ignore
