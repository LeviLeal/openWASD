#!/usr/bin/env bash
set -e

# Build script for WASD Anywhere
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
BUILD_DIR="$ROOT_DIR/build"

echo "========================================"
echo "  Building WASD Anywhere (Release)"
echo "========================================"

mkdir -p "$BUILD_DIR"

# Run CMake configuration if needed
if [ ! -f "$BUILD_DIR/Makefile" ]; then
    echo "-> Running CMake configuration..."
    cmake -B "$BUILD_DIR" -S "$ROOT_DIR" -DCMAKE_BUILD_TYPE=Release
fi

# Compile using all available CPU cores
echo "-> Compiling..."
cmake --build "$BUILD_DIR" -j"$(nproc)"

echo "========================================"
echo "✓ Build complete!"
echo "Binary location: $BUILD_DIR/wasd-gui"
echo "To run the application: ./run.sh (or $BUILD_DIR/wasd-gui)"
echo "========================================"
