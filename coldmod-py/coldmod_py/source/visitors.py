from typing import List, Tuple, Dict, Optional, Iterable

import libcst
import libcst.metadata

class FunctionFinder(libcst.CSTVisitor):
    METADATA_DEPENDENCIES = (libcst.metadata.PositionProvider,)

    def __init__(self):
        # stack for storing the canonical name of the current function
        self.stack: List[libcst.ClassDef] = []
        self.functions: List[Tuple[str, int, str|None]] = []

    def visit_ClassDef(self, node: libcst.ClassDef) -> Optional[bool]:
        self.stack.append(node)

    def leave_ClassDef(self, node: libcst.ClassDef) -> None:
        self.stack.pop()

    def visit_FunctionDef(self, node: libcst.FunctionDef) -> Optional[bool]:

        class_name = self.stack[-1].name.value if self.stack else None

        line = self.get_metadata(libcst.metadata.PositionProvider, node).start.line
        self.functions.append((node.name.value, line, class_name))

        return True # keep going, there might be nested functions
