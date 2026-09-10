#!/usr/bin/env bash
# Fix common macOS EchoMusic DSP import failures for unsigned / quarantined dylibs.
# Usage:
#   ./fix-macos-dylib.sh
#   ./fix-macos-dylib.sh /path/to/echo_8d_dsp.dylib
set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
TARGET="${1:-$ROOT/echo_8d_dsp.dylib}"

if [[ ! -f "$TARGET" ]]; then
  echo "找不到 dylib: $TARGET"
  echo "请先解压 echo-8d-dsp-macos-universal.zip，再对本目录里的 echo_8d_dsp.dylib 运行本脚本。"
  exit 1
fi

echo "==> Clearing quarantine / extended attributes"
xattr -cr "$TARGET" || true

echo "==> Ad-hoc code signing"
codesign --force --sign - --timestamp=none "$TARGET"

echo "==> Verify"
file "$TARGET"
lipo -info "$TARGET" || true
codesign -dv --verbose=2 "$TARGET" 2>&1 | sed -n '1,20p' || true
if ! nm -gU "$TARGET" | grep -q "echo_dsp_get_api"; then
  echo "ERROR: 缺少导出符号 echo_dsp_get_api，这个文件不是有效的 EchoMusic DSP。"
  exit 1
fi

echo
echo "OK. 现在用 Mac 版 EchoMusic → 设置 → 音效管理 → 导入音效引擎 → 选这个文件："
echo "  $TARGET"
echo
echo "注意："
echo "  - Windows 版请导入 .dll，不要导入 .dylib"
echo "  - 不要选 zip / 源码包，只要解压后的 echo_8d_dsp.dylib"
