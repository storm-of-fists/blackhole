#! /bin/bash

# Rebuild the notebook
bazel build //notebooks/...

# turn on the server
./bazel-bin/notebooks/jupyter
