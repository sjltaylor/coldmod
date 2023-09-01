from typing import Iterable
import coldmod_msg.proto.tracing_pb2 as tracing_pb2
import coldmod_msg.proto.tracing_pb2_grpc as tracing_pb2_grpc
from coldmod_py import coldmod_d
import grpc
from coldmod_py import config

def register_trace_srcs(trace_srcs: Iterable[tracing_pb2.TraceSrc]):
    trace_srcs_msg = tracing_pb2.TraceSrcs(trace_srcs=trace_srcs)
    with coldmod_d.grpc_channel() as channel:
        stub = tracing_pb2_grpc.TracesStub(channel)
        if config.env.insecure():
            stub.set(trace_srcs_msg)
        else:
            metadata = [("authorization", f"Bearer {config.env.api_key()}")]
            stub.set.with_call(trace_srcs_msg, metadata=metadata)
