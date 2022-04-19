#!/bin/sh

cargo build --release || exit 1
arm-none-eabi-objcopy -O binary target/thumbv4t-none-eabi/release/gba target/build.gba || exit 1
gbafix target/build.gba || exit 1
mgba -g target/build.gba
echo "ROM built successfully!"
