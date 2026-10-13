#!/usr/bin/env bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
APP_DEST="$HOME/.local/share/applications"

echo "Integrating openWASD (Rust + egui) into your application menu..."

mkdir -p "$APP_DEST"

# Ensure binary is built
if [ ! -f "$DIR/target/release/openwasd" ]; then
    echo "Building release binary..."
    cd "$DIR"
    cargo build --release
fi

cat << DESKTOP > "$APP_DEST/openwasd.desktop"
[Desktop Entry]
Name=openWASD
Comment=Virtual Gamepad Mapper for Keyboard (Rust + egui)
Exec="$DIR/run.sh"
Path=$DIR
Icon=$DIR/icon.png
Terminal=false
Type=Application
Categories=Game;Utility;
Keywords=gamepad;controller;xbox;virtual;mapper;wasd;openwasd;rust;
DESKTOP

# Limpar atalho antigo se existir
rm -f "$APP_DEST/wasd-anywhere.desktop"

echo "=========================================="
echo "✓ Desktop shortcut created!"
echo "openWASD is now in your application menu."
echo "Target: $DIR/run.sh"
echo "=========================================="
