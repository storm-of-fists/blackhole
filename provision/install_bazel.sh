bazel_target=bazelisk-linux-amd64
bazel_version=1.15.0

wget -O bazel https://github.com/bazelbuild/bazelisk/releases/download/v$bazel_version/$bazel_target
chmod +x bazel
sudo mv bazel /usr/local/bin/
