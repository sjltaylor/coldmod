import threading
import grpc
import coldmod_msg.proto.tracing_pb2_grpc as tracing_pb2_grpc
import coldmod_msg.proto.tracing_pb2 as tracing_pb2
import queue
from typing import Iterable
import coldmod_py.coldmod_d as coldmod_d

Q: queue.Queue[tracing_pb2.Trace] = queue.Queue(maxsize=65536)
# could use a deque here instead, but reading from this doesn't block, we'd have to

def _stream_q() -> Iterable[tracing_pb2.Trace]:
    while True:
        yield Q.get()

def sender():
    with coldmod_d.grpc_channel() as channel:
        stub = tracing_pb2_grpc.TracesStub(channel)
        if coldmod_d.config.env.insecure():
            stub.collect(_stream_q())
        else:
            metadata = [("authorization", f"Bearer {coldmod_d.config.env.api_key()}")]
            stub.collect.with_call(_stream_q(), metadata=metadata)

def start():
    threading.Thread(target=sender, daemon=True).start()
