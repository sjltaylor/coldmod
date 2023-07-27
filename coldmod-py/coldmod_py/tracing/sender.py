import threading
from coldmod_py.tracing.src.tracing_src import TracingSrc
import grpc
import coldmod_msg.proto.trace_pb2_grpc as trace_pb2_grpc
import coldmod_msg.proto.trace_pb2 as trace_pb2
import queue
from typing import Iterable, Tuple, Dict

Q = queue.Queue(maxsize=65536)

def _q_generator() -> Iterable[Tuple[str,int, int, int]]:
    while True:
        yield Q.get()

def _stream_q_to_trace(tracing_srcs: Dict[str, TracingSrc]):
    for [path, line, thread_id, process_id] in _q_generator():
        # TODO: lookup tracing_srcs by path:line
        print("TRACING SRC:", vars(tracing_srcs[f"{path}:{line}"]))
        yield trace_pb2.Trace(path=path, line=line, thread_id=thread_id, process_id=process_id)

def sender(tracing_srcs: Dict[str, TracingSrc]):
    with grpc.insecure_channel("127.0.0.1:7777") as channel:
        stub = trace_pb2_grpc.TracingDaemonStub(channel)
        stub.collect(_stream_q_to_trace(tracing_srcs))

def start(tracing_srcs: Dict[str, TracingSrc]):
    def start_with_tracing_srcs():
        sender(tracing_srcs)
    threading.Thread(target=start_with_tracing_srcs, daemon=True).start()
