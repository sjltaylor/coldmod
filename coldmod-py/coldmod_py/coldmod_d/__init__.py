import grpc
import coldmod_py.config as config

def grpc_channel() -> grpc.Channel:
    if config.INSECURE:
        return grpc.insecure_channel(config.COLDMOD_GRPC_HOST)

    rootca = open(config.COLDMOD_CA, "rb").read()
    creds = grpc.ssl_channel_credentials(root_certificates=rootca)
    return grpc.secure_channel(config.COLDMOD_GRPC_HOST, creds)
