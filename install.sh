#!/bin/bash
# if ! [ $(id -u) = 0 ]; then
#   echo "The script need to be run as root." >&2
#   exit 1
# fi

read -p "what type of connie client is smaller in size but slower server is larger in size but faster NOTE: both can act as a server and client  ENTER S or C : " type
if [ $type == "S" ] || [ $type == "s" ] ; then 
  cargo build --release
 
elif [ $type = "C" ] || [ $type == "c" ]; then 
  cargo build --profile client 
else 
  echo "chose S / C" 
  exit 1

fi

mkdir /opt/Connie
echo "created /Connie/"
mkdir /opt/Connie/metadata
echo "created: /Connie/metadata"
mkdir /opt/Connie/tmp
echo "created: /Connie/tmp/"
mkdir /opt/Connie/logs
echo "created: /Connie/logs"
mkdir /opt/Connie/bin
echo "created: /Connie/bin"
mkdir /opt/Connie/conf
echo "created: /Connie/conf"


# cp ./connie.service /lib/systemd/system/connie.service
# echo "created /lib/systemd/system/connie.service"
cp PATH /opt/Connie/bin/cie

echo "now you can export /opt/Connie/bin/ into your PATH :) "



exit 0
