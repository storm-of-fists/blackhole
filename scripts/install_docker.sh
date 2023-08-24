# install docker, includes docker compose cli
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh ./get-docker.sh

# on centos lets open up docker
iptables -t filter -N DOCKER
