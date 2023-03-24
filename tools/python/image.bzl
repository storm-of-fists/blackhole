load("@io_bazel_rules_docker//python3:image.bzl", "py3_image")
load("@rules_python//python:defs.bzl", "py_binary")

def py_image_combo(
    name,
    deps,
    srcs,
    main = None,
):
    if not main:
        main = "{}.py".format(name)

    py3_image(
        name= "{}.image".format(name),
        main = main,
        srcs = srcs,
        deps = deps,
    )

    py_binary(
        name = name,
        srcs = srcs,
        main = main,
        deps = deps
    )