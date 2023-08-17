INSTALL_LOCATION=/usr/local/bin/buildifier

sudo rm "${INSTALL_LOCATION}"
sudo wget -O "${INSTALL_LOCATION}" https://github.com/bazelbuild/buildtools/releases/download/v6.1.2/buildifier-linux-amd64
sudo chmod +x "${INSTALL_LOCATION}"
