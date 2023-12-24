load("@rules_proto//proto:defs.bzl", _proto_library = "proto_library")
load("@grpc//bazel:python_rules.bzl", _python_proto_library = "py_proto_library")
# load("@grpc//bazel:rust_rules.bzl", _rust_proto_library = "rust_proto_library")

proto_library = _proto_library
python_proto_library = _python_proto_library

# def rust_proto_library(**kwargs):
#     _rust_proto_library(**kwargs)
