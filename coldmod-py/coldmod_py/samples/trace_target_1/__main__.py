from . import helper
import coldmod_py

coldmod_py.tracing.coldmod_tracing_root_marker()

def useless_function(): #noqa
    pass

def get_helper():
    name="world"
    return helper.NameHelper(name=name)

print(get_helper().message())
