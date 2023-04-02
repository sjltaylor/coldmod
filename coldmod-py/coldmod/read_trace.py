from typing import List
from coldmod.repr import repr_vars

@repr_vars
class CallTrace():
    def __init__(self, call):
        parts = call.split(' ')
        self.lineno : int = int(parts[1])
        self.fn_name : str = parts[2]
        self.class_name : str = parts[3]
        self.path : str = ' '.join(parts[4:]) # because there might have been spaces in the path

def read_trace(file) -> List[CallTrace]:
    return list(map(CallTrace, map(str.strip, open(file, 'r').readlines())))