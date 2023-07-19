"""
Some helpful python rules.
"""

load(
    "@rules_python//python:defs.bzl", 
    _py_library="py_library", 
    _py_binary="py_binary"
)
load("@third_party_python//:requirements.bzl", "requirement")
load("@io_bazel_rules_docker//python3:image.bzl", _py3_image="py3_image")

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

def py_notebook(name, deps):
    py_binary(
        name = name,
        srcs = ["{}.py".format(name)],
        deps = deps,
        tags = ["manual"],
        reqs = [
            "notebook",
            "numpy",
            "matplotlib",
            "jplephem",
            "astropy",
            "PyYAML",
        ],
    )