#!/usr/bin/bash
#
# 


echo "Running benchmark"

cargo build --bin matmul --release && ./target/release/matmul $1 $2
