# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: coldmod-msg/proto/tracing.proto
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


from google.protobuf import empty_pb2 as google_dot_protobuf_dot_empty__pb2


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1f\x63oldmod-msg/proto/tracing.proto\x12\x19\x63oldmod_msg.proto.tracing\x1a\x1bgoogle/protobuf/empty.proto\">\n\x05Trace\x12\x0e\n\x06\x64igest\x18\x01 \x01(\t\x12\x11\n\tthread_id\x18\x02 \x01(\t\x12\x12\n\nprocess_id\x18\x03 \x01(\t\"\x85\x01\n\x08TraceSrc\x12\x0c\n\x04path\x18\x01 \x01(\t\x12\x0e\n\x06lineno\x18\x02 \x01(\r\x12\x0c\n\x04name\x18\x03 \x01(\t\x12\x1c\n\x0f\x63lass_name_path\x18\x04 \x01(\tH\x00\x88\x01\x01\x12\x0b\n\x03src\x18\x05 \x01(\t\x12\x0e\n\x06\x64igest\x18\x06 \x01(\tB\x12\n\x10_class_name_path\"W\n\tTraceSrcs\x12\x11\n\troot_path\x18\x01 \x01(\t\x12\x37\n\ntrace_srcs\x18\x02 \x03(\x0b\x32#.coldmod_msg.proto.tracing.TraceSrc\"\x1d\n\x0e\x46ilterSetQuery\x12\x0b\n\x03key\x18\x01 \x01(\t\"D\n\tFilterSet\x12\x37\n\ntrace_srcs\x18\x01 \x03(\x0b\x32#.coldmod_msg.proto.tracing.TraceSrc2\xfc\x01\n\x06Traces\x12\x45\n\x07\x63ollect\x12 .coldmod_msg.proto.tracing.Trace\x1a\x16.google.protobuf.Empty(\x01\x12\x43\n\x03set\x12$.coldmod_msg.proto.tracing.TraceSrcs\x1a\x16.google.protobuf.Empty\x12\x66\n\x11stream_filtersets\x12).coldmod_msg.proto.tracing.FilterSetQuery\x1a$.coldmod_msg.proto.tracing.FilterSet0\x01\x62\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'coldmod_msg.proto.tracing_pb2', _globals)
if _descriptor._USE_C_DESCRIPTORS == False:

  DESCRIPTOR._options = None
  _globals['_TRACE']._serialized_start=91
  _globals['_TRACE']._serialized_end=153
  _globals['_TRACESRC']._serialized_start=156
  _globals['_TRACESRC']._serialized_end=289
  _globals['_TRACESRCS']._serialized_start=291
  _globals['_TRACESRCS']._serialized_end=378
  _globals['_FILTERSETQUERY']._serialized_start=380
  _globals['_FILTERSETQUERY']._serialized_end=409
  _globals['_FILTERSET']._serialized_start=411
  _globals['_FILTERSET']._serialized_end=479
  _globals['_TRACES']._serialized_start=482
  _globals['_TRACES']._serialized_end=734
# @@protoc_insertion_point(module_scope)
