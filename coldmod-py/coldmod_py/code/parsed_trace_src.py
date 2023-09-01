import libcst
import libcst.metadata
import coldmod_msg.proto.tracing_pb2 as tracing_pb2

class ParsedTraceSrc:
    position: libcst.metadata.CodePosition
    name_position: libcst.metadata.CodePosition
    trace_src: tracing_pb2.TraceSrc

    def __init__(self, *, position: libcst.metadata.CodePosition, name_position: libcst.metadata.CodePosition, trace_src: tracing_pb2.TraceSrc):
        self.trace_src = trace_src
        self.position = position
        self.name_position = name_position
