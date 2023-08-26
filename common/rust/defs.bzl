"""
Some helpful rust rules.
"""

load(
    "@rules_rust//rust:defs.bzl",
    _rust_binary = "rust_binary",
    _rust_library = "rust_library",
    _rust_test = "rust_test",
)

def rust_binary(**kwargs):
    deps = kwargs.get("deps", [])
    deps.append("//common/rust:base")
    kwargs["deps"] = deps

    _rust_binary(**kwargs)

def rust_library(**kwargs):
    deps = kwargs.get("deps", [])
    deps.append("//common/rust:base")
    kwargs["deps"] = deps

    _rust_library(**kwargs)

def rust_test(**kwargs):
    deps = kwargs.get("deps", [])
    deps.append("//common/rust:base")
    kwargs["deps"] = deps

    _rust_test(**kwargs)
