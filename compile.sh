#!/bin/bash
# Minimal bash script to compile lib.c to shared object
echo "Compiling lib.c to SO..."

# Build builder crate  
cd crates/builder
cargo build
cd - > /dev/null

# Compile C code to shared object
cd thirdparty
clang -shared -fPIC -o ../target/debug/libthirdparty.so lib.c -ldl -lpthread -lm
cd - > /dev/null
