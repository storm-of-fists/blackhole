#! /bin/bash
bazel build //py_notebooks/...
path=$(dirname -- "$0")
$path/../bazel-bin/py_notebooks/jupyter
