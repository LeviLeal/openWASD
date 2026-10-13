#!/usr/bin/env bash
set -e

# Packaging script for openWASD (Rust + egui)
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
VERSION="0.2.0"
ARCH="$(uname -m)"
PACKAGE_NAME="openwasd-v${VERSION}-linux-${ARCH}"
DIST_DIR="$ROOT_DIR/dist"
STAGE_DIR="$DIST_DIR/$PACKAGE_NAME"

echo "========================================"
echo "  Packaging openWASD v${VERSION} (Rust)"
echo "========================================"

# 1. Build release binary
echo "-> Building project with cargo..."
cd "$ROOT_DIR"
cargo build --release

BIN_PATH="$ROOT_DIR/target/release/openwasd"
if [ ! -f "$BIN_PATH" ]; then
    echo "Error: Binary not found at $BIN_PATH"
    exit 1
fi

# 2. Prepare staging directory
echo "-> Creating staging structure in $STAGE_DIR..."
rm -rf "$STAGE_DIR"
mkdir -p "$STAGE_DIR"

# Copy binary and strip debug symbols to reduce size
cp "$BIN_PATH" "$STAGE_DIR/openwasd"
strip "$STAGE_DIR/openwasd" 2>/dev/null || true
chmod +x "$STAGE_DIR/openwasd"

# Copy icon and scripts
cp "$ROOT_DIR/icon.png" "$STAGE_DIR/icon.png"
cp "$ROOT_DIR/setup_permissions.sh" "$STAGE_DIR/setup_permissions.sh"
chmod +x "$STAGE_DIR/setup_permissions.sh"

# Launcher script inside tarball
cat << 'EOF' > "$STAGE_DIR/run.sh"
#!/usr/bin/env bash
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
exec "$DIR/openwasd" "$@"
EOF
chmod +x "$STAGE_DIR/run.sh"

# In-place desktop integration script
cat << 'EOF' > "$STAGE_DIR/install.sh"
#!/usr/bin/env bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
APP_DEST="$HOME/.local/share/applications"
ICON_DEST="$HOME/.local/share/icons"
HICOLOR_DEST="$HOME/.local/share/icons/hicolor/256x256/apps"

echo "Integrating openWASD into your application menu..."

mkdir -p "$APP_DEST"
mkdir -p "$ICON_DEST"
mkdir -p "$HICOLOR_DEST"

# Instalar ícone no tema XDG para reconhecimento automático no Wayland (KDE/GNOME)
cp "$DIR/icon.png" "$ICON_DEST/openwasd.png"
cp "$DIR/icon.png" "$HICOLOR_DEST/openwasd.png"

cat << DESKTOP > "$APP_DEST/openwasd.desktop"
[Desktop Entry]
Name=openWASD
Comment=Virtual Gamepad Mapper for Keyboard
Exec="$DIR/run.sh"
Path=$DIR
Icon=openwasd
Terminal=false
Type=Application
Categories=Game;Utility;
Keywords=gamepad;controller;xbox;virtual;mapper;wasd;openwasd;
StartupWMClass=openwasd
DESKTOP

# Limpar atalho antigo se existir
rm -f "$APP_DEST/wasd-anywhere.desktop"

echo "=========================================="
echo "✓ Desktop shortcut and icons installed!"
echo "openWASD is now in your application menu."
echo "Target: $DIR/run.sh"
echo "=========================================="
EOF
chmod +x "$STAGE_DIR/install.sh"

# Uninstall script
cat << 'EOF' > "$STAGE_DIR/uninstall.sh"
#!/usr/bin/env bash
echo "Removing desktop shortcut..."
rm -f "$HOME/.local/share/applications/openwasd.desktop"
rm -f "$HOME/.local/share/applications/wasd-anywhere.desktop"
echo "✓ Shortcut removed."
EOF
chmod +x "$STAGE_DIR/uninstall.sh"

# 3. Create Tarball
echo "-> Compressing archive..."
cd "$DIST_DIR"
tar -czf "${PACKAGE_NAME}.tar.gz" "$PACKAGE_NAME"

echo "========================================"
echo "✓ Package created successfully!"
echo "Archive: $DIST_DIR/${PACKAGE_NAME}.tar.gz"
echo "Size: $(du -h "$DIST_DIR/${PACKAGE_NAME}.tar.gz" | cut -f1)"
echo "========================================"
