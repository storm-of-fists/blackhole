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

def _py_notebook_runner_impl(ctx):
    bin = ctx.attr.py_bin

    ctx.actions.run(
        inputs = bin.files,
        outputs = [ctx.outputs.executable],
        # command = "{} --ip=0.0.0.0 --port=25252 --NotebookApp.token='' --NotebookApp.password=''".format(bin_file.path),
        executable = ctx.executable.py_bin,
    )

py_notebook_runner = rule(
    implementation = _py_notebook_runner_impl,
    executable = True,
    attrs = {
        "py_bin": attr.label(
            executable = True,
            cfg = "exec",
        ),
    },
)

def py_notebook(name, deps):
    py_binary(
        name = "{}.py_binary".format(name),
        srcs = ["//common/python:jupyter.py"],
        main = "//common/python:jupyter.py",
        deps = deps,
        tags = ["manual"],
    )

    py_notebook_runner(
        name = name,
        py_bin = ":{}.py_binary".format(name),
        tags = [
            "manual",
            "no-sandbox",
            "no-cache",
            "no-remote",
            "local",
            "requires-network",
        ],
    )
