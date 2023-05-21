from google.protobuf import empty_pb2 as _empty_pb2
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Optional as _Optional

DESCRIPTOR: _descriptor.FileDescriptor

class Source(_message.Message):
    __slots__ = ["class_name", "line", "name", "path"]
    CLASS_NAME_FIELD_NUMBER: _ClassVar[int]
    LINE_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    PATH_FIELD_NUMBER: _ClassVar[int]
    class_name: str
    line: int
    name: str
    path: str
    def __init__(self, path: _Optional[str] = ..., line: _Optional[int] = ..., name: _Optional[str] = ..., class_name: _Optional[str] = ...) -> None: ...
