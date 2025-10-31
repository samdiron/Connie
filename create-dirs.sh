#!/bin/bash


sudo mkdir /opt/Connie
echo "created opt/Connie/"
chown -c  -R $USER /opt/Connie
mkdir /opt/Connie/metadata
echo "created: /opt/Connie/metadata"

mkdir /opt/Connie/tmp
echo "created: /opt/Connie/tmp/"

mkdir /opt/Connie/logs
echo "created: /opt/Connie/logs"

mkdir /opt/Connie/bin
echo "created: /opt/Connie/bin"

mkdir /opt/Connie/conf
echo "created: /opt/Connie/conf"

mkdir /opt/Connie/conf/certs/

cp openssl_script.sh /opt/Connie/bin/
chmod +x /opt/Connie/bin/openssl_script.sh

sudo touch /opt/Connie/conf/.connieDB.sqlite
sudo touch /opt/Connie/conf/server_ident.toml
sudo touch /opt/Connie/conf/certs/key.pem
sudo touch /opt/Connie/conf/certs/cert.pem

