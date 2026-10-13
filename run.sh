#!/usr/bin/env bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
BIN="$DIR/target/release/wasd-anywhere"

if [ -f "$BIN" ]; then
    exec "$BIN" "$@"
else
    echo "Building and starting WASD Anywhere (Rust + egui)..."
    cd "$DIR"
    cargo run --release -- "$@"
fi
