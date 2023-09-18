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

class OpenCommand(_message.Message):
    __slots__ = ["key"]
    KEY_FIELD_NUMBER: _ClassVar[int]
    key: str
    def __init__(self, key: _Optional[str] = ...) -> None: ...

class RemoveCommand(_message.Message):
    __slots__ = ["key"]
    KEY_FIELD_NUMBER: _ClassVar[int]
    key: str
    def __init__(self, key: _Optional[str] = ...) -> None: ...

class IgnoreCommand(_message.Message):
    __slots__ = ["key"]
    KEY_FIELD_NUMBER: _ClassVar[int]
    key: str
    def __init__(self, key: _Optional[str] = ...) -> None: ...

class SendSrcInfo(_message.Message):
    __slots__ = []
    def __init__(self) -> None: ...

class ModCommand(_message.Message):
    __slots__ = ["send_src_info", "ignore", "remove", "open"]
    SEND_SRC_INFO_FIELD_NUMBER: _ClassVar[int]
    IGNORE_FIELD_NUMBER: _ClassVar[int]
    REMOVE_FIELD_NUMBER: _ClassVar[int]
    OPEN_FIELD_NUMBER: _ClassVar[int]
    send_src_info: SendSrcInfo
    ignore: IgnoreCommand
    remove: RemoveCommand
    open: OpenCommand
    def __init__(self, send_src_info: _Optional[_Union[SendSrcInfo, _Mapping]] = ..., ignore: _Optional[_Union[IgnoreCommand, _Mapping]] = ..., remove: _Optional[_Union[RemoveCommand, _Mapping]] = ..., open: _Optional[_Union[OpenCommand, _Mapping]] = ...) -> None: ...

class ConnectKey(_message.Message):
    __slots__ = ["key"]
    KEY_FIELD_NUMBER: _ClassVar[int]
    key: str
    def __init__(self, key: _Optional[str] = ...) -> None: ...

class SrcIgnore(_message.Message):
    __slots__ = ["key"]
    KEY_FIELD_NUMBER: _ClassVar[int]
    key: str
    def __init__(self, key: _Optional[str] = ...) -> None: ...

class SrcAvailable(_message.Message):
    __slots__ = ["keys"]
    KEYS_FIELD_NUMBER: _ClassVar[int]
    keys: _containers.RepeatedScalarFieldContainer[str]
    def __init__(self, keys: _Optional[_Iterable[str]] = ...) -> None: ...

class SrcRefs(_message.Message):
    __slots__ = ["key", "count"]
    KEY_FIELD_NUMBER: _ClassVar[int]
    COUNT_FIELD_NUMBER: _ClassVar[int]
    key: str
    count: int
    def __init__(self, key: _Optional[str] = ..., count: _Optional[int] = ...) -> None: ...

class SrcRemoveResult(_message.Message):
    __slots__ = ["key", "success"]
    KEY_FIELD_NUMBER: _ClassVar[int]
    SUCCESS_FIELD_NUMBER: _ClassVar[int]
    key: str
    success: bool
    def __init__(self, key: _Optional[str] = ..., success: bool = ...) -> None: ...

class SrcMessage(_message.Message):
    __slots__ = ["connect_key", "src_ignore", "src_available", "src_refs", "src_remove_result"]
    CONNECT_KEY_FIELD_NUMBER: _ClassVar[int]
    SRC_IGNORE_FIELD_NUMBER: _ClassVar[int]
    SRC_AVAILABLE_FIELD_NUMBER: _ClassVar[int]
    SRC_REFS_FIELD_NUMBER: _ClassVar[int]
    SRC_REMOVE_RESULT_FIELD_NUMBER: _ClassVar[int]
    connect_key: ConnectKey
    src_ignore: SrcIgnore
    src_available: SrcAvailable
    src_refs: SrcRefs
    src_remove_result: SrcRemoveResult
    def __init__(self, connect_key: _Optional[_Union[ConnectKey, _Mapping]] = ..., src_ignore: _Optional[_Union[SrcIgnore, _Mapping]] = ..., src_available: _Optional[_Union[SrcAvailable, _Mapping]] = ..., src_refs: _Optional[_Union[SrcRefs, _Mapping]] = ..., src_remove_result: _Optional[_Union[SrcRemoveResult, _Mapping]] = ...) -> None: ...

class FetchOptions(_message.Message):
    __slots__ = ["all"]
    ALL_FIELD_NUMBER: _ClassVar[int]
    all: bool
    def __init__(self, all: bool = ...) -> None: ...

class HeatSrc(_message.Message):
    __slots__ = ["key", "trace_count"]
    KEY_FIELD_NUMBER: _ClassVar[int]
    TRACE_COUNT_FIELD_NUMBER: _ClassVar[int]
    key: str
    trace_count: int
    def __init__(self, key: _Optional[str] = ..., trace_count: _Optional[int] = ...) -> None: ...

class HeatMap(_message.Message):
    __slots__ = ["srcs"]
    SRCS_FIELD_NUMBER: _ClassVar[int]
    srcs: _containers.RepeatedCompositeFieldContainer[HeatSrc]
    def __init__(self, srcs: _Optional[_Iterable[_Union[HeatSrc, _Mapping]]] = ...) -> None: ...
