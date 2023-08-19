#! /bin/bash

# Rebuild the notebook
bazel build :{{ name }}

# turn on the server
./bazel-bin/notebooks/jupyter
