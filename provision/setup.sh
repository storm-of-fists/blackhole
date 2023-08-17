#! /bin/bash

# get the current path
path=$(dirname -- "$0")

# update local libs
sudo dnf upgrade

sudo $path/install_docker.sh

sudo $path/install_bazelisk.sh

sudo $path/install_buildifier.sh
