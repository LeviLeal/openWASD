#!/usr/bin/env bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
APP_DEST="$HOME/.local/share/applications"

echo "Integrating WASD Anywhere (Rust + egui) into your application menu..."

mkdir -p "$APP_DEST"

# Ensure binary is built
if [ ! -f "$DIR/target/release/wasd-anywhere" ]; then
    echo "Building release binary..."
    cd "$DIR"
    cargo build --release
fi

cat << DESKTOP > "$APP_DEST/wasd-anywhere.desktop"
[Desktop Entry]
Name=WASD Anywhere
Comment=Virtual Gamepad Mapper for Keyboard (Rust + egui)
Exec="$DIR/run.sh"
Path=$DIR
Icon=$DIR/icon.png
Terminal=false
Type=Application
Categories=Game;Utility;
Keywords=gamepad;controller;xbox;virtual;mapper;wasd;rust;
DESKTOP

echo "=========================================="
echo "✓ Desktop shortcut created!"
echo "WASD Anywhere is now in your application menu."
echo "Target: $DIR/run.sh"
echo "=========================================="
