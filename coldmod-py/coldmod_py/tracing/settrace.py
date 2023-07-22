import os
import threading
from coldmod_py.tracing.sender import Q

_root_marker_prefix: str = ''

def fn(frame, event, _):
    f_code = frame.f_code
    path = f_code.co_filename
    # path can be None, check the event refers to something in the marked root
    # also, we only care about function calls (co_name can be a class name <dictcomp>, <listcomp>, etc.)
    if path and path.startswith(_root_marker_prefix) and event == "call":
        path = path[len(_root_marker_prefix)+1:]
        line = f_code.co_firstlineno
        payload = [
            path,
            line,
            threading.current_thread().ident,
            os.getpid()
        ]
        # don't block - if the queue cannot drain (e.g. because the sender dies)
        # put_nowait raises a Full exception which will cause the trace function to be unset
        Q.put_nowait(payload)

    # return None because we don't want to trace within the scope of a call
    # just want to trace when we enter new scopes (functions)
