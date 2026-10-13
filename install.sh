#!/usr/bin/env bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
APP_DEST="$HOME/.local/share/applications"
ICON_DEST="$HOME/.local/share/icons"
HICOLOR_DEST="$HOME/.local/share/icons/hicolor/256x256/apps"

echo "Integrating openWASD (Rust + egui) into your application menu..."

mkdir -p "$APP_DEST"
mkdir -p "$ICON_DEST"
mkdir -p "$HICOLOR_DEST"

# Ensure binary is built
if [ ! -f "$DIR/target/release/openwasd" ]; then
    echo "Building release binary..."
    cd "$DIR"
    cargo build --release
fi

# Instalar ícone no tema XDG para reconhecimento automático no Wayland (KDE/GNOME)
cp "$DIR/icon.png" "$ICON_DEST/openwasd.png"
cp "$DIR/icon.png" "$HICOLOR_DEST/openwasd.png"

cat << DESKTOP > "$APP_DEST/openwasd.desktop"
[Desktop Entry]
Name=openWASD
Comment=Virtual Gamepad Mapper for Keyboard (Rust + egui)
Exec="$DIR/run.sh"
Path=$DIR
Icon=openwasd
Terminal=false
Type=Application
Categories=Game;Utility;
Keywords=gamepad;controller;xbox;virtual;mapper;wasd;openwasd;rust;
StartupWMClass=openwasd
DESKTOP

# Limpar atalho antigo se existir
rm -f "$APP_DEST/wasd-anywhere.desktop"

echo "=========================================="
echo "✓ Desktop shortcut and icons installed!"
echo "openWASD is now in your application menu."
echo "Target: $DIR/run.sh"
echo "=========================================="
