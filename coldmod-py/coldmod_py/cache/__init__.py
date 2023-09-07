from typing import Dict, Callable, List
import coldmod_py.config
import coldmod_py.files as files
from coldmod_py.code import ParsedTraceSrc
from pathlib import Path
import logging
import hashlib
import pickle
import shutil

LOG=logging.getLogger(__name__)

class CachedTraceSrcs:
    key_to_path: Dict[str, str]
    path_to_key: Dict[str, str]

def cache_dir() -> Path:
    d = Path(".coldmod-cache")
    d.mkdir(exist_ok=True, parents=True)
    return d

def clear():
    shutil.rmtree(cache_dir().absolute())

def parsed_trace_srcs(relative_path, origin: Callable[[], List[ParsedTraceSrc]]) -> List[ParsedTraceSrc]:
    file_path = Path(relative_path)
    h = hashlib.blake2b()
    h.update(b"parsed_trace_src:")
    h.update(relative_path.encode("utf-8"))
    h.update(b":")
    h.update(file_path.read_bytes())

    digest = h.hexdigest()

    cache = cache_dir().joinpath(digest)

    if cache.exists():
        LOG.debug(f"cache HIT for trace_src {relative_path}")
        parsed_trace_src = pickle.loads(cache.read_bytes())
    else:
        LOG.debug(f"cache MISS for trace_src {relative_path}")
        parsed_trace_src = origin()
        cache.write_bytes(pickle.dumps(parsed_trace_src))

    return parsed_trace_src
