import libcst as cst
from glob import glob
import os
from itertools import chain
from typing import List, Tuple, Dict, Optional, Iterable
from coldmod_py.repr import repr_vars

@repr_vars
class FnSource():
    def __init__(self, file: str, lineno: int) -> None:
        self.file : str = file
        self.lineno : int = lineno

    def __eq__(self, other: object) -> bool:
        if isinstance(other, FnSource):
           return self.file == other.file and self.lineno == other.lineno
        return super().__eq__(other)

@repr_vars
class FnFinder(cst.CSTVisitor):
    METADATA_DEPENDENCIES = (cst.metadata.PositionProvider,)

    def __init__(self):
        # stack for storing the canonical name of the current function
        self.stack: List[Tuple[str, ...]] = []
        # store the annotations
        self.annotations: Dict[
            Tuple[str, ...],  # key: tuple of canonical class/function name
            Tuple[cst.Parameters, Optional[cst.Annotation]],  # value: (params, returns)
        ] = {}
        self.fns: List[FnSource] = []

    def visit_ClassDef(self, node: cst.ClassDef) -> Optional[bool]:
        self.stack.append(node.name.value)

    def leave_ClassDef(self, node: cst.ClassDef) -> None:
        self.stack.pop()

    def visit_FunctionDef(self, node: cst.FunctionDef) -> Optional[bool]:
        self.stack.append(node.name.value)
        self.annotations[tuple(self.stack)] = (node.params, node.returns)
        line = self.get_metadata(cst.metadata.PositionProvider, node).start.line
        self.fns.append(line)

        return (
            False
        )  # pyi files don't support inner functions, return False to stop the traversal.

    def leave_FunctionDef(self, node: cst.FunctionDef) -> None:
        self.stack.pop()

def list_sources(path: str) -> List[str]:
    # Find all Python files in path
    return glob(os.path.join(path, "**/*.py"), recursive=True, )

def read_sources(files: List[str]) -> Iterable[FnSource]:
    return chain(*map(read_source, files))

# TODO: insist on absolute path
def read_source(src_file: str) -> Iterable[FnSource]:
    src_file = os.path.abspath(os.path.expanduser(src_file))
    src = open(src_file, 'r').read()

    module = cst.parse_module(src)
    wrapper = cst.metadata.MetadataWrapper(module)

    visitor = FnFinder()
    wrapper.visit(visitor)

    return map(lambda lineno: FnSource(src_file, lineno), visitor.fns)
