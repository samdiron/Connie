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
elif [ $type = "dev" ] || [ $type == "DEV" ]; then 
  cargo build -vv 
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

./create-dirs.sh


read -p "do you want to install connie.service file to start with systemctl or systemd(y/n): " service
if [ $service == "y" ] || [ $service == "Y" ] || [ $service == "yes"] ; then 
  sudo cp ./connie.service /etc/systemd/system/
else 
  echo "will not create a connie.service file ";

if [ $type == "S" ] || [ $type == "s" ] ; then 
  cp ./target/client/Connie /opt/Connie/bin/cie
  chmod +x /opt/Connie/bin/cie
elif [ $type = "dev" ] || [ $type == "DEV" ]; then 
  rm /opt/Connie/bin/cie
  cp ./target/debug/Connie /opt/Connie/bin/cie-debug
  chmod +x /opt/Connie/bin/cie-debug
else
  cp ./target/client/Connie /opt/Connie/bin/cie  
  chmod +x /opt/Connie/bin/cie
fi

echo "now you can export /opt/Connie/bin/ into your PATH :) "



exit 0
