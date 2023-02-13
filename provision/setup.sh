#! /bin/bash

# get the current path
path=$(dirname -- "$0")

# update local libs
sudo apt-get update

# get curl for docker
sudo apt-get install curl

# install docker
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh ./get-docker.sh

# install docker-compose
sudo curl -L "https://github.com/docker/compose/releases/download/v2.16.0/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose

# Install bazel
sudo $path/install_bazel.sh
