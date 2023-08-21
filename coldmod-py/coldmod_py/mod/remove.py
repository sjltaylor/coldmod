import coldmod_msg.proto.tracing_pb2 as tracing_pb2
from coldmod_py.code.parsed_trace_src import ParsedTraceSrc
from typing import List, Iterable, Set, Optional, Dict, Union
import logging
from libcst import CSTNodeT, FunctionDef, BaseStatement, FlattenSentinel, RemovalSentinel, RemoveFromParent
import libcst.codemod
import coldmod_py.code as code
import coldmod_py.files as files
from coldmod_py.code.digest import function_def_digest
import os
import jedi
from libcst.metadata.full_repo_manager import FullRepoManager


class _RemoveAndCommentRefs(libcst.codemod.ContextAwareTransformer):
    METADATA_DEPENDENCIES = (libcst.metadata.PositionProvider,)

    def __init__(self, context: libcst.codemod.CodemodContext, remove: Set[str], removed: Set[str], path: str, ref_lines: List[int], comment: str) -> None:
        super().__init__(context)
        self.remove = remove
        self.path = path
        self.removed = removed
        self.class_name_stack: List[str] = []
        self.lines = ref_lines
        self.comment = comment

    def visit_ClassDef(self, node: libcst.ClassDef) -> BaseStatement | FlattenSentinel[BaseStatement] | RemovalSentinel:
        self.class_name_stack.append(node.name.value)
        return node

    def leave_ClassDef(self, node: libcst.ClassDef, update_node: libcst.ClassDef) -> BaseStatement | FlattenSentinel[BaseStatement] | RemovalSentinel:
        self.class_name_stack.pop()
        return update_node

    def leave_FunctionDef(self, original_node: FunctionDef, updated_node: FunctionDef) -> BaseStatement | FlattenSentinel[BaseStatement] | RemovalSentinel:
        src = self.module.code_for_node(original_node)
        class_name_path = '.'.join(self.class_name_stack) if self.class_name_stack else None
        digest = function_def_digest(src, rel_module_path=self.path, class_name_path=class_name_path)
        if digest in self.remove:
            self.removed.add(digest)
            return RemoveFromParent()

        return updated_node

    def leave_SimpleStatementLine(self, original_node: libcst.CSTNodeT, updated_node: libcst.CSTNodeT) -> Union[libcst.CSTNodeT, RemovalSentinel, FlattenSentinel[libcst.CSTNodeT]]:
        if self.get_metadata(libcst.metadata.PositionProvider, original_node).start.line in self.lines:
            print('commenting')
            comment_node = libcst.EmptyLine(indent=True, comment=libcst.Comment(value=self.comment))
            return FlattenSentinel([comment_node, original_node])
        return original_node

def remove(srcs_root_dir, remote_trace_srcs: Iterable[tracing_pb2.TraceSrc], src_files: Iterable[str]) -> Set[str]:
    srcs_by_path = files.read_all(src_files)
    modules = code.parse_modules(srcs_by_path)

    local_trace_srcs = code.find_trace_srcs_in(srcs_root_dir, modules)
    local_trace_srcs_by_digest = code.key_by_digest(local_trace_srcs)

    remove = set([e.digest for e in remote_trace_srcs])
    removed = set() # TODO: useful for assertions


    for remote in remote_trace_srcs:
        local = local_trace_srcs_by_digest.get(remote.digest)
        if local is None:
            continue
        abs_path = os.path.join(srcs_root_dir, local.trace_src.path)

        project = jedi.Project(srcs_root_dir)
        script = jedi.Script(srcs_by_path[abs_path], path=abs_path, project=project)
        refs = script.get_references(line=local.name_position.line, column=local.name_position.column)

        refs_by_path = {}

        for r in refs:
            ref_abs_path = str(r.module_path)
            if refs_by_path.get(ref_abs_path) is None:
                refs_by_path[ref_abs_path] = []
            refs_by_path[ref_abs_path].append(r.line)

        rel_path = os.path.relpath(abs_path, srcs_root_dir)


        context = libcst.codemod.CodemodContext()
        for p, lines in refs_by_path.items():
            module = modules[p]
            comment_transform = _RemoveAndCommentRefs(context=context, remove=remove, removed=removed, path=rel_path, ref_lines=lines, comment=f"# TODO: {local.trace_src.name} was removed by coldmod.")
            modules[p] = comment_transform.transform_module(module)
            print(modules[p].code == module.code)


        for p in refs_by_path.keys():
            m = modules[p]
            with open(f"{p}", "w") as f:
                f.write(m.code)

        break

    return removed
