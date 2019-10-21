#!/bin/sh
cargo build
arm-none-eabi-objcopy -O binary target/thumbv6m-none-eabi/debug/stroborust stroborust.bin
st-flash write stroborust.bin 0x8000000