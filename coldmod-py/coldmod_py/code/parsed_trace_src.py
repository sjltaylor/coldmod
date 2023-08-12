import coldmod_msg.proto.tracing_pb2 as tracing_pb2
import ast
import libcst
import libcst.metadata
from libcst.metadata.scope_provider import _ASSIGNMENT_LIKE_NODES

class ParsedTraceSrc():
    trace_src: tracing_pb2.TraceSrc
    function_def: libcst.FunctionDef

    def __init__(self, trace_src: tracing_pb2.TraceSrc, function_def: libcst.FunctionDef):
        self.trace_src = trace_src
        self.function_def = function_def
