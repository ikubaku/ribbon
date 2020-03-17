#!/bin/sh

cargo build --release
arm-none-eabi-objcopy -O binary target/thumbv7m-none-eabi/release/ribbon ribbon.bin
