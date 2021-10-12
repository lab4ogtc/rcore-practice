#!/bin/bash

RUST_TARGET=riscv64gc-unknown-none-elf
RUSTSBI_QEMU=../../../rcore-os/rustsbi-qemu/target/riscv64imac-unknown-none-elf/release/rustsbi-qemu.bin

cargo build --target $RUST_TARGET --release
rust-objcopy --binary-architecture=riscv64 target/$RUST_TARGET/release/os --strip-all -O binary target/$RUST_TARGET/release/os.bin
qemu-system-riscv64 -machine virt -nographic -bios $RUSTSBI_QEMU -device loader,file=target/$RUST_TARGET/release/os.bin,addr=0x80200000
