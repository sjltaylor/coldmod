import sys
import threading
from socket import socket, AF_INET, SOCK_DGRAM
from coldmod_msg_py import Trace
import flatbuffers

_socket: socket|None = None

def install(*, host: str, port: int):
    udp_socket = socket(AF_INET, SOCK_DGRAM)
    _socket = udp_socket
    udp_socket.connect((host, port))

    def trace_fn(frame, event, _):
        if event == "call":
            builder = flatbuffers.Builder(1024)
            p_filename = builder.CreateString(frame.f_code.co_filename)
            Trace.Start(builder)
            Trace.AddPath(builder, p_filename)
            Trace.AddLine(builder, frame.f_code.co_firstlineno)
            Trace.AddCol(builder, 0)

            trace = Trace.End(builder)
            builder.Finish(trace)
            udp_socket.sendall(builder.Output())
        # return None because we don't want to trace within the scope of a call
        # just want to trace when we enter new scopes (functions)

    threading.settrace(trace_fn)
    sys.settrace(trace_fn)

def uninstall():
    global _socket
    sys.settrace(None)
    threading.settrace(None) # noqa
    if (_socket is not None):
        _socket.close()
        _socket = None
