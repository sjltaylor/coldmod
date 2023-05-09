import sys
import os
import threading
import coldmod_py.tracing.functions as tracing_functions
import coldmod_py.tracing.sender as sender
from coldmod_py.tracing.functions import coldmod_trace_fn as _tracing_fn

def start_in_this_dir():
    return start(path=os.path.dirname(sys._getframe().f_back.f_code.co_filename)) #noqa

def start(*, path: str):
    install(path=path)
    sender.start()

def install(*, path: str):
    tracing_functions._path_prefix_filter = path
    threading.settrace(_tracing_fn)
    sys.settrace(_tracing_fn)

def uninstall():
    if sys.gettrace() == _tracing_fn:
        sys.settrace(None)
    if threading.gettrace() == _tracing_fn:
        threading.settrace(None) # noqa
