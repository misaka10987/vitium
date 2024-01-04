#!/bin/bash
cargo build
cbindgen --config cbindgen.toml --crate vitium-client --output vitium_client_rust.h
g++ ./test_call_rust.cpp ./target/debug/libvitium_client.a -o test-call-rust
echo "test output should be 3:"
./test-call-rust
