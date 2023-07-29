from google.protobuf import empty_pb2 as _empty_pb2
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Optional as _Optional

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
