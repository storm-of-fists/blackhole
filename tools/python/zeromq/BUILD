load("@rules_python//python:defs.bzl", "py_library", "py_test")

package(default_visibility = ["//visibility:public"])

py_library(
    name = "zeromq",
    srcs = ["zeromq.py"],
    deps = ["@external_py//pyzmq"],
)

py_test(
    name = "test",
    srcs = ["test.py"],
    deps = [":zeromq", "//tools/python/log"],
    size = "small"
)