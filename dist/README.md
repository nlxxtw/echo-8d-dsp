# 动态 8D 环绕（EchoMusic 音效引擎）

实时旋转声像 + 双耳延迟（ITD）+ 轻度空间混响，做出「会绕头转」的 8D / 双耳环绕效果。

> 建议用**耳机**听。扬声器模式下引擎会自动减弱环绕强度。

## 导入

1. 打开 EchoMusic → **设置 → 音效管理**
2. 点击 **导入音效引擎**
3. 选择本目录中的：

`C:\Users\china\Projects\echo-8d-dsp\dist\echo_8d_dsp.dll`

或解压后的：

`echo_8d_dsp.dll`（在 zip 包内）

4. 启用该引擎后，在播放器 **音效** 面板选择预设：
   - 经典 8D
   - 慢速环绕
   - 快速旋转
   - 双耳旋涡
5. 点预设旁的设置图标可调：旋转周期、环绕深度、空间混响、效果比例

## 文件

| 文件 | 说明 |
|------|------|
| `dist/echo_8d_dsp.dll` | Windows x64 音效引擎（导入这个） |
| `echo-8d-dsp.zip` | 便于分发的压缩包 |

## 重新编译

需要 Rust（Windows GNU toolchain）：

```powershell
cd C:\Users\china\Projects\echo-8d-dsp
$env:CARGO_TARGET_DIR = "$PWD\target"
cargo build --release
Copy-Item .\target\release\echo_8d_dsp.dll .\dist\echo_8d_dsp.dll -Force
```

## 说明

- 这是 EchoMusic **原生 DSP Provider**（ABI v2），不是普通 JS 插件。
- 与之前的静态 WAV 空间音效包不同：本引擎会**随时间旋转**声像。
- 仅支持 Windows x64（当前构建目标）。若要 macOS/Linux，需要在对应平台重新编译。
