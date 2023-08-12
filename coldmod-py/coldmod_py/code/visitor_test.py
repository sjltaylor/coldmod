import libcst as cst
from .visitor import _visit_module

def test_visit_no_functions():
    module = cst.parse_module('')
    parsed_trace_srcs = _visit_module("path", module)
    assert len(list(parsed_trace_srcs)) == 0

_src = """

def foo():
    pass

def bar():
    pass

def real_deal():
    print('hello world')
    if True:
        print('hello world, me again')

class Class1():
    def __init__(self):
        print('I am a constructor')

"""

def test_visit_many_functions():
    # a file with multiple functions incl one in a class and one with a duplicate body
    module = cst.parse_module(_src)
    parsed_trace_srcs = list(_visit_module("the/path", module))
    assert len(parsed_trace_srcs) == 4

    uniq_digests = set([s.trace_src.digest for s in parsed_trace_srcs])
    assert len(uniq_digests) == 4

    # assert node[3] is a cst.FunctionDef
    assert isinstance(parsed_trace_srcs[3].function_def, cst.FunctionDef)

    trace_src = parsed_trace_srcs[3].trace_src
    assert trace_src.name == "__init__"
    assert trace_src.path == "the/path"
    assert trace_src.lineno == 15
    assert trace_src.class_name_path == "Class1"
