#!/bin/bash


read -p "what to build (All/debug/client/release/windows):(a/d/c/r/w): " type
if [ $type == "a" ] ; then
  echo "building all now"

  echo "building release"
  cargo build  --release
  echo "done"
  
  echo "windows release "
  cargo build --target=x86_64-pc-windows-gnu --release
  echo "done"
  
  echo "building client"
  cargo build  --profile client
  echo "done"
  
  echo "building debug now"
  cargo build 
  echo "done"
  clear
  echo "size"
  ls -lh ./target/debug/cie
  ls -lh ./target/client/cie
  ls -lh ./target/release/cie
  ls -lh ./target/x86_64-pc-windows-gnu/release/cie.exe
elif [ $type == "r" ] ; then 
  echo "building release"
  cargo build --release
  clear
  ls -lh ./target/release/cie
elif [ $type == "c" ] ; then 
  echo "building client"
  cargo build  --profile client
  clear
  ls -lh ./target/client/cie
elif [ $type == "d" ] ; then 
  echo "building debug now"
  cargo build 
  clear
  ls -lh ./target/debug/cie
elif [ $type == "w" ] ; then 
  echo "building windows now"
  cargo build --target=x86_64-pc-windows-gnu --release
  clear
  ls -lh ./target/x86_64-pc-windows-gnu/release/cie.exe
else 
  echo "please enter a valid argument. "
  exit 1
fi
  

