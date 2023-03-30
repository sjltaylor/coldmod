import os
import sys
import threading

# https://docs.python.org/3/library/sys.html?highlight=sys#sys.settrace
def trace_fn(frame, event, _):
    if event == "call":
        filename = frame.f_code.co_filename
        classname = frame.f_locals.get('self', None).__class__.__name__ if 'self' in frame.f_locals else None
        sys.stderr.write(f"{event} {frame.f_code.co_firstlineno} {frame.f_code.co_name} {classname} {filename}\n")

    # return None because we don't want to trace within the scope of a call
    # just want to trace when we enter new scopes (functions)

class _TraceAgent():
    def __init__(self, trace_root=''):
        self.trace_root = trace_root

    def trace_fn(self, frame, event, _):
        if event == "call":
            filename = frame.f_code.co_filename
            if filename.startswith(self.trace_root):
                classname = frame.f_locals.get('self', None).__class__.__name__ if 'self' in frame.f_locals else None
                sys.stderr.write(f"{event} {frame.f_code.co_firstlineno} {frame.f_code.co_name} {classname} {filename}\n")

        # return None because we don't want to trace within the scope of a call
        # just want to trace when we enter new scopes (functions)

    def settrace(self):
        threading.settrace(self.trace_fn)
        sys.settrace(self.trace_fn)

def init_from_trace_root():
    return init(os.path.dirname(sys._getframe().f_back.f_code.co_filename))

def init(trace_root=''):
    _TraceAgent(trace_root=trace_root).settrace()


