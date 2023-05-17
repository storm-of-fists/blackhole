"""
Some helpful rust rules.
"""

load(
    "@rules_rust//rust:defs.bzl",
    _rust_binary = "rust_binary",
    _rust_library = "rust_library",
)

package(default_visibility = ["//visibility:public"])

def rust_binary(**kwargs):
    deps = kwargs.get("deps", [])
    deps.append("//base/rust:base")
    kwargs["deps"] = deps

    _rust_binary(**kwargs)

def rust_library(**kwargs):
    deps = kwargs.get("deps", [])
    deps.append("//base/rust:base")
    kwargs["deps"] = deps

    _rust_library(**kwargs)
