#!/usr/bin/env bash
# Build EchoMusic dynamic 8D DSP for macOS (universal by default).
# Run on a Mac (Apple Silicon runner / local Mac with Xcode CLT).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

MODE="${1:-universal}" # universal | arm64 | x64

if ! command -v rustc >/dev/null 2>&1; then
  echo "Rust 未安装。请先安装: https://rustup.rs"
  exit 1
fi

build_one() {
  local triple="$1"
  local out_dir="$2"
  echo "==> Building $triple"
  rustup target add "$triple"
  cargo build --release --target "$triple"
  mkdir -p "$out_dir"
  cp "target/$triple/release/libecho_8d_dsp.dylib" "$out_dir/echo_8d_dsp.dylib"
  codesign --force --sign - --timestamp=none "$out_dir/echo_8d_dsp.dylib"
  xattr -cr "$out_dir/echo_8d_dsp.dylib" || true
  file "$out_dir/echo_8d_dsp.dylib"
  lipo -info "$out_dir/echo_8d_dsp.dylib" || true
  nm -gU "$out_dir/echo_8d_dsp.dylib" | grep echo_dsp_get_api
  cp "$ROOT/README.md" "$out_dir/README.md"
}

case "$MODE" in
  arm64)
    build_one aarch64-apple-darwin "$ROOT/dist/macos-arm64"
    ZIP="$ROOT/dist/echo-8d-dsp-macos-arm64.zip"
    rm -f "$ZIP"
    (cd "$ROOT/dist/macos-arm64" && zip -9 "$ZIP" echo_8d_dsp.dylib README.md)
    echo "Done: $ROOT/dist/macos-arm64/echo_8d_dsp.dylib"
    ;;
  x64)
    build_one x86_64-apple-darwin "$ROOT/dist/macos-x64"
    ZIP="$ROOT/dist/echo-8d-dsp-macos-x64.zip"
    rm -f "$ZIP"
    (cd "$ROOT/dist/macos-x64" && zip -9 "$ZIP" echo_8d_dsp.dylib README.md)
    echo "Done: $ROOT/dist/macos-x64/echo_8d_dsp.dylib"
    ;;
  universal|*)
    build_one aarch64-apple-darwin "$ROOT/dist/macos-arm64"
    build_one x86_64-apple-darwin "$ROOT/dist/macos-x64"
    OUT="$ROOT/dist/macos-universal"
    mkdir -p "$OUT"
    lipo -create -output "$OUT/echo_8d_dsp.dylib" \
      "$ROOT/dist/macos-arm64/echo_8d_dsp.dylib" \
      "$ROOT/dist/macos-x64/echo_8d_dsp.dylib"
    codesign --force --sign - --timestamp=none "$OUT/echo_8d_dsp.dylib"
    xattr -cr "$OUT/echo_8d_dsp.dylib" || true
    cp "$ROOT/README.md" "$OUT/README.md"
    cp "$ROOT/scripts/fix-macos-dylib.sh" "$OUT/fix-macos-dylib.sh"
    chmod +x "$OUT/fix-macos-dylib.sh"
    file "$OUT/echo_8d_dsp.dylib"
    lipo -info "$OUT/echo_8d_dsp.dylib"
    nm -gU "$OUT/echo_8d_dsp.dylib" | grep echo_dsp_get_api
    ZIP="$ROOT/dist/echo-8d-dsp-macos-universal.zip"
    rm -f "$ZIP"
    (
      cd "$OUT"
      zip -9 "$ZIP" echo_8d_dsp.dylib README.md fix-macos-dylib.sh
    )
    echo "Done: $OUT/echo_8d_dsp.dylib"
    echo "Zip:  $ZIP"
    ;;
esac

echo
echo "Import in Mac EchoMusic: 设置 → 音效管理 → 导入音效引擎 → echo_8d_dsp.dylib"
echo "If import still fails: bash scripts/fix-macos-dylib.sh /path/to/echo_8d_dsp.dylib"
