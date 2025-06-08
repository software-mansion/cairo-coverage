#!/usr/bin/env bash
set -euxo pipefail

TARGET="$1"
PKG_FULL_NAME="$2"

rm -rf "$PKG_FULL_NAME"
mkdir -p "$PKG_FULL_NAME/bin"

cp "./target/${TARGET}/release/cairo-coverage" "$PKG_FULL_NAME/bin/"

cp -r README.md "$PKG_FULL_NAME/"

tar czvf "${PKG_FULL_NAME}.tar.gz" "$PKG_FULL_NAME"
