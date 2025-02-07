#!/bin/bash


read -p "what to build (All/debug/client/release):(a/b/c/r): " type
if [ $type == "a" ] ; then
  echo "building all now"

  echo "building release"
  cargo build  --release
  echo "release"
  ls -lh ./target/release/Connie

  echo "building client"
  cargo build  --profile client
  echo "client"
  ls -lh ./target/client/Connie
  
  echo "building debug now"
  cargo build 
  echo "debug"
  ls -lh ./target/debug/Connie

elif [ $type == "r" ] ; then 
  echo "building release"
  cargo build -q --release
  ls -lh ./target/release/Connie
elif [ $type == "c" ] ; then 
  echo "building client"
  cargo build -q --profile client
  ls -lh ./target/client/Connie
elif [ $type == "d" ] ; then 
  echo "building debug now"
  cargo builad -q
  ls -lh ./target/debug/Connie
else 
  echo "enter a valid arg bitch"
  exit 1
fi
  

