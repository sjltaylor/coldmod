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


DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x1f\x63oldmod-msg/proto/tracing.proto\x12\x19\x63oldmod_msg.proto.tracing\x1a\x1bgoogle/protobuf/empty.proto\";\n\x05Trace\x12\x0b\n\x03key\x18\x01 \x01(\t\x12\x11\n\tthread_id\x18\x02 \x01(\t\x12\x12\n\nprocess_id\x18\x03 \x01(\t\"\x17\n\x08TraceSrc\x12\x0b\n\x03key\x18\x01 \x01(\t\"D\n\tTraceSrcs\x12\x37\n\ntrace_srcs\x18\x02 \x03(\x0b\x32#.coldmod_msg.proto.tracing.TraceSrc\"\x1a\n\x0bOpenCommand\x12\x0b\n\x03key\x18\x01 \x01(\t\"\x1c\n\rRemoveCommand\x12\x0b\n\x03key\x18\x01 \x01(\t\"\x1c\n\rIgnoreCommand\x12\x0b\n\x03key\x18\x01 \x01(\t\"\r\n\x0bSendSrcInfo\"\x88\x02\n\nModCommand\x12?\n\rsend_src_info\x18\x01 \x01(\x0b\x32&.coldmod_msg.proto.tracing.SendSrcInfoH\x00\x12:\n\x06ignore\x18\x02 \x01(\x0b\x32(.coldmod_msg.proto.tracing.IgnoreCommandH\x00\x12:\n\x06remove\x18\x03 \x01(\x0b\x32(.coldmod_msg.proto.tracing.RemoveCommandH\x00\x12\x36\n\x04open\x18\x04 \x01(\x0b\x32&.coldmod_msg.proto.tracing.OpenCommandH\x00\x42\t\n\x07\x63ommand\"\x19\n\nConnectKey\x12\x0b\n\x03key\x18\x01 \x01(\t\"\x18\n\tSrcIgnore\x12\x0b\n\x03key\x18\x01 \x01(\t\"\x1c\n\x0cSrcAvailable\x12\x0c\n\x04keys\x18\x01 \x03(\t\"%\n\x07SrcRefs\x12\x0b\n\x03key\x18\x01 \x01(\t\x12\r\n\x05\x63ount\x18\x02 \x01(\r\"/\n\x0fSrcRemoveResult\x12\x0b\n\x03key\x18\x01 \x01(\t\x12\x0f\n\x07success\x18\x02 \x01(\x08\"\xe1\x02\n\nSrcMessage\x12<\n\x0b\x63onnect_key\x18\x01 \x01(\x0b\x32%.coldmod_msg.proto.tracing.ConnectKeyH\x00\x12:\n\nsrc_ignore\x18\x02 \x01(\x0b\x32$.coldmod_msg.proto.tracing.SrcIgnoreH\x00\x12@\n\rsrc_available\x18\x03 \x01(\x0b\x32\'.coldmod_msg.proto.tracing.SrcAvailableH\x00\x12\x36\n\x08src_refs\x18\x04 \x01(\x0b\x32\".coldmod_msg.proto.tracing.SrcRefsH\x00\x12G\n\x11src_remove_result\x18\x05 \x01(\x0b\x32*.coldmod_msg.proto.tracing.SrcRemoveResultH\x00\x42\x16\n\x14possible_src_message\"\x1b\n\x0c\x46\x65tchOptions\x12\x0b\n\x03\x61ll\x18\x01 \x01(\x08\"+\n\x07HeatSrc\x12\x0b\n\x03key\x18\x01 \x01(\t\x12\x13\n\x0btrace_count\x18\x02 \x01(\x03\";\n\x07HeatMap\x12\x30\n\x04srcs\x18\x01 \x03(\x0b\x32\".coldmod_msg.proto.tracing.HeatSrc2\xc3\x02\n\x06Traces\x12\x45\n\x07\x63ollect\x12 .coldmod_msg.proto.tracing.Trace\x1a\x16.google.protobuf.Empty(\x01\x12\x43\n\x03set\x12$.coldmod_msg.proto.tracing.TraceSrcs\x1a\x16.google.protobuf.Empty\x12W\n\x03mod\x12%.coldmod_msg.proto.tracing.SrcMessage\x1a%.coldmod_msg.proto.tracing.ModCommand(\x01\x30\x01\x12T\n\x05\x66\x65tch\x12\'.coldmod_msg.proto.tracing.FetchOptions\x1a\".coldmod_msg.proto.tracing.HeatMapb\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'coldmod_msg.proto.tracing_pb2', _globals)
if _descriptor._USE_C_DESCRIPTORS == False:

  DESCRIPTOR._options = None
  _globals['_TRACE']._serialized_start=91
  _globals['_TRACE']._serialized_end=150
  _globals['_TRACESRC']._serialized_start=152
  _globals['_TRACESRC']._serialized_end=175
  _globals['_TRACESRCS']._serialized_start=177
  _globals['_TRACESRCS']._serialized_end=245
  _globals['_OPENCOMMAND']._serialized_start=247
  _globals['_OPENCOMMAND']._serialized_end=273
  _globals['_REMOVECOMMAND']._serialized_start=275
  _globals['_REMOVECOMMAND']._serialized_end=303
  _globals['_IGNORECOMMAND']._serialized_start=305
  _globals['_IGNORECOMMAND']._serialized_end=333
  _globals['_SENDSRCINFO']._serialized_start=335
  _globals['_SENDSRCINFO']._serialized_end=348
  _globals['_MODCOMMAND']._serialized_start=351
  _globals['_MODCOMMAND']._serialized_end=615
  _globals['_CONNECTKEY']._serialized_start=617
  _globals['_CONNECTKEY']._serialized_end=642
  _globals['_SRCIGNORE']._serialized_start=644
  _globals['_SRCIGNORE']._serialized_end=668
  _globals['_SRCAVAILABLE']._serialized_start=670
  _globals['_SRCAVAILABLE']._serialized_end=698
  _globals['_SRCREFS']._serialized_start=700
  _globals['_SRCREFS']._serialized_end=737
  _globals['_SRCREMOVERESULT']._serialized_start=739
  _globals['_SRCREMOVERESULT']._serialized_end=786
  _globals['_SRCMESSAGE']._serialized_start=789
  _globals['_SRCMESSAGE']._serialized_end=1142
  _globals['_FETCHOPTIONS']._serialized_start=1144
  _globals['_FETCHOPTIONS']._serialized_end=1171
  _globals['_HEATSRC']._serialized_start=1173
  _globals['_HEATSRC']._serialized_end=1216
  _globals['_HEATMAP']._serialized_start=1218
  _globals['_HEATMAP']._serialized_end=1277
  _globals['_TRACES']._serialized_start=1280
  _globals['_TRACES']._serialized_end=1603
# @@protoc_insertion_point(module_scope)
