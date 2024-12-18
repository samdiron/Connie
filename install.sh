#!/bin/bash

# ref: https://askubuntu.com/a/30157/8698
if ! [ $(id -u) = 0 ]; then
  echo "The script need to be run as root." >&2
  exit 1
fi

mkdir /Connie
echo "created /Connie/"
mkdir /Connie/metadata
echo "created: /Connie/metadata"
mkdir /Connie/tmp
echo "created: /Connie/tmp/"
mkdir /Connie/.config
echo "created: /Connie/.config/"
mkdir /Connie/logs
echo "created: /Connie/logs"
mkdir /Connie/bin
echo "created: /Connie/bin"
mkdir /Connie/bin/uninstall

# cp ./connie.service /lib/systemd/system/connie.service
# echo "created /lib/systemd/system/connie.service"
cp ./target/debug/Connie /Connie/bin/connie.bin
