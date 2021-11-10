#!/bin/bash

RUST_TARGET=riscv64gc-unknown-none-elf
LINK_SCRIPT="src/linker.ld"

BASE_ADDRESS=0x80400000
STEP=0x20000

APP_BASE_ADDRESS=$BASE_ADDRESS
for f in src/bin/*.rs; do
  app=$(basename -s .rs "$f")
  sed -i "/BASE_ADDRESS/{s/0x[0-9]\+/$APP_BASE_ADDRESS/}" $LINK_SCRIPT
  APP_BASE_ADDRESS=$(printf "%#x" $((APP_BASE_ADDRESS + STEP)))

  cargo build --target $RUST_TARGET --bin "$app" --release
done
sed -i "/BASE_ADDRESS/{s/0x[0-9]\+/$BASE_ADDRESS/}" $LINK_SCRIPT

find target/$RUST_TARGET/release/ -maxdepth 1 -type f -executable -exec sh -c '
    rust-objcopy -O binary $1 $1.bin
    chmod -x $1.bin
' sh {} \;

#find target/$RUST_TARGET/release/ -maxdepth 1 -type f -executable -exec sh -c '
#    echo
#    echo "========== Run $(basename $1) =========="
#    qemu-riscv64 $1
#    echo "========== Finish $(basename $1) =========="
#    echo
#' sh {} \;
