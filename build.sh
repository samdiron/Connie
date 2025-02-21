#!/bin/bash


read -p "what to build (All/debug/client/release):(a/d/c/r): " type
if [ $type == "a" ] ; then
  echo "building all now"

  echo "building release"
  cargo build  --release
  echo "done"

  echo "building client"
  cargo build  --profile client
  echo "done"
  
  echo "building debug now"
  cargo build 
  echo "done"
  clear
  echo "size"
  ls -lh ./target/debug/Connie
  ls -lh ./target/client/Connie
  ls -lh ./target/release/Connie
elif [ $type == "r" ] ; then 
  echo "building release"
  cargo build -q --release
  clear
  ls -lh ./target/release/Connie
elif [ $type == "c" ] ; then 
  echo "building client"
  cargo build -q --profile client
  clear
  ls -lh ./target/client/Connie
elif [ $type == "d" ] ; then 
  echo "building debug now"
  cargo builad -q
  clear
  ls -lh ./target/debug/Connie
else 
  echo "please enter a valid argument. "
  exit 1
fi
  

