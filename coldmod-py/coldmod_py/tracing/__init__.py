import sys
import os
import threading
from . import sender
from .settrace import fn
from coldmod_py.files import find_src_files_in
from coldmod_py.tracing.src import key_by_location, find_tracing_srcs_in
import coldmod_py.config

def start(path: str|None):
    config = coldmod_py.config.load(path)
    srcs = find_src_files_in(config.srcs_root_dir, config.ignore_patterns)
    tracing_srcs = key_by_location(find_tracing_srcs_in(config.srcs_root_dir, srcs))
    _install(tracing_srcs)
    sender.start()

def _install(tracing_srcs: Dict[str, sender.TracingSrc]):
    settrace.tracing_srcs = tracing_srcs
    threading.settrace(fn)
    sys.settrace(fn)

def _uninstall():
    if sys.gettrace() == fn:
        sys.settrace(None)
    if threading.gettrace() == fn:
        threading.settrace(None) # noqa
