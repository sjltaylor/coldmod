from . import helper
import coldmod.write_trace

coldmod.write_trace.init()

def useless_function(): #noqa
    pass

def get_helper():
    return helper.NameHelper(name="world")

print(get_helper().message())
