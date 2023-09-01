import libcst
import libcst.metadata
from typing import Dict, Iterable, List, Tuple, Any, Optional
from .parsed_trace_src import ParsedTraceSrc, TraceSrc2
from coldmod_py.code2 import parsed_trace_src


class FunctionFinder(libcst.CSTVisitor):
    METADATA_DEPENDENCIES = (libcst.metadata.PositionProvider,libcst.metadata.FullyQualifiedNameProvider,)

    def __init__(self):
        super().__init__()
        self.trace_srcs = []

    def visit_FunctionDef(self, node: libcst.FunctionDef) -> Optional[bool]:
        name = node.name.value

        pos = self.get_metadata(libcst.metadata.PositionProvider, node).start
        name_pos = self.get_metadata(libcst.metadata.PositionProvider, node.name).start
        fqns = self.get_metadata(libcst.metadata.FullyQualifiedNameProvider, node)

        if len(fqns) != 1:
            raise Exception("Expected exactly one fully qualified name for function definition.")

        t = TraceSrc2()
        t.key = list(fqns)[0].name

        p = parsed_trace_src.ParsedTraceSrc(name_position=name_pos, position=pos, trace_src=t)
        self.trace_srcs.append(p)

        return True # visit nested functions
