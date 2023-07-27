import libcst as cst
from .visitor import _visit_module

def test_visit_no_functions():
    module = cst.parse_module('')
    tracing_srcs = _visit_module("path", module)
    assert len(list(tracing_srcs)) == 0

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
    tracing_srcs = list(_visit_module("the/path", module))
    assert len(tracing_srcs) == 4

    uniq_digests = set([s.digest for s in tracing_srcs])
    assert len(uniq_digests) == 4

    tracing_src = tracing_srcs[3]
    assert tracing_src.name == "__init__"
    assert tracing_src.path == "the/path"
    assert tracing_src.lineno == 15
    assert tracing_src.class_name_path == "Class1"
