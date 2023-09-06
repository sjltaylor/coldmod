from typing import Iterable, Tuple
import coldmod_msg.proto.tracing_pb2 as tracing_pb2
import coldmod_msg.proto.tracing_pb2_grpc as tracing_pb2_grpc
import google.protobuf.empty_pb2
import grpc
import secrets
import base64
from urllib.parse import urlparse
from coldmod_py import coldmod_d
from coldmod_py import config

def extract_key(web_app_url: str) -> str:
    segments = urlparse(web_app_url).path.split('/')[1:]

    if len(segments) >= 2 and segments[0] == "connect":
        return segments[1]

    raise Exception(f"couldn't extract connect key from '{web_app_url}'")

def stream_filterset(web_app_url: str) -> Iterable[tracing_pb2.TraceSrcs]:
    q = tracing_pb2.FilterSetQuery(key=web_app_url)
    with coldmod_d.grpc_channel() as channel:
        stub = tracing_pb2_grpc.TracesStub(channel)

        if config.env.insecure():
            filtersets = stub.stream_filtersets(q)
        else:
            filtersets = stub.stream_filtersets.with_call(q, metadata=[("authorization", f"Bearer {config.env.api_key()}")])

        for filterset in filtersets:
            yield filterset
