from typing import Dict, Iterable, List, Tuple, Any
from .tracing_src import TracingSrc
import ast
from hashlib import blake2b

class _FunctionVisitor(ast.NodeVisitor):
    def __init__(self):
        self.src_functions: List[Tuple[str, int, str]] = []

    def visit_FunctionDef(self, node: ast.FunctionDef) -> Any:
        digest = blake2b(ast.dump(node).encode('utf-8')).hexdigest()
        self.src_functions.append((node.name, node.lineno, digest))
        self.generic_visit(node)

def _visit_module(path: str, module: ast.Module) -> Iterable[TracingSrc]:
    visitor = _FunctionVisitor()
    visitor.visit(module)
    for (name, lineno, digest) in visitor.src_functions:
        yield TracingSrc(path=path, name=name, lineno=lineno, digest=digest)

def _visit_all(modules: Dict[str, ast.Module]) -> Iterable[TracingSrc]:
    for path, module in modules.items():
        for source in _visit_module(path, module):
            yield source
