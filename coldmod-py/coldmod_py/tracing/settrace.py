import os
import threading
from coldmod_py.tracing.sender import Q
from coldmod_py.tracing.src.tracing_src import TracingSrc
from typing import Dict

tracing_srcs: Dict[str, TracingSrc] | None = None

def fn(frame, event, _):
    f_code = frame.f_code

    path = f_code.co_filename
    line = f_code.co_firstlineno
    key = f"{path}:{line}"

    if tracing_srcs is None:
        # this should be set globally on initialization
        # if it isn't give up, raise an exception so the settrace function is unset
        raise Exception("source look not initialized")

    tracing_src = tracing_srcs.get(path)

    if tracing_src is None:
        return None

    payload = (
        tracing_src,
        threading.current_thread().ident,
        os.getpid()
    )

    # don't block - if the queue cannot drain (e.g. because the sender dies)
    # put_nowait raises a Full exception which will cause the trace function to be unset
    Q.put_nowait(payload)
