import sys
import os
import threading
import coldmod_py.tracing.functions as tracing_functions
import coldmod_py.tracing.sender as sender
from coldmod_py.tracing.functions import coldmod_trace_fn as _tracing_fn

def coldmod_tracing_root_marker():
    return _start(path=os.path.dirname(sys._getframe().f_back.f_code.co_filename)) #noqa

def _start(*, path: str):
    _install(path=path)
    sender.start()

def _install(*, path: str):
    tracing_functions._root_marker_prefix = path
    threading.settrace(_tracing_fn)
    sys.settrace(_tracing_fn)

def _uninstall():
    if sys.gettrace() == _tracing_fn:
        sys.settrace(None)
    if threading.gettrace() == _tracing_fn:
        threading.settrace(None) # noqa