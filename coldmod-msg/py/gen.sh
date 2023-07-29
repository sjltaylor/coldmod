python -m grpc_tools.protoc -I../../ --python_out . --pyi_out=. --grpc_python_out=. coldmod-msg/proto/trace.proto coldmod-msg/proto/heat.proto
