load(":defs.bzl", "run_binary")
load("//tools/python:defs.bzl", "py_binary")

py_binary(
    name = "wow",
    srcs = ["wow.py"],
)

run_binary(
    name = "bae",
    run = ":wow"
)