import os
import threading
from coldmod_py.tracing.queue import Q

_root_marker_prefix: str = ''

def coldmod_trace_fn(frame, event, _):
    path = frame.f_code.co_filename
     # path can be None, check the event refers to something in the marked root
    if path and path.startswith(_root_marker_prefix):
        path = path[len(_root_marker_prefix)+1:]
        if event == "call":
            line = frame.f_code.co_firstlineno
            # don't block - if the queue cannot drain (e.g. because the sender dies)
            # put_nowait raises an exception which will cause the trace function to be unset
            Q.put_nowait([
                path,
                line,
                threading.current_thread().ident,
                os.getpid()
            ])

    # return None because we don't want to trace within the scope of a call
    # just want to trace when we enter new scopes (functions)
