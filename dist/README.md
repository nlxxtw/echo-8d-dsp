# 动态 8D 环绕（EchoMusic 音效引擎）

## Windows

导入：`echo_8d_dsp.dll`（本目录）

## macOS

导入：`macos-arm64/echo_8d_dsp.dylib` 或更好：`echo-8d-dsp-macos-universal.zip` 解压后的 dylib。

从网盘拷到 Mac 后先执行：

```bash
xattr -cr echo_8d_dsp.dylib
codesign --force --sign - echo_8d_dsp.dylib
```

**平台别混：**

| 系统 | 文件 |
|------|------|
| Windows EchoMusic | `.dll` |
| Mac EchoMusic | `.dylib` |

混用会直接 `failed to load DSP`。
