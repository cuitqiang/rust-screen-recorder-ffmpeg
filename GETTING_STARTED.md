# 🎬 Screen Recorder - 快速开始指南

## 项目概述

这是一个用 Rust 编写的高性能屏幕录制和直播推流工具。基于 FFmpeg 库，支持：
- 🎥 本地屏幕录制（MP4、AVI、MOV、MKV 等）
- 🌐 RTMP 直播推流
- ⚙️ 灵活的编码和质量设置
- 🖥️ Windows、Linux、macOS 跨平台支持

## 项目结构

```
screen_recorder_ffmpeg/
├── src/
│   ├── main.rs              # 主程序入口，CLI 参数处理
│   ├── config.rs            # 配置管理和验证
│   ├── error.rs             # 自定义错误类型
│   ├── ffmpeg_encoder.rs    # FFmpeg 编码器接口
│   ├── screen_capture.rs    # 屏幕捕获模块
│   └── stream.rs            # RTMP 推流模块
├── Cargo.toml               # 项目依赖配置
├── Cargo.lock               # 依赖版本锁定
├── README.md                # 完整文档
├── EXAMPLES.md              # 详细使用示例
├── GETTING_STARTED.md       # 本文件
└── LICENSE                  # MIT 许可证
```

## 环境要求

### 必须安装
1. **Rust** (1.70+)
   - 下载: https://rustup.rs/
   - Windows 用户也需要安装 Visual Studio Build Tools

2. **FFmpeg** (4.0+)
   - **Windows**: 
     ```powershell
     choco install ffmpeg
     ```
   - **Linux**:
     ```bash
     sudo apt-get install ffmpeg
     ```
   - **macOS**:
     ```bash
     brew install ffmpeg
     ```

### 验证安装

```bash
# 检查 Rust
rustc --version
cargo --version

# 检查 FFmpeg
ffmpeg -version
```

## 快速开始

### 1. 编译项目

```bash
cd H:\Desktop\screen_recorder_ffmpeg
cargo build --release
```

编译完成后，可执行文件位于 `target/release/screen_recorder.exe`

### 2. 基础录制

最简单的录制命令：
```bash
cargo run --release -- --output my_recording.mp4
```

这将：
- 录制整个屏幕
- 使用 1920x1080 分辨率
- 设置 30 FPS 帧率
- 使用 H.264 编码
- 按 Ctrl+C 停止

### 3. 指定参数录制

```bash
cargo run --release -- \
  --output output.mp4 \
  --resolution 1920x1080 \
  --fps 30 \
  --bitrate 5000k \
  --duration 60
```

## 常用命令

### 高质量录制（推荐用于编辑）
```bash
cargo run --release -- \
  --output hq_recording.mp4 \
  --fps 60 \
  --bitrate 8000k \
  --codec h264
```

### 中等质量（平衡存储和质量）
```bash
cargo run --release -- \
  --output recording.mp4 \
  --fps 30 \
  --bitrate 5000k \
  --codec h264
```

### 低带宽（节省存储或网络）
```bash
cargo run --release -- \
  --output recording.mp4 \
  --fps 24 \
  --resolution 1280x720 \
  --bitrate 2500k \
  --codec h265
```

### 推流到 RTMP 服务器
```bash
cargo run --release -- \
  --output rtmp://your-server.com/live/stream \
  --stream \
  --fps 30 \
  --bitrate 5000k
```

## 参数说明

| 参数 | 说明 | 默认值 | 示例 |
|------|------|--------|------|
| `--output` | 输出文件路径或 RTMP URL | 必填 | `output.mp4` / `rtmp://...` |
| `--device` | 捕获设备 | `desktop` | `desktop` / `:0` (Linux) |
| `--fps` | 帧率 (1-120) | `30` | `24`, `30`, `60` |
| `--resolution` | 分辨率 | `1920x1080` | `1920x1080`, `1280x720` |
| `--bitrate` | 比特率 | `5000k` | `2500k`, `5000k`, `10000k` |
| `--codec` | 视频编码器 | `h264` | `h264`, `h265` |
| `--duration` | 录制时长 (秒) | `0` (无限制) | `10`, `60`, `300` |
| `--stream` | 启用推流模式 | `false` | 使用此标志启用 |
| `--log-level` | 日志级别 | `info` | `trace`, `debug`, `info` |

## 实际使用场景

### 场景 1: 录制教学视频

```bash
# 清晰的 1080p 30fps，文件大小适中
cargo run --release -- \
  --output tutorial.mp4 \
  --fps 30 \
  --bitrate 5000k \
  --resolution 1920x1080 \
  --duration 600  # 10 分钟
```

### 场景 2: 实时直播

```bash
# 推流到 YouTube
cargo run --release -- \
  --output rtmp://a.rtmp.youtube.com/live2/YOUR_STREAM_KEY \
  --stream \
  --fps 30 \
  --bitrate 6000k
```

### 场景 3: 长时间录制

```bash
# 低比特率节省存储
cargo run --release -- \
  --output long_recording.mp4 \
  --fps 24 \
  --resolution 1280x720 \
  --bitrate 2500k \
  --codec h265  # 更好的压缩
```

### 场景 4: 高帧率录制（游戏）

```bash
# 60fps 高流畅度
cargo run --release -- \
  --output gameplay.mp4 \
  --fps 60 \
  --bitrate 8000k \
  --codec h264
```

## 输出示例

运行一个命令后，你会看到类似的日志输出：

```
🎬 屏幕录制器启动
📹 输出: output.mp4
📊 分辨率: 1920x1080
⏱️  帧率: 30 FPS
🎥 比特率: 5000k
🔧 编码器: h264
⚙️  初始化 FFmpeg...
✅ FFmpeg 初始化成功
💾 录制模式: output.mp4
🎥 开始屏幕录制...
⏳ 无时间限制，按 Ctrl+C 停止录制
```

按 Ctrl+C 停止录制后：
```
✅ 录制完成: output.mp4
```

## 故障排除

### 问题 1: "ffmpeg 不是内部或外部命令"

**原因**: FFmpeg 未安装或不在 PATH 中
**解决**:
1. 重新安装 FFmpeg
2. 验证安装: `ffmpeg -version`
3. 添加 FFmpeg 到系统 PATH

### 问题 2: 编译失败 "cannot find -lFFmpeg"

**原因**: 系统缺少 FFmpeg 开发库
**解决**: 
- Windows: 使用完整的 FFmpeg 二进制包
- Linux: `sudo apt-get install libavformat-dev libavcodec-dev`

### 问题 3: 录制卡顿或掉帧

**原因**: 编码器过载
**解决**:
- 降低 FPS: `--fps 24`
- 降低分辨率: `--resolution 1280x720`
- 降低比特率: `--bitrate 3000k`
- 关闭其他程序释放 CPU

### 问题 4: 推流连接失败

**原因**: 网络或 RTMP 服务器问题
**解决**:
1. 检查网络连接: `ping server.com`
2. 验证 RTMP URL 格式: `rtmp://server:1935/app/stream`
3. 启用调试日志: `--log-level debug`

## 性能优化

### CPU 占用高

```bash
# 使用更快的编码预设
cargo run --release -- \
  --fps 24 \
  --resolution 1280x720 \
  --bitrate 2000k
```

### 文件太大

```bash
# 使用 H.265 编码压缩率更好
cargo run --release -- \
  --output output.mp4 \
  --codec h265 \
  --bitrate 3000k
```

## 下一步

1. **查看详细示例**: 阅读 [EXAMPLES.md](EXAMPLES.md)
2. **完整文档**: 查看 [README.md](README.md)
3. **修改代码**: 根据需要定制功能
4. **提交反馈**: 提交 Issue 或 PR

## 常见问题

**Q: 支持录制音频吗?**
A: 目前版本支持视频录制。音频支持正在开发中。

**Q: 支持录制游戏吗?**
A: 支持，使用 60fps 和高比特率:
```bash
cargo run --release -- --fps 60 --bitrate 8000k
```

**Q: 可以同时推流和录制吗?**
A: 需要运行两个实例，或修改代码添加此功能。

**Q: 支持 macOS 吗?**
A: 有基本支持，但需要测试。

## 获取帮助

- 查看日志: 添加 `--log-level debug`
- 检查 FFmpeg: `ffmpeg -codecs` 查看可用编码器
- GitHub Issues: 提交问题和建议

---

**开始录制吧！** 🎬

```bash
cargo run --release -- --output my_first_recording.mp4
```
