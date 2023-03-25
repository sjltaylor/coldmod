import sys

# https://docs.python.org/3/library/sys.html?highlight=sys#sys.settrace
def trace_fn(frame, event, _):
    if event == "call":
        filename = frame.f_code.co_filename
        classname = frame.f_locals.get('self', None).__class__.__name__ if 'self' in frame.f_locals else None
        sys.stderr.write(f"{event} {frame.f_code.co_firstlineno} {frame.f_code.co_name} {classname} {filename}\n")

    # return None because we don't want to trace within the scope of a call
    # just want to trace when we enter new scopes (functions)


def init():
    sys.settrace(trace_fn)


