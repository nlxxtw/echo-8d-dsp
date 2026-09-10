# 动态 8D 环绕（EchoMusic 音效引擎）

实时旋转声像 + 双耳延迟（ITD）+ 轻度空间混响，做出「会绕头转」的 8D / 双耳环绕效果。

> 建议用**耳机**听。扬声器模式下引擎会自动减弱环绕强度。

## Windows 导入

1. EchoMusic → **设置 → 音效管理** → **导入音效引擎**
2. 选择：`dist/echo_8d_dsp.dll`（**不要选 .dylib**）
3. 音效面板选预设：经典 8D / 慢速环绕 / 快速旋转 / 双耳旋涡

## macOS 导入（推荐通用包）

请用 **Mac 版 EchoMusic** 导入 `.dylib`（Windows 里导入会报 `failed to load DSP`）。

1. 下载 / 解压：`dist/echo-8d-dsp-macos-universal.zip`  
   （或 `dist/macos-universal/echo_8d_dsp.dylib`）
2. 若刚从网盘 / U 盘拷过来，先在终端执行：

```bash
cd /path/to/unzipped
chmod +x fix-macos-dylib.sh
./fix-macos-dylib.sh
# 等价于：
# xattr -cr echo_8d_dsp.dylib
# codesign --force --sign - echo_8d_dsp.dylib
```

3. Mac EchoMusic → **设置 → 音效管理** → **导入音效引擎** → 选 `echo_8d_dsp.dylib`

### 导入失败排查

报错形如：`音效引擎导入失败 ... failed to load DSP ...`

| 情况 | 处理 |
|------|------|
| 在 Windows EchoMusic 里导入了 `.dylib` | 改用 `echo_8d_dsp.dll` |
| 选了 zip / 源码包 | 先解压，只选 `.dylib` |
| 刚下载被隔离（最常见） | 运行上面的 `fix-macos-dylib.sh` |
| Intel Mac / Rosetta | 用 **universal** 包，不要只用旧的 arm64 单架构包 |
| 仍失败 | 终端把完整错误贴出来：把 dylib 路径告诉我，或看 EchoMusic 日志 |

## 本地重新编译（需 Mac）

```bash
chmod +x scripts/build-macos-apple-silicon.sh
./scripts/build-macos-apple-silicon.sh          # universal
./scripts/build-macos-apple-silicon.sh arm64    # 仅 Apple Silicon
./scripts/build-macos-apple-silicon.sh x64      # 仅 Intel
```

GitHub Actions：`.github/workflows/build-macos-arm64.yml`（会产出 universal / arm64 / x64）。

## 文件

| 文件 | 说明 |
|------|------|
| `dist/echo_8d_dsp.dll` | Windows x64 |
| `dist/macos-universal/echo_8d_dsp.dylib` | Mac 通用（arm64 + x86_64，推荐） |
| `dist/macos-arm64/echo_8d_dsp.dylib` | 仅 Apple Silicon |
| `scripts/fix-macos-dylib.sh` | Mac 去隔离 + 临时签名 |

## Windows 重新编译

```powershell
cd C:\Users\china\Projects\echo-8d-dsp
cargo build --release
Copy-Item .\target\release\echo_8d_dsp.dll .\dist\echo_8d_dsp.dll -Force
```

## 说明

- EchoMusic **原生 DSP Provider**（ABI v2），不是普通 JS 插件。
- 与静态 WAV 空间音效包不同：本引擎会**随时间旋转**声像。
