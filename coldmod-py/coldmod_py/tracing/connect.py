from typing import Iterable
import coldmod_msg.proto.heat_pb2 as heat_pb2
import coldmod_msg.proto.heat_pb2_grpc as heat_pb2_grpc
import grpc

def submit_heat_map(colmod_root_marker_path: str, heat_srcs: Iterable[heat_pb2.HeatSrc]):
    heat_map = heat_pb2.HeatMap(root_path=..., heat_srcs=heat_srcs)
    with grpc.insecure_channel("127.0.0.1:7777") as channel:
        stub = heat_pb2_grpc.HeatMapsStub(channel)
        stub.submit(heat_map)
