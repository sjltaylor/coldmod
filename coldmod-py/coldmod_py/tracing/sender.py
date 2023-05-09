import threading
import grpc
import coldmod_msg.proto.trace_pb2_grpc as trace_pb2_grpc
import coldmod_msg.proto.trace_pb2 as trace_pb2
from coldmod_py.tracing.queue import Q, generator

def _stream_q_to_trace():
    for [path, line] in generator():
        yield trace_pb2.Trace(path=path, line=line)

def sender():
    with grpc.insecure_channel("127.0.0.1:7777") as channel:
        stub = trace_pb2_grpc.TracingDaemonStub(channel)
        stub.collect(_stream_q_to_trace())

def start():
    threading.Thread(target=sender, daemon=True).start()
