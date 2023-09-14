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


class _RemoveAndCommentRefs(libcst.codemod.ContextAwareTransformer):
    METADATA_DEPENDENCIES = (libcst.metadata.PositionProvider,libcst.metadata.FullyQualifiedNameProvider,)

    def __init__(self, context: libcst.codemod.CodemodContext, remove: Set[str], removed: Set[str], ref_lines: List[int], comment: str) -> None:
        super().__init__(context)
        self.remove = remove
        self.removed = removed
        self.lines = ref_lines
        self.comment = comment

    def leave_FunctionDef(self, original_node: FunctionDef, updated_node: FunctionDef) -> BaseStatement | FlattenSentinel[BaseStatement] | RemovalSentinel:
        fqns = self.get_metadata(libcst.metadata.FullyQualifiedNameProvider, original_node)
        for fqn in fqns:
            if fqn in self.remove:
                self.removed.add(fqn)
                return RemoveFromParent()

        return updated_node

    def leave_SimpleStatementLine(self, original_node: libcst.CSTNodeT, updated_node: libcst.CSTNodeT) -> Union[libcst.CSTNodeT, RemovalSentinel, FlattenSentinel[libcst.CSTNodeT]]:
        if self.get_metadata(libcst.metadata.PositionProvider, original_node).start.line in self.lines:
            print('commenting')
            comment_node = libcst.EmptyLine(indent=True, comment=libcst.Comment(value=self.comment))
            return FlattenSentinel([comment_node, original_node])
        return original_node


def remove(root_dir: str, remote_trace_srcs: Iterable[tracing_pb2.TraceSrc], local_src_files: Iterable[str]) -> Set[str]:
    local_src_files_by_path = files.read_all(local_src_files)
    modules = code.parse_modules(local_src_files_by_path)
    local_trace_srcs = code.find_trace_srcs_by_relative_paths(local_src_files)

    # key -> (rel_path, parsed_src)
    local_trace_srcs_by_key = {
        parsed_trace_src.trace_src.key:(path, parsed_trace_src)
            for path, parsed_trace_srcs in local_trace_srcs.items()
                for parsed_trace_src in parsed_trace_srcs
    }


    remove = set([e.key for e in remote_trace_srcs])
    removed = set() # TODO: useful for assertions


    for remote in remote_trace_srcs:
        local = local_trace_srcs_by_key.get(remote.key)
        if local is None:
            continue
        (rel_path, local_parsed_trace_src) = local
        abs_path = os.path.join(root_dir, rel_path)

        project = jedi.Project(root_dir)
        script = jedi.Script(local_src_files_by_path[abs_path], path=abs_path, project=project)
        refs = script.get_references(line=local_parsed_trace_src.name_position.line, column=local_parsed_trace_src.name_position.column)

        refs_by_path = {}

        for r in refs:
            ref_abs_path = str(r.module_path)
            if refs_by_path.get(ref_abs_path) is None:
                refs_by_path[ref_abs_path] = []
            refs_by_path[ref_abs_path].append(r.line)


        context = libcst.codemod.CodemodContext()
        for p, lines in refs_by_path.items():
            module = modules[p]
            comment_transform = _RemoveAndCommentRefs(context=context, remove=remove, removed=removed, ref_lines=lines, comment=f"# TODO: {local_parsed_trace_src.trace_src.key} was removed by coldmod.")
            modules[p] = comment_transform.transform_module(module)
            print(modules[p].code == module.code)


        for p in refs_by_path.keys():
            m = modules[p]
            with open(f"{p}", "w") as f:
                f.write(m.code)

        break

    return removed
