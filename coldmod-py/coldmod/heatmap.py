from typing import Iterable, List
from coldmod.repr import repr_vars
from coldmod.read_trace import CallTrace
from coldmod.read_sources import FnSource

@repr_vars
class Heatmap():
    def __init__(self) -> None:
        self.called: List[FnSource] = []
        self.not_called: List[FnSource] = []

def create_heatmap(call_traces: Iterable[CallTrace], fn_sources: Iterable[FnSource]) -> Heatmap:
    heatmap = Heatmap()
    lookup = {}

    for src in call_traces:
        lookup[f"{src.path}:{src.lineno}"] = src

    for src in fn_sources:
        if f"{src.file}:{src.lineno}" in lookup:
            heatmap.called.append(src)
        else:
            heatmap.not_called.append(src)

    return heatmap
