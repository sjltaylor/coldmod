from logging import Manager
from coldmod_py.code import parsed_trace_src
from coldmod_py.code.parsed_trace_src import ParsedTraceSrc
from coldmod_msg.proto import tracing_pb2
from typing import Iterable, Tuple, List
from queue import Queue
import jedi
from pathlib import Path
import threading
import logging

LOG = logging.getLogger(__name__)

def refs(root_dir: str, parsed_trace_src: ParsedTraceSrc, path: Path) -> List[Tuple[str, int]]:
    project = jedi.Project(root_dir)
    script = jedi.Script(path.read_text(), path=path, project=project)

    LOG.debug(f"Looking for references to {parsed_trace_src.trace_src.key} : line={parsed_trace_src.name_position.line}")

    refs = script.get_references(line=parsed_trace_src.name_position.line, column=parsed_trace_src.name_position.column)

    return [(str(r.module_path), r.line) for r in refs]


def queue_src_refs(root_dir: str, parsed_trace_srcs: Iterable[Tuple[ParsedTraceSrc, Path]], q: Queue[tracing_pb2.SrcMessage], stop_event: threading.Event):
    create_src_refs_message = lambda n,k: tracing_pb2.SrcMessage(src_refs=tracing_pb2.SrcRefs(count=n, key=k))
    for (parsed_trace_src, path) in parsed_trace_srcs:
        if stop_event.is_set():
            break

        try:
            ref_locs = refs(root_dir, parsed_trace_src, path)
        except ValueError:
            LOG.debug("error finding references for:", parsed_trace_src.trace_src.key)
            # it might have been removed, this is expected
            continue
        src_message = create_src_refs_message(len(ref_locs), parsed_trace_src.trace_src.key)
        q.put_nowait(src_message)
