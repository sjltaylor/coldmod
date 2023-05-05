import sys
import threading
from socket import socket, AF_INET, SOCK_DGRAM
from coldmod_msg_py import Trace
import flatbuffers
import time

class _Installation:
    def __init__(self, *, host: str, port: int):
        self._socket = socket(AF_INET, SOCK_DGRAM)
        self._socket.connect((host, port))

    def trace_fn(self, frame, event, _):
        if event == "call":
            builder = flatbuffers.Builder(1024)
            p_filename = builder.CreateString(frame.f_code.co_filename)

            Trace.Start(builder)
            Trace.AddPath(builder, p_filename)
            Trace.AddLine(builder, frame.f_code.co_firstlineno)

            trace = Trace.End(builder)
            builder.Finish(trace)

            self._socket.sendall(builder.Output())

        # return None because we don't want to trace within the scope of a call
        # just want to trace when we enter new scopes (functions)

_installation: _Installation | None = None;

def install(*, host: str, port: int):
    _installation = _Installation(host=host, port=port)
    threading.settrace(_installation.trace_fn)
    sys.settrace(_installation.trace_fn)

def uninstall():
    sys.settrace(None)
    threading.settrace(None) # noqa
    assert _installation is not None
    _installation._socket.close()
