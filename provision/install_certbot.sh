#! /bin/bash

sudo snap install core; sudo snap refresh core

sudo apt-get remove certbot

sudo snap install --classic certbot

# adds to our path to call certbot anywhere
sudo ln -s /snap/bin/certbot /usr/bin/certbot

read -p "What is the domain name we are getting certs for? " domain_name

sudo certbot certonly --standalone -d $domain_name