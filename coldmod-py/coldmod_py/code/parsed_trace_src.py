import coldmod_msg.proto.tracing_pb2 as tracing_pb2
import ast
import libcst
import libcst.metadata
from libcst.metadata.scope_provider import _ASSIGNMENT_LIKE_NODES

class ParsedTraceSrc():
    trace_src: tracing_pb2.TraceSrc
    name_position: libcst.metadata.CodePosition

    def __init__(self, trace_src: tracing_pb2.TraceSrc, name_position: libcst.metadata.CodePosition):
        self.trace_src = trace_src
        self.name_position = name_position
