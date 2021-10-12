#!/bin/bash

RUST_TARGET=riscv64gc-unknown-none-elf

cargo build --target $RUST_TARGET --release

for e in `find target/$RUST_TARGET/release/ -maxdepth 1 -type f -executable`; do
    echo -e "\n"
    echo "========== Run $(basename $e) =========="
    qemu-riscv64 $e
    echo "========== Finish $(basename $e) =========="
    echo -e "\n"
done
