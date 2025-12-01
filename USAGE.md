# Screen Recorder FFmpeg - 使用指南

## 📖 快速开始

### 方式一：使用便捷脚本（推荐）

#### 1. 构建项目
```powershell
# 调试版本（编译快）
.\build.ps1

# 发布版本（性能优化，推荐实际使用）
.\build.ps1 release
```

#### 2. 运行程序
```powershell
# 查看使用示例
.\run.ps1

# 录制 10 秒视频
.\run.ps1 --output test.mp4 --duration 10

# 推流到 RTSP
.\run.ps1 --output rtsp://127.0.0.1:8554/stream --stream --duration 30
```

### 方式二：直接使用 Cargo

```powershell
# 编译并运行
cargo run -- --output test.mp4 --duration 10

# 仅编译
cargo build --release

# 运行已编译的程序
.\target\release\screen_recorder.exe --output test.mp4 --duration 10
```

---

## 🎯 常用命令示例

### 📹 本地录制

#### 基础录制
```powershell
# 录制 10 秒（默认 1920x1080, 30fps, 5000k码率）
cargo run -- --output video.mp4 --duration 10

# 无限录制（按 Ctrl+C 停止）
cargo run -- --output video.mp4 --duration 0
```

#### 自定义参数
```powershell
# 高清 60fps 录制
cargo run -- --output video.mp4 --fps 60 --bitrate 8000k --duration 30

# 720p 录制（节省空间）
cargo run -- --output video.mp4 --resolution 1280x720 --bitrate 3000k --duration 30

# 使用 H.265 编码（更高压缩率）
cargo run -- --output video.mp4 --codec h265 --bitrate 3000k --duration 30
```

#### 启用音频
```powershell
# 录制音频（自动检测设备）
cargo run -- --output video.mp4 --audio --duration 30

# 指定音频设备
cargo run -- --output video.mp4 --audio --audio-device "立体声混音" --duration 30

# 查看可用音频设备（使用 FFmpeg）
ffmpeg -list_devices true -f dshow -i dummy
```

---

### 🌐 RTSP 推流

```powershell
# 推流到本地 RTSP 服务器（如 MediaMTX）
cargo run -- --output rtsp://127.0.0.1:8554/stream --stream --duration 60

# 推流到远程服务器
cargo run -- --output rtsp://192.168.1.100:8554/live --stream

# 无限推流
cargo run -- --output rtsp://127.0.0.1:8554/stream --stream --duration 0

# 使用 VLC 观看
# 打开 VLC -> 媒体 -> 打开网络串流 -> 输入 rtsp://127.0.0.1:8554/stream
```

---

### 📡 RTMP 推流

```powershell
# 推流到 RTMP 服务器
cargo run -- --output rtmp://localhost/live/stream --stream

# 推流到 Nginx-RTMP
cargo run -- --output rtmp://localhost:1935/live/stream --stream

# 推流到直播平台（示例）
cargo run -- --output rtmp://live.example.com/app/streamkey --stream
```

---

## ⚙️ 参数说明

| 参数 | 简写 | 默认值 | 说明 | 示例 |
|------|------|--------|------|------|
| `--output` | `-o` | 必需 | 输出路径或推流地址 | `test.mp4` |
| `--device` | `-d` | `desktop` | 捕获设备 | `desktop` |
| `--fps` | `-f` | `30` | 帧率 (FPS) | `60` |
| `--resolution` | `-r` | `1920x1080` | 分辨率 (WxH) | `1280x720` |
| `--bitrate` | `-b` | `5000k` | 视频码率 | `8000k`, `1M` |
| `--codec` | `-c` | `h264` | 编码器 | `h264`, `h265`, `libx264` |
| `--duration` | `-t` | `0` | 录制时长（秒），0=无限 | `10`, `60` |
| `--stream` | - | `false` | 启用推流模式 | - |
| `--audio` | - | `false` | 启用音频采集 | - |
| `--audio-device` | - | 无 | 音频设备名称 | `Microphone` |
| `--draw-mouse` | - | `true` | 视频中显示鼠标 | - |
| `--use-gdigrab` | - | `false` | 使用 gdigrab（回退） | - |
| `--log-level` | - | `info` | 日志级别 | `debug`, `trace` |

---

## 🎬 使用场景

### 场景 1: 教程录制（显示鼠标）
```powershell
# 默认配置，鼠标在视频中可见
cargo run -- --output tutorial.mp4 --audio --audio-device Microphone --duration 300
```

### 场景 2: 游戏录制（显示鼠标）
```powershell
# 高帧率、高码率录制，包含鼠标
cargo run -- --output gameplay.mp4 --fps 60 --bitrate 10000k --duration 600
```

### 场景 3: 远程监控（无鼠标，高性能）
```powershell
# 无鼠标显示，无本地闪烁，性能最佳
cargo run -- --output rtsp://192.168.1.100:8554/monitor --stream --draw-mouse false --duration 0
```

### 场景 4: 直播推流（显示鼠标）
```powershell
# 推流到直播平台，鼠标可见
cargo run -- --output rtmp://live.bilibili.com/live/your_key --stream --fps 30 --bitrate 6000k
```

### 场景 5: 桌面分享（显示鼠标）
```powershell
# 推流到本地，用 VLC/浏览器观看
cargo run -- --output rtsp://127.0.0.1:8554/desktop --stream
```

---

## 🔧 高级用法

### 调试模式
```powershell
# 启用详细日志
cargo run -- --output test.mp4 --log-level debug --duration 10

# 追踪所有日志
cargo run -- --output test.mp4 --log-level trace --duration 10
```

### 性能优化
```powershell
# 使用发布版本（编译时间长，但运行更快）
.\build.ps1 release
.\target\release\screen_recorder.exe --output test.mp4 --duration 10

# 降低分辨率和帧率（减少 CPU 占用）
cargo run -- --output test.mp4 --resolution 1280x720 --fps 24 --duration 10
```

### 回退到 gdigrab
```powershell
# 如果 Desktop Duplication API 不可用
cargo run -- --output test.mp4 --use-gdigrab --duration 10
```

---

## 📊 性能指标

### Desktop Duplication API（默认）
- **延迟**: < 10ms
- **CPU 占用**: 5-15% (1080p30)
- **稳定帧率**: 29.5-30.0 FPS
- **鼠标闪烁**: ✅ 无

### gdigrab（回退方案）
- **延迟**: 10-30ms
- **CPU 占用**: 10-20% (1080p30)
- **鼠标闪烁**: ⚠️ 可能存在

---

## 🐛 故障排除

### 问题 1: FFmpeg 找不到
```
错误: FFmpeg 初始化失败
```

**解决方案:**
```powershell
# 检查 FFmpeg 是否安装
ffmpeg -version

# 如果没有，使用 Chocolatey 安装
choco install ffmpeg

# 或使用 Scoop
scoop install ffmpeg
```

---

### 问题 2: Desktop Duplication API 不可用
```
错误: 无法创建屏幕捕获器
```

**解决方案:**
1. 确认使用 Windows 10/11
2. 更新显卡驱动
3. 尝试使用 `--use-gdigrab` 回退

```powershell
cargo run -- --output test.mp4 --use-gdigrab --duration 10
```

---

### 问题 3: 音频设备找不到
```
警告: 未检测到音频设备 'Microphone'
```

**解决方案:**
```powershell
# 查看可用音频设备
ffmpeg -list_devices true -f dshow -i dummy

# 使用正确的设备名称
cargo run -- --output test.mp4 --audio --audio-device "立体声混音" --duration 10
```

---

### 问题 4: RTSP 连接失败
```
错误: 推流失败
```

**解决方案:**
1. 确保 RTSP 服务器已启动（如 MediaMTX）
2. 检查防火墙设置
3. 使用 VLC 测试连接

```powershell
# 启动 MediaMTX（示例）
mediamtx.exe

# 使用 VLC 测试
vlc rtsp://127.0.0.1:8554/stream
```

---

## 📚 相关链接

- [FFmpeg 官网](https://ffmpeg.org/)
- [MediaMTX (RTSP服务器)](https://github.com/bluenviron/mediamtx)
- [Rust 官网](https://www.rust-lang.org/)
- [Scrap 库](https://github.com/quadrupleslap/scrap)

---

## 💡 提示

1. **首次运行时**，程序会自动检测并使用最佳捕获方式（Desktop Duplication API 优先）
2. **录制结束后**，视频文件会自动优化（faststart）以便快速播放
3. **推流时**，建议使用有线网络以保证稳定性
4. **本地录制**，建议使用 SSD 硬盘以避免写入瓶颈
5. **长时间录制**，注意磁盘空间（1080p30 约 2.3GB/小时）

---

**提示**: 程序默认使用 Desktop Duplication API，无鼠标闪烁且性能更好！
