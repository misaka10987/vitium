#!/bin/bash
set -e
# npm install -D tailwindcss
./build-tailwind.sh
cargo install tauri-cli
cargo tauri build
echo "see build artifacts in ./target/release/"
