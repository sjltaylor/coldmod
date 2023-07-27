import os
from typing import Dict, Iterable, List, Tuple, Any, Optional
from .tracing_src import TracingSrc
from hashlib import blake2b
import ast
import libcst
import libcst.metadata

class FunctionFinder(libcst.CSTVisitor):
    METADATA_DEPENDENCIES = (libcst.metadata.PositionProvider,)

    def __init__(self, *, module: libcst.Module, path: str):
        self.module = module
        self.path = path
        # stack for storing the canonical name of the current function
        self.class_name_stack: List[str] = []
        self.tracing_srcs: List[TracingSrc] = []

    def visit_ClassDef(self, node: libcst.ClassDef) -> Optional[bool]:
        self.class_name_stack.append(node.name.value)

    def leave_ClassDef(self, node: libcst.ClassDef) -> None:
        self.class_name_stack.pop()

    def visit_FunctionDef(self, node: libcst.FunctionDef) -> Optional[bool]:
        name = node.name.value
        class_name_path = '.'.join(self.class_name_stack) if self.class_name_stack else None
        line = self.get_metadata(libcst.metadata.PositionProvider, node).start.line
        stripped_src = ast.dump(ast.parse(self.module.code_for_node(node)))
        digest = blake2b(stripped_src.encode('utf-8')).hexdigest()

        self.tracing_srcs.append(TracingSrc(path=self.path, name=name, lineno=line, digest=digest, class_name_path=class_name_path, src=stripped_src))

        return False # don't visit nested functions


def _visit_module(path: str, module: libcst.Module) -> Iterable[TracingSrc]:
    wrapper = libcst.metadata.MetadataWrapper(module)

    visitor = FunctionFinder(module=module, path=path)
    wrapper.visit(visitor)

    return visitor.tracing_srcs

def _visit_all(srcs_root_dir: str, modules: Dict[str, libcst.Module]) -> Iterable[TracingSrc]:
    for abs_path, module in modules.items():
        path = os.path.relpath(abs_path, srcs_root_dir)
        for source in _visit_module(path, module):
            yield source
