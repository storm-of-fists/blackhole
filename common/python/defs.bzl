"""
Some helpful python rules.
"""

load(
    "@rules_python//python:defs.bzl",
    _py_binary = "py_binary",
    _py_library = "py_library",
    _py_test = "py_test",
)
load("@third_party_py//:requirements.bzl", "requirement")
# load("@io_bazel_rules_docker//python3:image.bzl", _py3_image = "py3_image")
load("//third_party/python:requirements.bzl", "PY_DEPS")

def py_binary(**kwargs):
    deps = kwargs.get("deps", [])
    deps.append("//common/python:base")

    reqs = kwargs.get("reqs", [])
    if reqs:
        for req in reqs:
            deps.append(requirement(req))
        kwargs.pop("reqs")

    kwargs["deps"] = deps

    _py_binary(**kwargs)

def py_library(**kwargs):
    deps = kwargs.get("deps", [])
    deps.append("//common/python:base")

    reqs = kwargs.get("reqs", [])
    if reqs:
        for req in reqs:
            deps.append(requirement(req))
        kwargs.pop("reqs")

    kwargs["deps"] = deps

    _py_library(**kwargs)

# def py_image(**kwargs):
#     deps = kwargs.get("deps", [])
#     deps.append("//common/python:base")

#     reqs = kwargs.get("reqs", [])
#     if reqs:
#         for req in reqs:
#             deps.append(requirement(req))
#         kwargs.pop("reqs")

#     kwargs["deps"] = deps

#     _py3_image(**kwargs)

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
        reqs = [
            dep
            for dep in list(PY_DEPS.keys())
            if not PY_DEPS[dep].get("no_notebooks")
        ],
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

def py_test(**kwargs):
    _py_test(**kwargs)
