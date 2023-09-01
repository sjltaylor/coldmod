import fire # https://github.com/google/python-fire/blob/master/docs/guide.md
import logging
import grpc
import coldmod_msg.proto.tracing_pb2 as tracing_pb2
import coldmod_msg.proto.tracing_pb2_grpc as tracing_pb2_grpc
from coldmod_py import coldmod_d
from coldmod_py import config

class CLI:
    def __init__(self, path=None, verbose=False):
        if verbose:
            logging.basicConfig(level=logging.DEBUG)

    def trace(self, key):
        trace = tracing_pb2.Trace(key=key, process_id="0", thread_id="0")
        traces = [trace]

        with coldmod_d.grpc_channel() as channel:
            stub = tracing_pb2_grpc.TracesStub(channel)
            if config.env.insecure():
                    stub.collect(iter(traces))
            else:
                metadata = [("authorization", f"Bearer {config.env.api_key()}")]
                stub.collect.with_call(iter(traces), metadata=metadata)


if __name__ == "__main__":
    try:
        fire.Fire(CLI)
    except KeyboardInterrupt:
        pass
