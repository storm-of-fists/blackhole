"""
Some helpful python rules.
"""

load(
    "@rules_python//python:defs.bzl",
    _py_binary = "py_binary",
    _py_library = "py_library",
    _py_test = "py_test",
)

py_library = _py_library
py_binary = _py_binary
py_test = _py_test


