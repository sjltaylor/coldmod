import grpc
import coldmod_py.config as config

def grpc_channel() -> grpc.Channel:
    grpc_host = config.env.grpc_host()
    if config.env.insecure():
        return grpc.insecure_channel(grpc_host)

    rootca = open(config.env.ca(), "rb").read()
    creds = grpc.ssl_channel_credentials(root_certificates=rootca)
    return grpc.secure_channel(grpc_host, creds)
