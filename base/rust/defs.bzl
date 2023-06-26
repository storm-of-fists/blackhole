"""
Some helpful rust rules.
"""

load(
    "@rules_rust//rust:defs.bzl",
    _rust_binary = "rust_binary",
    _rust_library = "rust_library",
)

def rust_binary(**kwargs):
    deps = kwargs.get("deps", [])
    deps.append("//base/rust:base")
    kwargs["deps"] = deps
    # args = ctx.actions.args()
    # args.add("--RUST_LOG=INFO")

    _rust_binary(**kwargs)

def rust_library(**kwargs):
    deps = kwargs.get("deps", [])
    deps.append("//base/rust:base")
    kwargs["deps"] = deps

    _rust_library(**kwargs)
