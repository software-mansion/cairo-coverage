#!/usr/bin/env bash
set -euxo pipefail

TARGET="$1"
PKG_FULL_NAME="$2"

rm -rf "$PKG_FULL_NAME"
mkdir -p "$PKG_FULL_NAME/bin"

binary_extension=""
[[ "$TARGET" == *-windows-* ]] && binary_extension=".exe"

cp "./target/${TARGET}/release/cairo-coverage${binary_extension}" "$PKG_FULL_NAME/bin/"

cp -r README.md "$PKG_FULL_NAME/"

if [[ "$TARGET" == *-windows-* ]]; then
  7z a "${PKG_FULL_NAME}.zip" "$PKG_FULL_NAME"
else
  tar czvf "${PKG_FULL_NAME}.tar.gz" "$PKG_FULL_NAME"
fi
