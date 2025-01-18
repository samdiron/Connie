#!/bin/bash


read -p "Ewhat type of connie \n client is smaller in size but slower \n server is larger in size but faster \n NOTE: both can act as a server and client \n ENTER S or C : " type
if [ $type == "S" ] || [ $type == "s" ] ; then 
  cargo build --release
 
elif [ $type = "C" ] || [ $type == "c" ]; then 
  cargo build --profile client 
else 
  echo "chose S / C" 
  exit 1

fi
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


# cp ./connie.service /lib/systemd/system/connie.service
# echo "created /lib/systemd/system/connie.service"
cp ./target/debug/Connie /Connie/bin/connie
if [(echo $SHELL) = "/usr/bin/fish" ]; then 
  fish_add_path  -g /Connie/bin/
  echo "you can try connie now "
  exit 1
fi
