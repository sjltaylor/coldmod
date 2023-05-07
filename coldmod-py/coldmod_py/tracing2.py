import coldmod_msg.proto.trace_pb2 as trace_pb2
import coldmod_msg.proto.trace_pb2_grpc as trace_pb2_grpc

from concurrent.futures import ThreadPoolExecutor
import logging
import threading
from typing import Iterator
import grpc

class TraceAgent:

    def __init__(self, executor: ThreadPoolExecutor, channel: grpc.Channel) -> None:
        self._executor = executor
        self._channel = channel
        self._stub = trace_pb2_grpc.TracingDaemonStub(self._channel)
        self._session_id = None
        self._consumer_future = None


    def send_trace(self) -> None:
        request = trace_pb2.Trace(line=145, path='/path/to/file/from/tracing2.py')
        self._stub.collect(iter((request,request,request,request,)))


def send_trace(executor: ThreadPoolExecutor, channel: grpc.Channel) -> None:
    a = TraceAgent(executor, channel)
    a.send_trace()


def run():
    executor = ThreadPoolExecutor()
    with grpc.insecure_channel("127.0.0.1:7777") as channel:
        future = executor.submit(send_trace, executor, channel)
        future.result()


if __name__ == '__main__':
    print("tracing 2 in full effect")
    logging.basicConfig(level=logging.INFO)
    run()
