import sys
import os
import threading
from . import sender
from .settrace import fn
from coldmod_py.files import find_srcs_in
import coldmod_py.config

def coldmod_tracing(path: str|None):
    config = coldmod_py.config.load(path)
    srcs = find_srcs_in(config.srcs_root_dir, config.ignore_patterns)
    # functions = ...
    # lookup = ...

def coldmod_tracing_root_marker():
    return _start(path=os.path.dirname(sys._getframe().f_back.f_code.co_filename)) #noqa

def _start(*, path: str):
    _install(path=path)
    sender.start()

def _install(*, path: str):
    # functions._root_marker_prefix = path
    threading.settrace(fn)
    sys.settrace(fn)

def _uninstall():
    if sys.gettrace() == fn:
        sys.settrace(None)
    if threading.gettrace() == fn:
        threading.settrace(None) # noqa
