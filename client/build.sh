#!/bin/bash
set -e
cargo install tauri-cli
./build-tailwind.sh
cargo tauri build
echo "see build artifacts in ./target/release/"
