"""
Some helpful python rules.
"""

load(
    "@rules_python//python:defs.bzl", 
    _py_library="py_library", 
    _py_binary="py_binary"
)

def py_binary(**kwargs):
    deps = kwargs.get("deps", [])
    deps.append("//base/python:base")
    kwargs["deps"] = deps

    _py_binary(**kwargs)

def py_library(**kwargs):
    deps = kwargs.get("deps", [])
    deps.append("//base/python:base")
    kwargs["deps"] = deps

    _py_library(**kwargs)
