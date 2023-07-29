from google.protobuf import empty_pb2 as _empty_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class HeatMap(_message.Message):
    __slots__ = ["root_path", "heat_srcs"]
    ROOT_PATH_FIELD_NUMBER: _ClassVar[int]
    HEAT_SRCS_FIELD_NUMBER: _ClassVar[int]
    root_path: str
    heat_srcs: _containers.RepeatedCompositeFieldContainer[HeatSrc]
    def __init__(self, root_path: _Optional[str] = ..., heat_srcs: _Optional[_Iterable[_Union[HeatSrc, _Mapping]]] = ...) -> None: ...

class HeatSrc(_message.Message):
    __slots__ = ["path", "line", "name", "class_name"]
    PATH_FIELD_NUMBER: _ClassVar[int]
    LINE_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    CLASS_NAME_FIELD_NUMBER: _ClassVar[int]
    path: str
    line: int
    name: str
    class_name: str
    def __init__(self, path: _Optional[str] = ..., line: _Optional[int] = ..., name: _Optional[str] = ..., class_name: _Optional[str] = ...) -> None: ...

class HeatMapRegister(_message.Message):
    __slots__ = ["default"]
    DEFAULT_FIELD_NUMBER: _ClassVar[int]
    default: bool
    def __init__(self, default: bool = ...) -> None: ...
