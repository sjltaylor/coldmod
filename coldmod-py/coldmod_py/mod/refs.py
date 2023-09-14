from coldmod_py.code import parsed_trace_src
from coldmod_py.code.parsed_trace_src import ParsedTraceSrc
from coldmod_msg.proto import tracing_pb2
from typing import Iterable, Tuple, List
from queue import Queue
import jedi
from pathlib import Path
import threading

def refs(root_dir: str, parsed_trace_src: ParsedTraceSrc, path: Path) -> List[str]:
    project = jedi.Project(root_dir)
    script = jedi.Script(path.read_text(), path=path, project=project)
    refs = script.get_references(line=parsed_trace_src.name_position.line, column=parsed_trace_src.name_position.column)

    return [f"{r.module_path}:{r.line}" for r in refs]


def queue_src_refs(root_dir: str, parsed_trace_srcs: Iterable[Tuple[ParsedTraceSrc, Path]], q: Queue[tracing_pb2.SrcMessage], stop_event: threading.Event):
    create_src_refs_message = lambda n,k: tracing_pb2.SrcMessage(src_refs=tracing_pb2.SrcRefs(count=n, key=k))
    for (parsed_trace_src, path) in parsed_trace_srcs:
       ref_locs = refs(root_dir, parsed_trace_src, path)
       src_message = create_src_refs_message(len(ref_locs), parsed_trace_src.trace_src.key)
       q.put_nowait(src_message)
       if stop_event.is_set():
           break
