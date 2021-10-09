#!/bin/bash

cargo build --target riscv64gc-unknown-none-elf
qemu-riscv64 target/riscv64gc-unknown-none-elf/debug/os; echo $?
