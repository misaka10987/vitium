#!/bin/bash
set -e
npx tailwindcss -i ./src/style.css -O ./src/assets/style.css > /dev/null
tsc
cargo tauri dev
