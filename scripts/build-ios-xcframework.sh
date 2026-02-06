#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CORE_DIR="$ROOT_DIR/core/rust-core"
OUTPUT_DIR="$ROOT_DIR/artifacts/ios"
XCFRAMEWORK="$OUTPUT_DIR/PantherSecurityCore.xcframework"

mkdir -p "$OUTPUT_DIR"

# Ensure targets are installed (ignore errors if already present)
if command -v rustup >/dev/null 2>&1; then
  rustup target add aarch64-apple-ios aarch64-apple-ios-sim || true
fi

cargo build --manifest-path "$CORE_DIR/Cargo.toml" --target aarch64-apple-ios --release
cargo build --manifest-path "$CORE_DIR/Cargo.toml" --target aarch64-apple-ios-sim --release

IOS_LIB="$CORE_DIR/target/aarch64-apple-ios/release/librust_core.a"
SIM_LIB="$CORE_DIR/target/aarch64-apple-ios-sim/release/librust_core.a"

rm -rf "$XCFRAMEWORK"

xcodebuild -create-xcframework \
  -library "$IOS_LIB" -headers "$CORE_DIR/include" \
  -library "$SIM_LIB" -headers "$CORE_DIR/include" \
  -output "$XCFRAMEWORK"

echo "XCFramework created at: $XCFRAMEWORK"
