# macOS 动态 8D（Apple Silicon）

若你手里只有这个目录，也可以导入；**更推荐**用上级目录的：

`../echo-8d-dsp-macos-universal.zip`

（同时支持 Apple Silicon 与 Intel / Rosetta）

## 导入前（重要）

从网盘、浏览器、Windows 拷到 Mac 后，先执行：

```bash
cd "$(dirname "$0")"
xattr -cr echo_8d_dsp.dylib
codesign --force --sign - echo_8d_dsp.dylib
```

或用仓库里的：`scripts/fix-macos-dylib.sh`

## 导入

Mac 版 EchoMusic → **设置 → 音效管理 → 导入音效引擎** → 选本目录 `echo_8d_dsp.dylib`

**不要**在 Windows EchoMusic 里导入 `.dylib`，会报 `failed to load DSP`。
