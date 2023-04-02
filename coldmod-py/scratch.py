from coldmod.read_sources import read_sources, list_sources, FnSource
from coldmod.read_trace import read_trace
from coldmod.write_trace import init_from_trace_root
from pprint import pp
import os
import sys

import threading

def m1():
    t = read_trace('/Users/sam/Documents/projects/sjltaylor/rust/coldmod/coldmod/err.out')

    sources = list_sources('coldmod/samples/trace_target_1')

    fns = list(read_sources(sources))

    pp(fns)
    print("\n\n")
    # pp(t)


