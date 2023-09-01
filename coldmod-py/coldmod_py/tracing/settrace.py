import os
import sys
import threading
from typing_extensions import TypeGuard
from coldmod_py.tracing.sender import Q
from coldmod_msg.proto.tracing_pb2 import TraceSrc, Trace
from typing import Dict

_trace_srcs_by_location: Dict[str, TraceSrc] | None = None

def _fn(frame, event, _):
    f_code = frame.f_code

    path = f_code.co_filename
    line = f_code.co_firstlineno
    key = f"{path}:{line}"

    if _trace_srcs_by_location is None:
        return

    assert _trace_srcs_by_location is not None
    trace_src = _trace_srcs_by_location.get(key)

    if trace_src is None:
        return None

    thread_ident = threading.current_thread().ident
    assert(thread_ident is not None) # because the thread must have started

    trace = Trace(key=trace_src.key, thread_id=str(thread_ident),process_id=str(os.getpid()))

    # don't block - if the queue cannot drain (e.g. because the sender dies)
    # put_nowait raises a Full exception which will cause the trace function to be unset
    Q.put_nowait(trace)




def install(trace_srcs_by_location: Dict[str, TraceSrc]):
    global _trace_srcs_by_location
    _trace_srcs_by_location = trace_srcs_by_location
    threading.settrace(_fn)
    sys.settrace(_fn)


def uninstall():
    global _trace_srcs_by_location
    _trace_srcs_by_location = None
    if sys.gettrace() == _fn:
        sys.settrace(None)
    if threading.gettrace() == _fn:
        threading.settrace(None) # type: ignore
