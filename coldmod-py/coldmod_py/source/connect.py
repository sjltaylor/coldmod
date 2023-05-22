from typing import Iterable
import coldmod_msg.proto.source_pb2 as source_pb2
import coldmod_msg.proto.source_pb2_grpc as source_pb2_grpc
import grpc

def _fn_to_elem(fn: source_pb2.SourceFn) -> source_pb2.SourceElement:
    return source_pb2.SourceElement(fn=fn)

def submit_source_scan(colmod_root_marker_path: str, source_fns: Iterable[source_pb2.SourceFn]):
    source_elements = map(_fn_to_elem, source_fns)
    source_scan = source_pb2.SourceScan(coldmod_root_marker_path=colmod_root_marker_path, source_elements=source_elements)
    with grpc.insecure_channel("127.0.0.1:7777") as channel:
        stub = source_pb2_grpc.SourceDaemonStub(channel)
        stub.submit(source_scan)
