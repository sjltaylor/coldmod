import os
import threading
from typing_extensions import TypeGuard
from coldmod_py.tracing.sender import Q
from coldmod_msg.proto.tracing_pb2 import TraceSrc, Trace
from typing import Dict

trace_srcs_by_location: Dict[str, TraceSrc] | None = None

def fn(frame, event, _):
    f_code = frame.f_code

    path = f_code.co_filename
    line = f_code.co_firstlineno
    key = f"{path}:{line}"

    if trace_srcs_by_location is None:
        # this should be set globally on initialization
        # if it isn't give up, raise an exception so the settrace function is unset
        raise Exception("lookup not initialized")

    trace_src = trace_srcs_by_location.get(key)

    if trace_src is None:
        return None

    thread_ident = threading.current_thread().ident
    assert(thread_ident is not None) # because the thread must have started

    trace = Trace(digest=trace_src.digest, thread_id=str(thread_ident),process_id=str(os.getpid()))

    # don't block - if the queue cannot drain (e.g. because the sender dies)
    # put_nowait raises a Full exception which will cause the trace function to be unset
    Q.put_nowait(trace)
