from google.protobuf import empty_pb2 as _empty_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class SourceElement(_message.Message):
    __slots__ = ["fn"]
    FN_FIELD_NUMBER: _ClassVar[int]
    fn: SourceFn
    def __init__(self, fn: _Optional[_Union[SourceFn, _Mapping]] = ...) -> None: ...

class SourceFn(_message.Message):
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

class SourceScan(_message.Message):
    __slots__ = ["coldmod_root_marker_path", "source_elements"]
    COLDMOD_ROOT_MARKER_PATH_FIELD_NUMBER: _ClassVar[int]
    SOURCE_ELEMENTS_FIELD_NUMBER: _ClassVar[int]
    coldmod_root_marker_path: str
    source_elements: _containers.RepeatedCompositeFieldContainer[SourceElement]
    def __init__(self, coldmod_root_marker_path: _Optional[str] = ..., source_elements: _Optional[_Iterable[_Union[SourceElement, _Mapping]]] = ...) -> None: ...
