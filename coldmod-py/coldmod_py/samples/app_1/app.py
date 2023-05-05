from . import helper
import coldmod_py.write_trace

def useless_function(): #noqa
    pass

def get_helper():
    name="world"
    return helper.NameHelper(name=name)

def run():
    return get_helper().message()
