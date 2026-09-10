# 动态 8D 环绕（EchoMusic 音效引擎）

实时旋转声像 + 双耳延迟（ITD）+ 轻度空间混响，做出「会绕头转」的 8D / 双耳环绕效果。

> 建议用**耳机**听。扬声器模式下引擎会自动减弱环绕强度。

## Windows 导入

1. EchoMusic → **设置 → 音效管理** → **导入音效引擎**
2. 选择：`dist/echo_8d_dsp.dll`  
   或压缩包：`echo-8d-dsp.zip`
3. 音效面板选预设：经典 8D / 慢速环绕 / 快速旋转 / 双耳旋涡

## macOS Apple Silicon 导入

Windows 上**编不出** `.dylib`，请在 Mac 上编译（约 1 分钟）：

```bash
cd /path/to/echo-8d-dsp
chmod +x scripts/build-macos-apple-silicon.sh
./scripts/build-macos-apple-silicon.sh
```

产物：

- `dist/macos-arm64/echo_8d_dsp.dylib`
- `dist/echo-8d-dsp-macos-arm64.zip`

然后在 Mac 版 EchoMusic：**设置 → 音效管理 → 导入音效引擎** → 选该 `.dylib`。

更完整说明见：`dist/macos-arm64/README.md`  
也可用 GitHub Actions：`.github/workflows/build-macos-arm64.yml`（`macos-14` runner）。

## 文件

| 文件 | 说明 |
|------|------|
| `dist/echo_8d_dsp.dll` | Windows x64 |
| `dist/macos-arm64/echo_8d_dsp.dylib` | Apple Silicon（需在 Mac 编译后生成） |
| `scripts/build-macos-apple-silicon.sh` | Mac 一键编译脚本 |

## Windows 重新编译

```powershell
cd C:\Users\china\Projects\echo-8d-dsp
cargo build --release
Copy-Item .\target\release\echo_8d_dsp.dll .\dist\echo_8d_dsp.dll -Force
```

## 说明

- EchoMusic **原生 DSP Provider**（ABI v2），不是普通 JS 插件。
- 与静态 WAV 空间音效包不同：本引擎会**随时间旋转**声像。
