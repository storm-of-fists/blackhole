INSTALL_LOCATION=/usr/local
TAR_NAME=go_tar.tar.gz

# Clean up anything old
sudo rm -rf "${INSTALL_LOCATION}/go"
sudo rm /usr/local/bin/go
sudo rm -r /tmp/go_install

# Stay in this dir when done
pushd

# Make a temp space for this
cd /tmp
mkdir go_install/
cd go_install/

# Get go, untar, symlink
wget -O "${TAR_NAME}" https://go.dev/dl/go1.21.0.linux-amd64.tar.gz
sudo tar -C "${INSTALL_LOCATION}" -xzf "${TAR_NAME}"
sudo ln -s "${INSTALL_LOCATION}/go/bin/go" /usr/local/bin/

# print some shit, make sure we are good
go version
