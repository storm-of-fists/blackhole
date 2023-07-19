"""
Some helpful python rules.
"""

load(
    "//base/python:defs.bzl",
    _py_library="py_library",
    _py_binary="py_binary",
    _py_notebook="py_notebook",
    _py_image="py_image"
)

load(
    "//base/rust:defs.bzl",
    _rust_library = "rust_library",
    _rust_binary = "rust_binary"
)

py_library = _py_library
py_binary = _py_binary

rust_library = _rust_library
rust_binary = _rust_binary
py_notebook = _py_notebook
py_image = _py_image