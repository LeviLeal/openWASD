#!/usr/bin/env bash
set -e

# Packaging script for WASD Anywhere (Rust + egui)
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
VERSION="0.2.0"
ARCH="$(uname -m)"
PACKAGE_NAME="wasd-anywhere-v${VERSION}-linux-${ARCH}"
DIST_DIR="$ROOT_DIR/dist"
STAGE_DIR="$DIST_DIR/$PACKAGE_NAME"

echo "========================================"
echo "  Packaging WASD Anywhere v${VERSION} (Rust)"
echo "========================================"

# 1. Build release binary
echo "-> Building project with cargo..."
cd "$ROOT_DIR"
cargo build --release

BIN_PATH="$ROOT_DIR/target/release/wasd-anywhere"
if [ ! -f "$BIN_PATH" ]; then
    echo "Error: Binary not found at $BIN_PATH"
    exit 1
fi

# 2. Prepare staging directory
echo "-> Creating staging structure in $STAGE_DIR..."
rm -rf "$STAGE_DIR"
mkdir -p "$STAGE_DIR"

# Copy binary and strip debug symbols to reduce size
cp "$BIN_PATH" "$STAGE_DIR/wasd-anywhere"
strip "$STAGE_DIR/wasd-anywhere" 2>/dev/null || true
chmod +x "$STAGE_DIR/wasd-anywhere"

# Copy icon and scripts
cp "$ROOT_DIR/icon.png" "$STAGE_DIR/icon.png"
cp "$ROOT_DIR/setup_permissions.sh" "$STAGE_DIR/setup_permissions.sh"
chmod +x "$STAGE_DIR/setup_permissions.sh"

# Launcher script inside tarball
cat << 'EOF' > "$STAGE_DIR/run.sh"
#!/usr/bin/env bash
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
exec "$DIR/wasd-anywhere" "$@"
EOF
chmod +x "$STAGE_DIR/run.sh"

# In-place desktop integration script
cat << 'EOF' > "$STAGE_DIR/install.sh"
#!/usr/bin/env bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
APP_DEST="$HOME/.local/share/applications"

echo "Integrating WASD Anywhere into your application menu..."

mkdir -p "$APP_DEST"

cat << DESKTOP > "$APP_DEST/wasd-anywhere.desktop"
[Desktop Entry]
Name=WASD Anywhere
Comment=Virtual Gamepad Mapper for Keyboard
Exec="$DIR/run.sh"
Path=$DIR
Icon=$DIR/icon.png
Terminal=false
Type=Application
Categories=Game;Utility;
Keywords=gamepad;controller;xbox;virtual;mapper;wasd;
DESKTOP

echo "=========================================="
echo "✓ Desktop shortcut created!"
echo "WASD Anywhere is now in your application menu."
echo "Target: $DIR/run.sh"
echo "=========================================="
EOF
chmod +x "$STAGE_DIR/install.sh"

# Uninstall script
cat << 'EOF' > "$STAGE_DIR/uninstall.sh"
#!/usr/bin/env bash
echo "Removing desktop shortcut..."
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
