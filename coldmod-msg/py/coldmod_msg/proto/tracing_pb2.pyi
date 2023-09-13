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
    __slots__ = ["send_src_info", "ignore", "remove"]
    SEND_SRC_INFO_FIELD_NUMBER: _ClassVar[int]
    IGNORE_FIELD_NUMBER: _ClassVar[int]
    REMOVE_FIELD_NUMBER: _ClassVar[int]
    send_src_info: SendSrcInfo
    ignore: IgnoreCommand
    remove: RemoveCommand
    def __init__(self, send_src_info: _Optional[_Union[SendSrcInfo, _Mapping]] = ..., ignore: _Optional[_Union[IgnoreCommand, _Mapping]] = ..., remove: _Optional[_Union[RemoveCommand, _Mapping]] = ...) -> None: ...

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
    __slots__ = ["key"]
    KEY_FIELD_NUMBER: _ClassVar[int]
    key: str
    def __init__(self, key: _Optional[str] = ...) -> None: ...

class SrcMessage(_message.Message):
    __slots__ = ["connect_key", "src_ignore", "src_available"]
    CONNECT_KEY_FIELD_NUMBER: _ClassVar[int]
    SRC_IGNORE_FIELD_NUMBER: _ClassVar[int]
    SRC_AVAILABLE_FIELD_NUMBER: _ClassVar[int]
    connect_key: ConnectKey
    src_ignore: SrcIgnore
    src_available: SrcAvailable
    def __init__(self, connect_key: _Optional[_Union[ConnectKey, _Mapping]] = ..., src_ignore: _Optional[_Union[SrcIgnore, _Mapping]] = ..., src_available: _Optional[_Union[SrcAvailable, _Mapping]] = ...) -> None: ...
