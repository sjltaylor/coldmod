from google.protobuf import empty_pb2 as _empty_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class Trace(_message.Message):
    __slots__ = ["digest", "thread_id", "process_id"]
    DIGEST_FIELD_NUMBER: _ClassVar[int]
    THREAD_ID_FIELD_NUMBER: _ClassVar[int]
    PROCESS_ID_FIELD_NUMBER: _ClassVar[int]
    digest: str
    thread_id: int
    process_id: int
    def __init__(self, digest: _Optional[str] = ..., thread_id: _Optional[int] = ..., process_id: _Optional[int] = ...) -> None: ...

class TraceSrc(_message.Message):
    __slots__ = ["path", "lineno", "name", "class_name_path", "src", "digest"]
    PATH_FIELD_NUMBER: _ClassVar[int]
    LINENO_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    CLASS_NAME_PATH_FIELD_NUMBER: _ClassVar[int]
    SRC_FIELD_NUMBER: _ClassVar[int]
    DIGEST_FIELD_NUMBER: _ClassVar[int]
    path: str
    lineno: int
    name: str
    class_name_path: str
    src: str
    digest: str
    def __init__(self, path: _Optional[str] = ..., lineno: _Optional[int] = ..., name: _Optional[str] = ..., class_name_path: _Optional[str] = ..., src: _Optional[str] = ..., digest: _Optional[str] = ...) -> None: ...

class TraceSrcs(_message.Message):
    __slots__ = ["root_path", "trace_srcs"]
    ROOT_PATH_FIELD_NUMBER: _ClassVar[int]
    TRACE_SRCS_FIELD_NUMBER: _ClassVar[int]
    root_path: str
    trace_srcs: _containers.RepeatedCompositeFieldContainer[TraceSrc]
    def __init__(self, root_path: _Optional[str] = ..., trace_srcs: _Optional[_Iterable[_Union[TraceSrc, _Mapping]]] = ...) -> None: ...
