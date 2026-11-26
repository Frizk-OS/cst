#!/usr/bin/env bash
# Helper script to configure Meson build strictly with LLVM and Clang
set -e

BUILD_DIR="${1:-build}"
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

echo "==> Configuring Meson build with LLVM/Clang (native-clang.ini) into '$BUILD_DIR'..."
meson setup "$BUILD_DIR" "$ROOT_DIR" \
    --native-file "$ROOT_DIR/tools/meson/native-clang.ini" \
    --reconfigure || meson setup "$BUILD_DIR" "$ROOT_DIR" \
    --native-file "$ROOT_DIR/tools/meson/native-clang.ini"

echo "==> Build directory '$BUILD_DIR' is ready with LLVM/Clang. To compile, run: ninja -C $BUILD_DIR"
