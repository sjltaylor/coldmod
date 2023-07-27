import ast

src = open('coldmod_py/samples/trace_target_1/helper.py', 'r').read()
mod = ast.parse(src)

class V(ast.NodeVisitor):
    def __init__(self):
        self.stack = []

    def visit(self, node):
        print('visiting');
        self.stack.insert(0, node)
        self.generic_visit(node)


    def visit_FunctionDef(self, node):
        print(node.name, node.lineno, node.__module__)
        self.generic_visit(node)

v = V()

v.visit(mod)
