from coldmod_py.tracing.queue import Q

_path_prefix_filter: str = ''

def coldmod_trace_fn(frame, event, _):
    path = frame.f_code.co_filename
    if path.startswith(_path_prefix_filter):
        if event == "call":
            line = frame.f_code.co_firstlineno
            Q.put_nowait([path, line])

    # return None because we don't want to trace within the scope of a call
    # just want to trace when we enter new scopes (functions)
