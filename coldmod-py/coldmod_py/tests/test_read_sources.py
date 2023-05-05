import os
from coldmod_py.read_sources import read_source, list_sources, read_sources

def test_read_source_from_sample_trace_target_1_helper():
    source = os.path.abspath('./coldmod/samples/trace_target_1/helper.py')
    fn_sources = read_source(source)

    [a, b, c, d] = fn_sources

    assert a.file == source
    assert a.lineno == 2

    assert b.file == source
    assert b.lineno == 6

    assert c.file == source
    assert c.lineno == 10

    assert d.file == source
    assert d.lineno == 13


def test_read_source_from_sample_trace_target_1():
    fn_sources = read_sources(list_sources('./coldmod/samples/trace_target_1'))
    assert len(list(fn_sources)) == 6
