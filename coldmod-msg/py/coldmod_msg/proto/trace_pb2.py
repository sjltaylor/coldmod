# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: coldmod-msg/proto/trace.proto
"""Generated protocol buffer code."""
from google.protobuf.internal import builder as _builder
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()


from google.protobuf import empty_pb2 as google_dot_protobuf_dot_empty__pb2


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1d\x63oldmod-msg/proto/trace.proto\x12\x11\x63oldmod_msg.proto\x1a\x1bgoogle/protobuf/empty.proto\"J\n\x05Trace\x12\x0c\n\x04path\x18\x01 \x01(\t\x12\x0c\n\x04line\x18\x02 \x01(\r\x12\x11\n\tthread_id\x18\x03 \x01(\x04\x12\x12\n\nprocess_id\x18\x04 \x01(\x04\x32N\n\rTracingDaemon\x12=\n\x07\x63ollect\x12\x18.coldmod_msg.proto.Trace\x1a\x16.google.protobuf.Empty(\x01\x62\x06proto3')

_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, globals())
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'coldmod_msg.proto.trace_pb2', globals())
if _descriptor._USE_C_DESCRIPTORS == False:

  DESCRIPTOR._options = None
  _TRACE._serialized_start=81
  _TRACE._serialized_end=155
  _TRACINGDAEMON._serialized_start=157
  _TRACINGDAEMON._serialized_end=235
# @@protoc_insertion_point(module_scope)
