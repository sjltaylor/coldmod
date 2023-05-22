from dataclasses import Field
from typing import List, Dict, Iterable, Tuple
from glob import glob
import os
import libcst
import libcst.metadata
import coldmod_msg.proto.source_pb2 as source_pb2
from coldmod_py.source.visitors import FunctionFinder

def find_root_markers(sources: Dict[str,str]) -> Iterable[str]:
   for path, source in sources.items():
        if "coldmod_tracing_root_marker" in source:
            yield path

def _parse_source(source: Tuple[str, str]) -> Tuple[str, libcst.Module]:
    path, raw = source
    return path, libcst.parse_module(raw)

def parse_all(sources: Dict[str,str]) -> Dict[str, libcst.Module]:
    return dict(map(_parse_source, sources.items()))

def _find_functions_in(path: str, module: libcst.Module) -> Iterable[source_pb2.SourceFn]:
    wrapper = libcst.metadata.MetadataWrapper(module)

    visitor = FunctionFinder()
    wrapper.visit(visitor)

    def _from_visitor(fn: Tuple[str, int, str|None]):
        (name, line, class_name) = fn
        return source_pb2.SourceFn(path=path, name=name, line=line, class_name=class_name)

    return map(_from_visitor, visitor.functions)

def find_functions_in_all(modules:Dict[str, libcst.Module]) -> Iterable[source_pb2.SourceFn]:
    for path, module in modules.items():
        for source in _find_functions_in(path, module):
            yield source
