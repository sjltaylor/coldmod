from . import helper
import coldmod.write_trace

coldmod.write_trace.init_from_trace_root()

def useless_function(): #noqa
    pass

def get_helper():
    name="world"
    return helper.NameHelper(name=name)

print(get_helper().message())
