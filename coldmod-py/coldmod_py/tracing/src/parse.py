from typing import Dict, Tuple
import coldmod_py.files as files
import logging
import ast

LOG=logging.getLogger(__name__)

def _parse_source(source: Tuple[str, str]) -> Tuple[str, ast.Module] | SyntaxError:
    path, raw = source
    try:
        parsed = ast.parse(raw)
    except SyntaxError as e:
        LOG.exception(f"parsing failed: {path}")
        return e
    return path, parsed



def _parse_all(sources: Dict[str,str]) -> Dict[str, ast.Module]:
    valid_sources = [t for t in map(_parse_source, sources.items()) if not isinstance(t, SyntaxError)]
    return dict(valid_sources)
