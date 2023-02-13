#! /bin/bash

# get the current path
path=$(dirname -- "$0")

# update local libs
sudo apt-get update

# get curl for docker
sudo apt-get install curl

# install docker, includes docker compose cli
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh ./get-docker.sh

# Install bazel
sudo $path/install_bazel.sh
