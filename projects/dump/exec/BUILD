load(":defs.bzl", "exec_rule1", "exec_rule2")
load("//common/python:defs.bzl", "py_binary")

exec_rule1(name = "foo")

exec_rule1(name = "bar")

exec_rule2(
    name = "baz",
    deps = [
        ":bar",
        ":foo",
    ],
)

py_binary(
    name = "wow",
    srcs = ["wow.py"],
)


exec_rule2(
    name = "bae",
    deps = [
        ":wow"
    ]
)