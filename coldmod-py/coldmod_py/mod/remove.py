import coldmod_msg.proto.tracing_pb2 as tracing_pb2
from coldmod_py.code.parsed_trace_src import ParsedTraceSrc
from typing import List, Iterable, Set, Optional, Dict, Union
import logging
from libcst import CSTNodeT, FunctionDef, BaseStatement, FlattenSentinel, RemovalSentinel, RemoveFromParent
import libcst.codemod
import coldmod_py.code as code
import coldmod_py.files as files
import os
import jedi
from libcst.metadata.full_repo_manager import FullRepoManager
from libcst.metadata import FullyQualifiedNameProvider
from pathlib import Path
from .refs import refs as mod_refs

LOG=logging.getLogger(__name__)

class _RemoveAndCommentRefs(libcst.codemod.ContextAwareTransformer):
    METADATA_DEPENDENCIES = (libcst.metadata.PositionProvider,libcst.metadata.FullyQualifiedNameProvider,)

    def __init__(self, context: libcst.codemod.CodemodContext, remove: str, ref_lines: List[int], comment: str) -> None:
        super().__init__(context)
        self.remove = remove
        self.lines = ref_lines
        self.comment = comment

    def leave_FunctionDef(self, original_node: FunctionDef, updated_node: FunctionDef) -> BaseStatement | FlattenSentinel[BaseStatement] | RemovalSentinel:
        fqns = self.get_metadata(libcst.metadata.FullyQualifiedNameProvider, original_node)
        for fqn in fqns:
            if fqn.name == self.remove:
                return RemoveFromParent()
        return updated_node

    def leave_SimpleStatementLine(self, original_node: libcst.CSTNodeT, updated_node: libcst.CSTNodeT) -> Union[libcst.CSTNodeT, RemovalSentinel, FlattenSentinel[libcst.CSTNodeT]]:
        if self.get_metadata(libcst.metadata.PositionProvider, original_node).start.line in self.lines:
            comment_node = libcst.EmptyLine(indent=True, comment=libcst.Comment(value=self.comment))
            return FlattenSentinel([comment_node, original_node])
        return original_node


def remove(root_dir: str, parsed_trace_src: ParsedTraceSrc, path: Path):

    refs = mod_refs(root_dir, parsed_trace_src, path)

    ref_lines_by_relative_path: Dict[str, List[int]] = {}

    for (ref_relative_path, line) in refs:
        relative_path = os.path.relpath(ref_relative_path, root_dir)
        if ref_lines_by_relative_path.get(relative_path) is None:
            ref_lines_by_relative_path[relative_path] = []
        ref_lines_by_relative_path[relative_path].append(line)


    frm = FullRepoManager(".", {*ref_lines_by_relative_path.keys()}, {FullyQualifiedNameProvider})
    context = libcst.codemod.CodemodContext()

    comment = f"# TODO: {parsed_trace_src.trace_src.key} was removed by coldmod."

    for (ref_relative_path, lines) in ref_lines_by_relative_path.items():
        wrapper = frm.get_metadata_wrapper_for_path(ref_relative_path)
        comment_transform = _RemoveAndCommentRefs(context=context, remove=parsed_trace_src.trace_src.key, ref_lines=lines, comment=comment)
        new_module = wrapper.visit(comment_transform)

        Path(ref_relative_path).write_text(new_module.code)
