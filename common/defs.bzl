"""
Some helpful python rules.
"""

load(
    "//common/python:defs.bzl",
    _py_binary = "py_binary",
    # _py_image = "py_image",
    _py_library = "py_library",
    _py_notebook = "py_notebook",
    _py_test = "py_test",
)
load(
    "//common/rust:defs.bzl",
    _rust_binary = "rust_binary",
    _rust_library = "rust_library",
    _rust_test = "rust_test",
)
load(
    "//common/proto:defs.bzl",
    _proto_library = "proto_library",
    _python_proto_library = "python_proto_library",
)

### PYTHON ###
py_library = _py_library
py_notebook = _py_notebook

### RUST ###
rust_library = _rust_library
rust_binary = _rust_binary
rust_test = _rust_test

### PROTO ###
proto_library = _proto_library
python_proto_library = _python_proto_library
