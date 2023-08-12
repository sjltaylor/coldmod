import coldmod_msg.proto.tracing_pb2 as tracing_pb2
from coldmod_py.code.parsed_trace_src import ParsedTraceSrc
from typing import List
import logging

def remove(parsed_trace_srcs: List[ParsedTraceSrc], filterset: tracing_pb2.FilterSet):
    filterset_by_digest = {e.digest: e for e in filterset.trace_srcs}
    srcs_by_digest = {e.trace_src.digest: e for e in parsed_trace_srcs}

    print(f"srcs in filterset: {len(filterset.trace_srcs)}")
    print(f"parsed srcs: {len(srcs_by_digest)}")

    matched_filterset = {k:v for k,v in filterset_by_digest.items() if k in srcs_by_digest}

    print(f"matched srcs: {len(matched_filterset)}")

    # TODO:.....
    # figure out how to remove a fn from a parsed tree
    # parse the srcs I have on the fs
    # look for a match

    pass
