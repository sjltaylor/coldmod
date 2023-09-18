import grpc
import coldmod_py.config as config
import coldmod_msg.proto.tracing_pb2 as tracing_pb2
import coldmod_msg.proto.tracing_pb2_grpc as tracing_pb2_grpc

def grpc_channel() -> grpc.Channel:
    grpc_host = config.env.grpc_host()
    if config.env.insecure():
        return grpc.insecure_channel(grpc_host)

    rootca = open(config.env.ca(), "rb").read()
    creds = grpc.ssl_channel_credentials(root_certificates=rootca)
    return grpc.secure_channel(grpc_host, creds)

def fetch(all: bool) -> tracing_pb2.HeatMap:
    with grpc_channel() as channel:
        stub = tracing_pb2_grpc.TracesStub(channel)

        fetch_options = tracing_pb2.FetchOptions(all=all)

        if config.env.insecure():
            heat_map = stub.fetch(fetch_options)
        else:
            heat_map = stub.fetch.with_call(fetch_options, metadata=[("authorization", f"Bearer {config.env.api_key()}")])

    return heat_map
