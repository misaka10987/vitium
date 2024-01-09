#!/bin/bash
set -e
cargo check
./cfi.sh
cargo build
gcc ./test_cfi.c ./target/debug/libvitium_common_cfi.a -o test
./test