#! /bin/bash

# get the current path
path=$(dirname -- "$0")

# update local libs
sudo apt-get update

# get curl for docker
sudo apt-get install curl
sudo apt-get install wget

sudo $path/install_docker.sh

sudo $path/install_bazelisk.sh

sudo $path/install_kubernetes.sh

sudo $path/install_certbot.sh

sudo $path/install_npx.sh