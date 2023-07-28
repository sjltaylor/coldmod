import threading
from coldmod_py.tracing.src.tracing_src import TracingSrc
import grpc
import coldmod_msg.proto.trace_pb2_grpc as trace_pb2_grpc
import coldmod_msg.proto.trace_pb2 as trace_pb2
import queue
from typing import Iterable, Tuple, Dict

Q = queue.Queue(maxsize=65536)

def _q_generator() -> Iterable[Tuple[TracingSrc, int, int]]:
    while True:
        yield Q.get()

def _stream_q_to_trace():
    # TODO: map TracingSrc to protobuf
    for (tracing_src, thread_id, process_id) in _q_generator():
        # TODO: tracing_src.digest
        # TODO: tracing_src.src
        # TODO: tracing_src.class_name_path
        trace = trace_pb2.Trace(path=tracing_src.path, line=tracing_src.lineno, thread_id=thread_id, process_id=process_id)
        yield trace

def sender():
    with grpc.insecure_channel("127.0.0.1:7777") as channel:
        stub = trace_pb2_grpc.TracingDaemonStub(channel)
        stub.collect(_stream_q_to_trace())

def start():
    threading.Thread(target=sender, daemon=True).start()
