from google.protobuf import empty_pb2 as _empty_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class Trace(_message.Message):
    __slots__ = ["key", "thread_id", "process_id"]
    KEY_FIELD_NUMBER: _ClassVar[int]
    THREAD_ID_FIELD_NUMBER: _ClassVar[int]
    PROCESS_ID_FIELD_NUMBER: _ClassVar[int]
    key: str
    thread_id: str
    process_id: str
    def __init__(self, key: _Optional[str] = ..., thread_id: _Optional[str] = ..., process_id: _Optional[str] = ...) -> None: ...

class TraceSrc(_message.Message):
    __slots__ = ["key"]
    KEY_FIELD_NUMBER: _ClassVar[int]
    key: str
    def __init__(self, key: _Optional[str] = ...) -> None: ...

class TraceSrcs(_message.Message):
    __slots__ = ["trace_srcs"]
    TRACE_SRCS_FIELD_NUMBER: _ClassVar[int]
    trace_srcs: _containers.RepeatedCompositeFieldContainer[TraceSrc]
    def __init__(self, trace_srcs: _Optional[_Iterable[_Union[TraceSrc, _Mapping]]] = ...) -> None: ...

class FilterSetQuery(_message.Message):
    __slots__ = ["key"]
    KEY_FIELD_NUMBER: _ClassVar[int]
    key: str
    def __init__(self, key: _Optional[str] = ...) -> None: ...
