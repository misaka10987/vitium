#!/bin/bash
set -e
pnpx tailwindcss -i ./src/style.css -o ./src/assets/style.css
tsc
cargo tauri dev
