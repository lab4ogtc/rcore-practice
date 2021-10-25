#!/bin/bash

RUST_TARGET=riscv64gc-unknown-none-elf

cargo build --target $RUST_TARGET --release

find target/$RUST_TARGET/release/ -maxdepth 1 -type f -executable -exec sh -c '
    echo
    echo "========== Run $(basename $1) =========="
    qemu-riscv64 $1
    echo "========== Finish $(basename $1) =========="
    echo
' sh {} \;

find target/$RUST_TARGET/release/ -maxdepth 1 -type f -executable -exec sh -c '
    rust-objcopy -O binary $1 $1.bin
    chmod -x $1.bin
' sh {} \;