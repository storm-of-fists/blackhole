#! /bin/bash

install_location=/usr/local/bin/bazel
bazelisk_target=bazelisk-linux-amd64
bazelisk_version=1.15.0

wget -O "${install_location}" https://github.com/bazelbuild/bazelisk/releases/download/v$bazelisk_version/$bazelisk_target
chmod +x "${install_location}"
