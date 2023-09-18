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
import queue

def generate_app_url() -> Tuple[str, str]:
    bytes = secrets.token_bytes(32)
    key = f"cm-{base64.urlsafe_b64encode(bytes).decode('utf-8')}"
    return (f"{config.env.web_app_url()}/connect/{key}", key)

def extract_key(web_app_url: str) -> str:
    segments = urlparse(web_app_url).path.split('/')[1:]

    if len(segments) >= 2 and segments[0] == "connect":
        return segments[1]

    raise Exception(f"invalid web_app_url: {web_app_url}")

def _stream_src_message_queue(q) -> Iterable[tracing_pb2.SrcMessage]:
    while True:
        yield q.get()

def stream_commands(src_message_queue: queue.Queue[tracing_pb2.SrcMessage]) -> Iterable[tracing_pb2.ModCommand]:

    src_message_stream = _stream_src_message_queue(src_message_queue)

    with coldmod_d.grpc_channel() as channel:
        stub = tracing_pb2_grpc.TracesStub(channel)

        if config.env.insecure():
            mod_commands = stub.mod(src_message_stream)
        else:
            mod_commands = stub.mod.with_call(src_message_stream, metadata=[("authorization", f"Bearer {config.env.api_key()}")])
        for mod_command in mod_commands:
            yield mod_command
