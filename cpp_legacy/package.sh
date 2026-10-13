#!/usr/bin/env bash
set -e

# Packaging script for WASD Anywhere
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
VERSION="0.1.0"
ARCH="$(uname -m)"
PACKAGE_NAME="wasd-anywhere-v${VERSION}-linux-${ARCH}"
DIST_DIR="$ROOT_DIR/dist"
STAGE_DIR="$DIST_DIR/$PACKAGE_NAME"

echo "========================================"
echo "  Packaging WASD Anywhere v${VERSION}"
echo "========================================"

# 1. Ensure project is built
echo "-> Building project..."
"$ROOT_DIR/build.sh"

BIN_PATH="$ROOT_DIR/build/wasd-gui"
if [ ! -f "$BIN_PATH" ]; then
    echo "Error: Binary not found at $BIN_PATH"
    exit 1
fi

# 2. Prepare staging directory
echo "-> Creating staging structure in $STAGE_DIR..."
rm -rf "$STAGE_DIR"
mkdir -p "$STAGE_DIR"

# Copy binary and strip debug symbols to reduce size
cp "$BIN_PATH" "$STAGE_DIR/wasd-gui"
strip "$STAGE_DIR/wasd-gui" 2>/dev/null || true
chmod +x "$STAGE_DIR/wasd-gui"

# Copy icon and desktop file
cp "$ROOT_DIR/icon.png" "$STAGE_DIR/icon.png"
cp "$ROOT_DIR/wasd-anywhere.desktop" "$STAGE_DIR/wasd-anywhere.desktop"
cp "$ROOT_DIR/setup_permissions.sh" "$STAGE_DIR/setup_permissions.sh"
chmod +x "$STAGE_DIR/setup_permissions.sh"

# Launcher script inside tarball
cat << 'EOF' > "$STAGE_DIR/run.sh"
#!/usr/bin/env bash
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
exec "$DIR/wasd-gui" "$@"
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
EOF
chmod +x "$STAGE_DIR/install.sh"

# Uninstallation script
cat << 'EOF' > "$STAGE_DIR/uninstall.sh"
#!/usr/bin/env bash
echo "Removing desktop shortcut..."
rm -f "$HOME/.local/share/applications/wasd-anywhere.desktop"
echo "✓ Desktop shortcut removed."
EOF
chmod +x "$STAGE_DIR/uninstall.sh"

# README file
cat << 'EOF' > "$STAGE_DIR/README.txt"
WASD Anywhere — Virtual Gamepad Mapper for Linux
=================================================

HOW TO RUN:
1. Portable Mode:
   ./run.sh
   (or directly ./wasd-gui)

2. Create Desktop Shortcut:
   ./install.sh
   (Adds WASD Anywhere to your KDE/GNOME application menu pointing to this directory)

3. Remove Desktop Shortcut:
   ./uninstall.sh

4. Permissions (First time only):
   If you see a permission error, run:
   ./setup_permissions.sh

REQUIREMENTS:
- Linux kernel with /dev/uinput enabled (standard on modern Linux)
- Qt 6 Widgets and libevdev
EOF

# 3. Create compressed tarball
echo "-> Creating tarball: ${PACKAGE_NAME}.tar.gz..."
cd "$DIST_DIR"
tar -czvf "${PACKAGE_NAME}.tar.gz" "$PACKAGE_NAME"

TARBALL_PATH="$DIST_DIR/${PACKAGE_NAME}.tar.gz"
SIZE=$(du -h "$TARBALL_PATH" | cut -f1)

echo "========================================"
echo "✓ Packaging successful!"
echo "Package: $TARBALL_PATH ($SIZE)"
echo "SHA256:  $(sha256sum "$TARBALL_PATH" | cut -d' ' -f1)"
echo "========================================"
