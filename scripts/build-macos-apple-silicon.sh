#!/usr/bin/env bash
# Build EchoMusic dynamic 8D DSP for Apple Silicon (arm64).
# Run on a Mac with Apple Silicon (M1/M2/M3/M4) or any Mac that can target arm64.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

if ! command -v rustc >/dev/null 2>&1; then
  echo "Rust 未安装。请先安装: https://rustup.rs"
  exit 1
fi

echo "==> Ensuring aarch64-apple-darwin target"
rustup target add aarch64-apple-darwin

echo "==> Building release dylib"
cargo build --release --target aarch64-apple-darwin

OUT_DIR="$ROOT/dist/macos-arm64"
mkdir -p "$OUT_DIR"
SRC="$ROOT/target/aarch64-apple-darwin/release/libecho_8d_dsp.dylib"
DST="$OUT_DIR/echo_8d_dsp.dylib"

cp "$SRC" "$DST"
cp "$ROOT/README.md" "$OUT_DIR/README.md"

echo "==> Verifying architecture / export"
file "$DST"
lipo -info "$DST" || true
if ! nm -gU "$DST" | grep -q "echo_dsp_get_api"; then
  echo "ERROR: echo_dsp_get_api not found in dylib"
  exit 1
fi

ZIP="$ROOT/dist/echo-8d-dsp-macos-arm64.zip"
rm -f "$ZIP"
(
  cd "$OUT_DIR"
  zip -9 "$ZIP" echo_8d_dsp.dylib README.md
)

echo
echo "Done."
echo "  dylib: $DST"
echo "  zip:   $ZIP"
echo
echo "Import in EchoMusic (macOS): Settings → 音效管理 → 导入音效引擎 → select echo_8d_dsp.dylib"
