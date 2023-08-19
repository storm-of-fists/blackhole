"""
Some helpful python rules.
"""

load(
    "@rules_python//python:defs.bzl",
    _py_binary = "py_binary",
    _py_library = "py_library",
)
load("@third_party_python//:requirements.bzl", "requirement")
load("@io_bazel_rules_docker//python3:image.bzl", _py3_image = "py3_image")
load("//third_party/python:requirements.bzl", "PY_DEPS")

def py_binary(**kwargs):
    deps = kwargs.get("deps", [])
    deps.append("//base/python:base")

    reqs = kwargs.get("reqs", [])
    if reqs:
        for req in reqs:
            deps.append(requirement(req))
        kwargs.pop("reqs")

    kwargs["deps"] = deps

    _py_binary(**kwargs)

def py_library(**kwargs):
    deps = kwargs.get("deps", [])
    deps.append("//base/python:base")

    reqs = kwargs.get("reqs", [])
    if reqs:
        for req in reqs:
            deps.append(requirement(req))
        kwargs.pop("reqs")

    kwargs["deps"] = deps

    _py_library(**kwargs)

def py_image(**kwargs):
    deps = kwargs.get("deps", [])
    deps.append("//base/python:base")

    reqs = kwargs.get("reqs", [])
    if reqs:
        for req in reqs:
            deps.append(requirement(req))
        kwargs.pop("reqs")

    kwargs["deps"] = deps

    _py3_image(**kwargs)

def _py_notebook_runner_impl(ctx):
    bin = ctx.attr.py_bin
    bin_file = [file for file in bin.files.to_list() if "py_binary" in file.basename].pop()

    ctx.actions.run_shell(
        inputs = bin.files,
        outputs = [ctx.outputs.executable],
        command = "{} --ip=0.0.0.0 --port=25252 --NotebookApp.token='' --NotebookApp.password=''".format(bin_file.path),
        execution_requirements = {
            "no-sandbox": "True",
            "no-cache": "True",
            "no-remote": "True",
            "local": "True",
            "requires-network": "True",
            "manual": "True",
        },
    )

py_notebook_runner = rule(
    implementation = _py_notebook_runner_impl,
    executable = True,
    attrs = {
        "py_bin": attr.label(),
    },
)

def py_notebook(name, deps):
    py_binary(
        name = "{}.py_binary".format(name),
        srcs = ["//base/python:jupyter.py"],
        main = "//base/python:jupyter.py",
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
    )
