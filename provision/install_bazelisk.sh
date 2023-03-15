#! /bin/bash

bazelisk_target=bazelisk-linux-amd64
bazelisk_version=1.15.0

wget -O bazel https://github.com/bazelbuild/bazelisk/releases/download/v$bazelisk_version/$bazelisk_target
chmod +x bazel
sudo mv bazel /usr/local/bin/
