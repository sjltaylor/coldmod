import libcst
import libcst.metadata

class TraceSrc2:
    key: str

class ParsedTraceSrc:
    position: libcst.metadata.CodePosition
    name_position: libcst.metadata.CodePosition
    trace_src: TraceSrc2

    def __init__(self, *, position: libcst.metadata.CodePosition, name_position: libcst.metadata.CodePosition, trace_src: TraceSrc2):
        self.trace_src = trace_src
        self.position = position
        self.name_position = name_position
