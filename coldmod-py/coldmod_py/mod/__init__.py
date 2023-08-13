import coldmod_msg.proto.tracing_pb2 as tracing_pb2
from coldmod_py.code.parsed_trace_src import ParsedTraceSrc
from typing import List, Iterable, Set, Optional
import logging
from libcst import FunctionDef, BaseStatement, FlattenSentinel, RemovalSentinel, RemoveFromParent
import libcst.codemod
import coldmod_py.code as code
import coldmod_py.files as files
from coldmod_py.code.digest import function_def_digest
import os

class RemoveFilterset(libcst.codemod.ContextAwareTransformer):
    def __init__(self, context: libcst.codemod.CodemodContext, filterset_digests: Set[str], removed: Set[str], path: str) -> None:
        super().__init__(context)
        self.filterset_digests = filterset_digests
        self.path = path
        self.removed = removed
        self.class_name_stack: List[str] = []

    def visit_ClassDef(self, node: libcst.ClassDef) -> Optional[bool]:
        self.class_name_stack.append(node.name.value)

    def leave_ClassDef(self, node: libcst.ClassDef, __: libcst.ClassDef) -> BaseStatement | FlattenSentinel[BaseStatement] | RemovalSentinel:
        self.class_name_stack.pop()
        return node


    def leave_FunctionDef(self, original_node: FunctionDef, updated_node: FunctionDef) -> BaseStatement | FlattenSentinel[BaseStatement] | RemovalSentinel:
        src = self.module.code_for_node(original_node)
        class_name_path = '.'.join(self.class_name_stack) if self.class_name_stack else None
        digest = function_def_digest(src, rel_module_path=self.path, class_name_path=class_name_path)

        if digest in self.filterset_digests:
            self.removed.add(digest)
            return RemoveFromParent()

        return updated_node

def remove(srcs_root_dir, filterset: tracing_pb2.FilterSet, srcs: List[str]):
    filterset_digests = set([e.digest for e in filterset.trace_srcs])

    modules = code.parse_modules(files.read_all(srcs))
    removed = set()

    for abs_path, m in modules.items():
        context = libcst.codemod.CodemodContext()
        rel_path = os.path.relpath(abs_path, srcs_root_dir)
        remove_transform = RemoveFilterset(context=context, filterset_digests=filterset_digests, removed=removed, path=rel_path)
        new_module = remove_transform.transform_module(m)
        with open(abs_path, "w") as src_file:
            src_file.write(new_module.code)
