#!/usr/bin/env bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
mkdir -p "$DIR/build"
cd "$DIR/build"

if [ ! -f "Makefile" ]; then
    cmake ..
fi

make -j$(nproc)
echo "Starting WASD Anywhere..."
exec ./wasd-gui "$@"
