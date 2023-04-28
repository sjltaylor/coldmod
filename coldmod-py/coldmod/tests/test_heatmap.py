import os
import pathlib
from coldmod.read_trace import read_trace
from coldmod.read_sources import read_sources, list_sources
from coldmod.heatmap import create_heatmap
from coldmod.read_sources import FnSource

SAMPLE_TRACE_TARGET_1_FILE = pathlib.Path(__file__).resolve().parent.joinpath('sample.trace_target_1')

def test_heatmap():
    call_traces = read_trace(SAMPLE_TRACE_TARGET_1_FILE)
    fn_sources = read_sources(list_sources('./coldmod/samples/trace_target_1'))

    heatmap = create_heatmap(call_traces, fn_sources)

    main_file = os.path.abspath('./coldmod/samples/trace_target_1/__main__.py')
    helper_file = os.path.abspath('./coldmod/samples/trace_target_1/helper.py')

    assert len(heatmap.called) == 4

    assert FnSource(main_file, 9) in heatmap.called
    assert FnSource(helper_file, 6) in heatmap.called
    assert FnSource(helper_file, 10) in heatmap.called
    assert FnSource(helper_file, 2) in heatmap.called

    assert len(heatmap.not_called) == 2

    assert FnSource(main_file, 6) in heatmap.not_called
    assert FnSource(helper_file, 13) in heatmap.not_called
