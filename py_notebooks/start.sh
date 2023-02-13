#! /bin/bash

# Get the current path
path=$(dirname -- "$0")

# Rebuild the notebook
bazel build //py_notebooks/...

# turn on the server
$path/../bazel-bin/py_notebooks/jupyter
