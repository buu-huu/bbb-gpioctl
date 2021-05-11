#!/usr/bin/env bash
export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER=arm-linux-gnueabihf-gcc
cargo build --release --target armv7-unknown-linux-musleabihf