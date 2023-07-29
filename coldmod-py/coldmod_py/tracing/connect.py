from typing import Iterable
import coldmod_msg.proto.tracing_pb2 as tracing_pb2
import coldmod_msg.proto.tracing_pb2_grpc as tracing_pb2_grpc
import grpc

def register_trace_srcs(root_path: str, trace_srcs: Iterable[tracing_pb2.TraceSrc]):
    trace_srcs_msg = tracing_pb2.TraceSrcs(root_path=root_path, trace_srcs=trace_srcs)
    with grpc.insecure_channel("127.0.0.1:7777") as channel:
        stub = tracing_pb2_grpc.TracesStub(channel)
        stub.register(trace_srcs_msg)
