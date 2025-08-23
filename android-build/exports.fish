#!/bin/fish
set ANDROID_HOME ~/Android/Sdk



set CARGO_NDK_SYSROOT_PATH $ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/sysroot
set CARGO_NDK_ANDROID_PLATFORM 33
set BINDGEN_EXTRA_CLANG_ARGS "--sysroot=$ANDROID_NDK_HOME/ndk/toolchains/llvm/prebuilt/linux-x86_64/sysroot" 
# my ndk version is 29.0.13599879 yours could be another version 
# set ANDROID_NDK_HOME $ANDROID_HOME/ndk/29.0.13599879
set NDK_HOME $ANDROID_HOME/ndk
set NDK_PROJECT_PATH  ~/Connie
set ANDROID_NDK_ROOT $ANDROID_HOME/ndk
set ANDROID_NDK_HOME $ANDROID_HOME/ndk
