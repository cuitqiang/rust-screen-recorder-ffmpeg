# 🎬 Screen Recorder - FFmpeg 屏幕录制和推流工具

![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)
![License](https://img.shields.io/badge/License-MIT-blue)
![FFmpeg](https://img.shields.io/badge/FFmpeg-7.0-blue)

一个高性能的 Rust 屏幕录制和推流工具，基于 FFmpeg 库。支持本地录制和 RTMP 推流，提供灵活的编码选项和实时监控。

## ✨ 功能特性

### 核心功能
- 🎥 **屏幕录制** - 支持本地视频文件保存
- 🌐 **RTMP 推流** - 直播流推送到服务器
- 📹 **多编码器** - 支持 H.264、H.265、硬件加速编码
- ⚙️ **灵活配置** - 自定义分辨率、帧率、比特率
- 📊 **实时监控** - 推流统计和性能指标
- 🖥️ **跨平台** - Windows、Linux、macOS 支持

### 编码选项
- ✅ **H.264 (libx264)** - 兼容性最好
- ✅ **H.265 (libx265)** - 更好的压缩率
- ✅ **NVIDIA NVENC** - GPU 硬件加速
- ✅ **Intel QSV** - Intel 快速同步
- ✅ **Apple VideoToolbox** - macOS GPU 加速

## 🚀 快速开始

### 前置要求
- Rust 1.70 或更新版本
- FFmpeg 4.0 或更新版本
- Windows / Linux / macOS

### 安装 FFmpeg

**Windows:**
```powershell
# 使用 Chocolatey
choco install ffmpeg

# 或从官网下载: https://ffmpeg.org/download.html
```

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get install ffmpeg
```

**macOS:**
```bash
brew install ffmpeg
```

### 编译项目

```bash
cd screen_recorder_ffmpeg
cargo build --release
```

## 📖 使用示例

### 基础录制

将屏幕录制为 MP4 文件：
```bash
cargo run --release -- --output output.mp4
```

### 指定参数

```bash
# 指定分辨率、帧率和比特率
cargo run --release -- \
  --output output.mp4 \
  --resolution 1920x1080 \
  --fps 30 \
  --bitrate 5000k

# 录制 10 秒
cargo run --release -- \
  --output test.mp4 \
  --duration 10

# 使用 H.265 编码
cargo run --release -- \
  --output output.mp4 \
  --codec h265 \
  --bitrate 3000k
```

### RTMP 推流

推流到 RTMP 服务器：
```bash
cargo run --release -- \
  --output rtmp://your-server.com/live/stream \
  --stream \
  --fps 30 \
  --bitrate 5000k

# 推流到 YouTube Live
cargo run --release -- \
  --output rtmp://a.rtmp.youtube.com/live2/YOUR_STREAM_KEY \
  --stream \
  --fps 30 \
  --bitrate 6000k
```

## 🎯 命令行参数

```
USAGE:
    screen_recorder [OPTIONS] --output <OUTPUT>

OPTIONS:
  -o, --output <OUTPUT>           输出文件路径或 RTMP 流地址 [必需]
  -d, --device <DEVICE>           屏幕捕获设备 [default: desktop]
      --fps <FPS>                 帧率 (FPS) [default: 30]
  -r, --resolution <RESOLUTION>   分辨率 WIDTHxHEIGHT [default: 1920x1080]
  -b, --bitrate <BITRATE>         比特率 (5000k, 1M) [default: 5000k]
  -c, --codec <CODEC>             编码器 (h264, h265, libx264, libx265) [default: h264]
  -t, --duration <DURATION>       录制时间 (秒，0 表示无限制) [default: 0]
      --stream                    启用推流模式
      --log-level <LOG_LEVEL>     日志级别 (trace, debug, info, warn, error) [default: info]
  -h, --help                      打印帮助信息
  -V, --version                   打印版本信息
```

## 🏗️ 项目结构

```
screen_recorder_ffmpeg/
├── src/
│   ├── main.rs              # 主程序入口
│   ├── config.rs            # 配置管理
│   ├── error.rs             # 错误定义
│   ├── ffmpeg_encoder.rs    # FFmpeg 编码器
│   ├── screen_capture.rs    # 屏幕捕获模块
│   └── stream.rs            # 推流模块
├── Cargo.toml               # 项目配置
├── Cargo.lock               # 依赖锁定
├── README.md                # 项目文档
└── LICENSE                  # MIT 许可证
```

## 🔧 技术栈

- **语言**: Rust
- **异步运行时**: Tokio
- **FFmpeg 绑定**: ffmpeg-next
- **CLI 解析**: clap
- **日志**: log、tracing
- **错误处理**: anyhow、thiserror

## 📊 编码建议

### 本地录制
```bash
# 高质量录制
--codec h264 --bitrate 8000k --fps 60

# 平衡质量和文件大小
--codec h264 --bitrate 5000k --fps 30

# 高压缩率
--codec h265 --bitrate 3000k --fps 30
```

### 推流配置
```bash
# 网络直播 (推荐)
--fps 30 --bitrate 5000k --codec h264

# 高清直播
--fps 60 --bitrate 8000k --codec h264

# 低带宽直播
--fps 24 --bitrate 2500k --codec h264
```

## 🔍 故障排除

### FFmpeg 未找到
```
错误: FFmpeg 初始化失败

解决方案:
1. 确认 FFmpeg 已安装: ffmpeg -version
2. 将 FFmpeg 路径添加到系统 PATH
3. 在 Linux 上: apt-get install ffmpeg
```

### 推流失败
```
错误: 推流错误: Connection refused

解决方案:
1. 检查 RTMP 服务器是否在线
2. 验证 RTMP 地址格式
3. 检查防火墙设置
4. 查看日志信息: --log-level debug
```

### 性能问题
```
- 使用硬件加速: --codec nvenc (NVIDIA) 或 qsv (Intel)
- 降低分辨率: --resolution 1280x720
- 降低帧率: --fps 24
- 降低比特率: --bitrate 3000k
```

## 📈 性能指标

- **最大分辨率**: 4K (3840x2160)
- **最大帧率**: 120 FPS
- **编码延迟**: 100-500ms (取决于设置)
- **CPU 占用**: 20-60% (H.264, 1080p30)
- **内存占用**: 200-500MB

## 🎓 学习资源

- [FFmpeg 官网](https://ffmpeg.org/)
- [FFmpeg 命令行手册](https://ffmpeg.org/ffmpeg.html)
- [RTMP 协议文档](https://rtmp.veriskope.com/docs/spec/)
- [H.264 编码指南](https://trac.ffmpeg.org/wiki/Encode/H.264)

## 📝 常见问题

**Q: 如何录制音频?**
A: 目前版本还未完全集成音频。需要手动添加:
```bash
ffmpeg -f gdigrab -i desktop -f dshow -i audio="Microphone" -c:v h264 -b:v 5000k -c:a aac output.mp4
```

**Q: 支持多显示器吗?**
A: Windows 支持通过 `--device display-nr` 参数选择。Linux 需要指定 X11 显示器 (`:0`, `:1`)。

**Q: 能否实时调整比特率?**
A: 当前版本不支持，但可以启动多个实例使用不同参数。

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

MIT 许可证

## 👨‍💻 作者

Screen Recorder - FFmpeg-based screen recording tool

---

**⭐ 如果有帮助，请给个 Star！**
