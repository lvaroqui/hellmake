#!/usr/bin/sh

cd rust-lib
cargo build
cd ..
mkdir -p build
clang++ -g main.cpp -o build/main

export LD_PRELOAD=/home/luc/lab/rust/dylib/rust-lib/target/debug/libdylib.so 
export LD_LIBRARY_PATH=$(pwd)/build:$LD_LIBRARY_PATH 
./build/main
