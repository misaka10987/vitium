#!/bin/bash
set -e
wget https://code.jquery.com/jquery-3.7.1.min.js -O ./src/assets/jquery.js
npm install -D tailwindcss
npx tailwindcss -i ./src/style.css -o ./src/assets/style.css
tsc
cargo install tauri-cli
cargo tauri build
echo "see build artifacts in ./target/release/"
