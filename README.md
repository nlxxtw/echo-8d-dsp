# 动态 8D 环绕（EchoMusic 音效引擎）

实时旋转声像 + 双耳延迟（ITD）+ 轻度空间混响。

- **耳机预设**：绕头 / 双耳（经典 8D、慢速环绕、快速旋转、双耳旋涡）
- **扬声器预设**：大幅左右扫动、弱双耳延迟（音箱左右扫、音箱宽扫、音箱快切）

> 输出设备选「扬声器」时，引擎会自动走音箱算法（大左右、弱 ITD）。选耳机预设并用音响听，绕感会偏弱，请改用带「扬声器」角标的预设。

## Windows 导入

1. EchoMusic → **设置 → 音效管理** → **导入音效引擎**
2. 选择：`dist/echo_8d_dsp.dll`（**不要选 .dylib**）
3. 输出设备选 **扬声器**，预设选 **音箱左右扫 / 音箱宽扫 / 音箱快切**

## macOS 导入（推荐通用包）

请用 **Mac 版 EchoMusic** 导入 `.dylib`（Windows 里导入会报 `failed to load DSP`）。

1. 下载 / 解压：`dist/echo-8d-dsp-macos-universal.zip`  
   （或 `dist/macos-universal/echo_8d_dsp.dylib`）
2. 若刚从网盘 / U 盘拷过来，先在终端执行：

```bash
cd /path/to/unzipped
chmod +x fix-macos-dylib.sh
./fix-macos-dylib.sh
```

3. Mac EchoMusic → **设置 → 音效管理** → **导入音效引擎** → 选 `echo_8d_dsp.dylib`

## Windows 重新编译

```powershell
cd C:\Users\china\Projects\echo-8d-dsp
cargo build --release
Copy-Item .\target\release\echo_8d_dsp.dll .\dist\echo_8d_dsp.dll -Force
```

## 说明

- EchoMusic **原生 DSP Provider**（ABI v2），不是普通 JS 插件。
- 本引擎不支持 VPF；扬声器增强靠左右幅度扫动，不是串扰抵消。
