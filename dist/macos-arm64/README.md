# Apple Silicon（arm64）编译说明

当前 Windows 开发机**无法直接产出** `echo_8d_dsp.dylib`（需要 macOS 链接器 / SDK）。

可选两种方式：

## 方式 A：在 Mac 上一键编译（最快）

1. 把整个 `echo-8d-dsp` 文件夹拷到 Mac（或用 U 盘 / 网盘 / git）
2. 安装 Rust：https://rustup.rs
3. 终端执行：

```bash
cd /path/to/echo-8d-dsp
chmod +x scripts/build-macos-apple-silicon.sh
./scripts/build-macos-apple-silicon.sh
```

4. 产物：
   - `dist/macos-arm64/echo_8d_dsp.dylib`
   - `dist/echo-8d-dsp-macos-arm64.zip`

5. Mac 版 EchoMusic → **设置 → 音效管理 → 导入音效引擎** → 选该 `.dylib`

## 方式 B：GitHub Actions

仓库已包含 `.github/workflows/build-macos-arm64.yml`（`macos-14` = Apple Silicon）。

1. 把项目推到 GitHub
2. 打开 Actions → **Build Apple Silicon DSP** → Run workflow
3. 下载产物 `echo-8d-dsp-macos-arm64` / zip

## 导入注意

- 文件名用 `echo_8d_dsp.dylib` 即可（脚本已从 `libecho_8d_dsp.dylib` 重命名）
- 需 Apple Silicon 的 EchoMusic（M 系列 Mac）
- Intel Mac 需要另编 `x86_64-apple-darwin`（本脚本默认 arm64）
