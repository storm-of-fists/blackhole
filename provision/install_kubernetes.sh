#! /bin/bash

kubernetes_target=linux/amd64/kubectl
kubernetes_version=v1.26.0

curl -LO https://dl.k8s.io/release/$kubernetes_version/bin/$kubernetes_target
curl -LO https://dl.k8s.io/$kubernetes_version/bin/$kubernetes_target.sha256

echo "$(cat kubectl.sha256)  kubectl" | sha256sum --check

sudo install -o root -g root -m 0755 kubectl /usr/local/bin/kubectl

rm kubectl
rm kubectl.sha256