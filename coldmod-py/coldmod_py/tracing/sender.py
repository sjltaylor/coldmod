import threading
import grpc
import coldmod_msg.proto.trace_pb2_grpc as trace_pb2_grpc
import coldmod_msg.proto.trace_pb2 as trace_pb2
import queue
from typing import Iterable, Tuple

Q = queue.Queue(maxsize=65536)

def _q_generator() -> Iterable[Tuple[str,int, int, int]]:
    while True:
        yield Q.get()

def _stream_q_to_trace():
    for [path, line, thread_id, process_id] in _q_generator():
        yield trace_pb2.Trace(path=path, line=line, thread_id=thread_id, process_id=process_id)

def sender():
    with grpc.insecure_channel("127.0.0.1:7777") as channel:
        stub = trace_pb2_grpc.TracingDaemonStub(channel)
        stub.collect(_stream_q_to_trace())

def start():
    threading.Thread(target=sender, daemon=True).start()
