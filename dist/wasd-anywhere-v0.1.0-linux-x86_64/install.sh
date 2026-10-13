#!/usr/bin/env bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
APP_DEST="$HOME/.local/share/applications"

echo "Integrating WASD Anywhere into your application menu..."

mkdir -p "$APP_DEST"

# Install .desktop entry pointing directly to this directory
cat << DESKTOP > "$APP_DEST/wasd-anywhere.desktop"
[Desktop Entry]
Name=WASD Anywhere
Comment=Virtual Gamepad Mapper for Keyboard
Exec="$DIR/wasd-gui"
Path=$DIR
Icon=$DIR/icon.png
Terminal=false
Type=Application
Categories=Game;Utility;
Keywords=gamepad;controller;xbox;virtual;mapper;wasd;
DESKTOP

# Run permissions check if needed
if [ -f "$DIR/setup_permissions.sh" ]; then
    "$DIR/setup_permissions.sh"
fi

echo "=========================================="
echo "✓ Desktop entry created successfully!"
echo "WASD Anywhere is now in your application menu."
echo "Target binary: $DIR/wasd-gui"
echo "=========================================="
