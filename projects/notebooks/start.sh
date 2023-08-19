#! /bin/bash

# Get the current path
path=$(dirname -- "$0")

# Rebuild the notebook
bazel build //notebooks/...

# turn on the server
$path/../bazel-bin/notebooks/jupyter
