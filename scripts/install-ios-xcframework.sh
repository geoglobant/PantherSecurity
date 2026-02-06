#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

"$ROOT_DIR/scripts/build-ios-xcframework.sh"

DEST_DIR="$ROOT_DIR/mobile/ios/sampleIOS/Frameworks"
SRC_DIR="$ROOT_DIR/artifacts/ios/PantherSecurityCore.xcframework"

mkdir -p "$DEST_DIR"
rm -rf "$DEST_DIR/PantherSecurityCore.xcframework"
cp -R "$SRC_DIR" "$DEST_DIR/"

echo "xcframework copiada para: $DEST_DIR/PantherSecurityCore.xcframework"

echo "Abra o Xcode e adicione a xcframework em:"

echo "  Target > General > Frameworks, Libraries, and Embedded Content"
