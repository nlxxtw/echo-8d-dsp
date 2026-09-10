# Apple Silicon（arm64）动态 8D

已通过 GitHub Actions（macos-14）编译完成。

## 导入

Mac 版 EchoMusic → **设置 → 音效管理 → 导入音效引擎** → 选择：

`echo_8d_dsp.dylib`

或使用压缩包：`dist/echo-8d-dsp-macos-arm64.zip`

## 重新编译

```bash
chmod +x scripts/build-macos-apple-silicon.sh
./scripts/build-macos-apple-silicon.sh
```
