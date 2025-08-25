#!/bin/bash

cd crates/builder
cargo build
cd - > /dev/null

cd thirdparty
clang -shared -fPIC -o ../target/debug/libthirdparty.so lib.c -ldl -lpthread -lm
cd - > /dev/null
