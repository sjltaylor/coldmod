from typing import Dict, Tuple
import coldmod_py.files as files
import logging
import libcst as cst
import ast

LOG=logging.getLogger(__name__)

def _parse_source(source: Tuple[str, str]) -> Tuple[str, cst.Module] | SyntaxError:
    path, raw = source
    try:
        parsed = cst.parse_module(raw)
    except SyntaxError as e:
        LOG.exception(f"parsing failed: {path}")
        return e
    return path, parsed

def parse_modules(sources: Dict[str,str]) -> Dict[str, cst.Module]:
    valid_sources = [t for t in map(_parse_source, sources.items()) if not isinstance(t, SyntaxError)]
    return dict(valid_sources)
