import threading
import grpc
import coldmod_msg.proto.tracing_pb2_grpc as tracing_pb2_grpc
import coldmod_msg.proto.tracing_pb2 as tracing_pb2
import queue
from typing import Iterable

Q: queue.Queue[tracing_pb2.Trace] = queue.Queue(maxsize=65536)
# could use a deque here instead, but reading from this doesn't block, we'd have to

def _stream_q() -> Iterable[tracing_pb2.Trace]:
    while True:
        yield Q.get()

def sender():
    with grpc.insecure_channel("127.0.0.1:7777") as channel:
        stub = tracing_pb2_grpc.TracesStub(channel)
        stub.collect(_stream_q())

def start():
    threading.Thread(target=sender, daemon=True).start()
