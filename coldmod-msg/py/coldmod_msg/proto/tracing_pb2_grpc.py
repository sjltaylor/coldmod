# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
"""Client and server classes corresponding to protobuf-defined services."""
import grpc

from coldmod_msg.proto import tracing_pb2 as coldmod__msg_dot_proto_dot_tracing__pb2
from google.protobuf import empty_pb2 as google_dot_protobuf_dot_empty__pb2


class TracesStub(object):
    """Missing associated documentation comment in .proto file."""

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.collect = channel.stream_unary(
                '/coldmod_msg.proto.tracing.Traces/collect',
                request_serializer=coldmod__msg_dot_proto_dot_tracing__pb2.Trace.SerializeToString,
                response_deserializer=google_dot_protobuf_dot_empty__pb2.Empty.FromString,
                )
        self.set = channel.unary_unary(
                '/coldmod_msg.proto.tracing.Traces/set',
                request_serializer=coldmod__msg_dot_proto_dot_tracing__pb2.TraceSrcs.SerializeToString,
                response_deserializer=google_dot_protobuf_dot_empty__pb2.Empty.FromString,
                )
        self.stream_filtersets = channel.unary_stream(
                '/coldmod_msg.proto.tracing.Traces/stream_filtersets',
                request_serializer=coldmod__msg_dot_proto_dot_tracing__pb2.FilterSetQuery.SerializeToString,
                response_deserializer=coldmod__msg_dot_proto_dot_tracing__pb2.TraceSrcs.FromString,
                )


class TracesServicer(object):
    """Missing associated documentation comment in .proto file."""

    def collect(self, request_iterator, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def set(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def stream_filtersets(self, request, context):
        """Missing associated documentation comment in .proto file."""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')


def add_TracesServicer_to_server(servicer, server):
    rpc_method_handlers = {
            'collect': grpc.stream_unary_rpc_method_handler(
                    servicer.collect,
                    request_deserializer=coldmod__msg_dot_proto_dot_tracing__pb2.Trace.FromString,
                    response_serializer=google_dot_protobuf_dot_empty__pb2.Empty.SerializeToString,
            ),
            'set': grpc.unary_unary_rpc_method_handler(
                    servicer.set,
                    request_deserializer=coldmod__msg_dot_proto_dot_tracing__pb2.TraceSrcs.FromString,
                    response_serializer=google_dot_protobuf_dot_empty__pb2.Empty.SerializeToString,
            ),
            'stream_filtersets': grpc.unary_stream_rpc_method_handler(
                    servicer.stream_filtersets,
                    request_deserializer=coldmod__msg_dot_proto_dot_tracing__pb2.FilterSetQuery.FromString,
                    response_serializer=coldmod__msg_dot_proto_dot_tracing__pb2.TraceSrcs.SerializeToString,
            ),
    }
    generic_handler = grpc.method_handlers_generic_handler(
            'coldmod_msg.proto.tracing.Traces', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))


 # This class is part of an EXPERIMENTAL API.
class Traces(object):
    """Missing associated documentation comment in .proto file."""

    @staticmethod
    def collect(request_iterator,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.stream_unary(request_iterator, target, '/coldmod_msg.proto.tracing.Traces/collect',
            coldmod__msg_dot_proto_dot_tracing__pb2.Trace.SerializeToString,
            google_dot_protobuf_dot_empty__pb2.Empty.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def set(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/coldmod_msg.proto.tracing.Traces/set',
            coldmod__msg_dot_proto_dot_tracing__pb2.TraceSrcs.SerializeToString,
            google_dot_protobuf_dot_empty__pb2.Empty.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def stream_filtersets(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_stream(request, target, '/coldmod_msg.proto.tracing.Traces/stream_filtersets',
            coldmod__msg_dot_proto_dot_tracing__pb2.FilterSetQuery.SerializeToString,
            coldmod__msg_dot_proto_dot_tracing__pb2.TraceSrcs.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)
