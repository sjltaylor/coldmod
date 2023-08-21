import os
from typing import Dict, Iterable, List, Tuple, Any, Optional
from coldmod_msg.proto.tracing_pb2 import TraceSrc
from hashlib import blake2b
from .parsed_trace_src import ParsedTraceSrc
import ast
import libcst
import libcst.metadata
from .digest import function_def_digest

class FunctionFinder(libcst.CSTVisitor):
    METADATA_DEPENDENCIES = (libcst.metadata.PositionProvider,)

    def __init__(self, *, module: libcst.Module, path: str):
        self.module = module
        self.path = path
        # stack for storing the canonical name of the current function
        self.class_name_stack: List[str] = []
        self.parsed_trace_srcs: List[ParsedTraceSrc] = []

    def visit_ClassDef(self, node: libcst.ClassDef) -> Optional[bool]:
        self.class_name_stack.append(node.name.value)

    def leave_ClassDef(self, node: libcst.ClassDef) -> None:
        self.class_name_stack.pop()

    def visit_FunctionDef(self, node: libcst.FunctionDef) -> Optional[bool]:
        name = node.name.value
        class_name_path = '.'.join(self.class_name_stack) if self.class_name_stack else None
        lineno = self.get_metadata(libcst.metadata.PositionProvider, node).start.line
        src = self.module.code_for_node(node)
        digest = function_def_digest(src, rel_module_path=self.path, class_name_path=class_name_path)

        trace_src = TraceSrc(path=self.path, name=name, lineno=lineno, digest=digest, class_name_path=class_name_path, src=src)
        name_position = self.get_metadata(libcst.metadata.PositionProvider, node.name).start
        self.parsed_trace_srcs.append(ParsedTraceSrc(trace_src=trace_src, name_position=name_position))

        return False # don't visit nested functions


def _visit_module(path: str, module: libcst.Module) -> Iterable[ParsedTraceSrc]:
    wrapper = libcst.metadata.MetadataWrapper(module)

    visitor = FunctionFinder(module=module, path=path)
    wrapper.visit(visitor)

    return visitor.parsed_trace_srcs

def _visit_all(srcs_root_dir: str, modules: Dict[str, libcst.Module]) -> Iterable[ParsedTraceSrc]:
    for abs_path, module in modules.items():
        rel_path = os.path.relpath(abs_path, srcs_root_dir)
        for source in _visit_module(rel_path, module):
            yield source
