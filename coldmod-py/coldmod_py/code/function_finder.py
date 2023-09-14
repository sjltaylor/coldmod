import libcst
import libcst.metadata
from typing import Dict, Iterable, List, Tuple, Any, Optional
from coldmod_msg.proto import tracing_pb2
from .parsed_trace_src import ParsedTraceSrc


class FunctionFinder(libcst.CSTVisitor):
    METADATA_DEPENDENCIES = (libcst.metadata.PositionProvider,libcst.metadata.FullyQualifiedNameProvider,)

    def __init__(self):
        super().__init__()
        self.trace_srcs: List[ParsedTraceSrc] = []

    def visit_FunctionDef(self, node: libcst.FunctionDef) -> Optional[bool]:
        pos = self.get_metadata(libcst.metadata.PositionProvider, node).start
        name_pos = self.get_metadata(libcst.metadata.PositionProvider, node.name).start
        fqns = self.get_metadata(libcst.metadata.FullyQualifiedNameProvider, node)

        if len(fqns) != 1:
            raise Exception("Expected exactly one fully qualified name for function definition.")

        trace_src = tracing_pb2.TraceSrc()
        trace_src.key = list(fqns)[0].name

        parsed_trace_src = ParsedTraceSrc(name_position=name_pos, position=pos, trace_src=trace_src)
        self.trace_srcs.append(parsed_trace_src)

        return True # visit nested functions
