"""
A all in one system for managing state and increasing visibility
inside of a program.
"""

load("//tools/rust:defs.bzl", "rust_binary")

rust_binary(
    name = "tokio",
    srcs = [
        "state_machine.rs",
    ],
    deps = [
        "//tools/rust/clock",
        "//tools/rust/context",
        "//tools/rust/raw_pointer",
        "@external_rust//:tokio",
    ],
)
