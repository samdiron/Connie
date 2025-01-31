#!/bin/bash
# if ! [ $(id -u) = 0 ]; then
#   echo "The script need to be run as root." >&2
#   exit 1
# fi

read -p "what type of connie client is smaller in size but slower server is larger in size but faster NOTE: both can act as a server and client  ENTER S or C : " type
if [ $type == "S" ] || [ $type == "s" ] ; then 
  cargo build --release
  set PATH = ./target/release/Connie 
elif [ $type = "C" ] || [ $type == "c" ]; then 
  cargo build --profile client
  set PATH = ./target/client/Connie
else 
  echo "chose S / C" 
  exit 1

fi

read -p "Hey, can i use sudo to create my home(y/n): " confirm
if [ $confirm == "y" ] || [ $confirm == "Y" ] || [ $confirm == "yes" ] ; then 
  echo "Thanks"
else
  echo "will have to exit :("
  exit 1
fi
sudo mkdir /opt/Connie
echo "created opt/Connie/"

sudo mkdir /opt/Connie/metadata
echo "created: /opt/Connie/metadata"

sudo mkdir /opt/Connie/tmp
echo "created: /opt/Connie/tmp/"

sudo mkdir /opt/Connie/logs
echo "created: /opt/Connie/logs"

sudo mkdir /opt/Connie/bin
echo "created: /opt/Connie/bin"

sudo mkdir /opt/Connie/conf
echo "created: /opt/Connie/conf"


# cp ./connie.service /lib/systemd/system/connie.service
# echo "created /lib/systemd/system/connie.service"

if [ $type == "S" ] || [ $type == "s" ] ; then 
  sudo cp ./target/client/Connie /opt/Connie/bin/cie
else
  sudo cp ./target/client/Connie /opt/Connie/bin/cie  
fi

echo "now you can export /opt/Connie/bin/ into your PATH :) "



exit 0
