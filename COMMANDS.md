# 📝 命令参考手册

完整的命令行参数和使用示例。

---

## 🎯 基础命令

### 1. 录制视频（默认配置）
```powershell
# 录制 30 秒视频，包含鼠标
cargo run -- --output video.mp4 --duration 30
```
**特点**：1920x1080，30 FPS，5000k 比特率，显示鼠标

---

### 2. RTSP 推流
```powershell
# 推流到 RTSP 服务器（1小时）
cargo run -- --output rtsp://127.0.0.1:8554/stream --stream --duration 3600
```
**用途**：实时监控、远程查看

---

### 3. RTMP 推流
```powershell
# 推流到 RTMP 服务器
cargo run -- --output rtmp://127.0.0.1/live/stream --stream
```
**用途**：直播平台推流

---

### 4. 无限时长推流
```powershell
# 持续推流直到手动停止（Ctrl+C）
cargo run -- --output rtsp://127.0.0.1:8554/stream --stream --duration 0
```

---

## ⚡ 高性能模式

### 5. 高性能录制（无鼠标）
```powershell
# 使用 Desktop Duplication API，性能提升 30-50%
cargo run -- --output video.mp4 --duration 60 --no-mouse
```
**特点**：CPU 占用更低，无本地闪烁，但视频中不显示鼠标

---

### 6. 高性能推流（无鼠标）
```powershell
# RTSP 高性能推流
cargo run -- --output rtsp://127.0.0.1:8554/stream --stream --no-mouse --duration 3600
```

---

## 🎨 自定义参数

### 7. 自定义分辨率
```powershell
# 720p 录制
cargo run -- --output video.mp4 --resolution 1280x720 --duration 60

# 4K 录制
cargo run -- --output video.mp4 --resolution 3840x2160 --bitrate 15000k --duration 60
```

---

### 8. 自定义帧率
```powershell
# 60 FPS 高帧率录制
cargo run -- --output video.mp4 --fps 60 --bitrate 8000k --duration 60

# 15 FPS 低帧率（节省带宽）
cargo run -- --output video.mp4 --fps 15 --bitrate 2000k --duration 60
```

---

### 9. 自定义比特率
```powershell
# 高质量（10000k）
cargo run -- --output video.mp4 --bitrate 10000k --duration 60

# 低带宽（2000k）
cargo run -- --output video.mp4 --bitrate 2000k --duration 60
```

---

### 10. 自定义编码器
```powershell
# H.265/HEVC 编码（更高压缩率）
cargo run -- --output video.mp4 --codec h265 --duration 60

# 明确指定 libx264
cargo run -- --output video.mp4 --codec libx264 --duration 60
```

---

## 🎤 音频录制

### 11. 录制视频 + 音频
```powershell
# 自动检测默认麦克风
cargo run -- --output video.mp4 --audio --duration 60
```

---

### 12. 指定音频设备
```powershell
# 指定特定麦克风（Windows）
cargo run -- --output video.mp4 --audio --audio-device "麦克风 (Realtek High Definition Audio)" --duration 60

# 查看可用音频设备（手动运行 FFmpeg）
ffmpeg -list_devices true -f dshow -i dummy
```

---

## 🖱️ 鼠标控制

### 13. 强制显示鼠标（gdigrab）
```powershell
# 即使在特殊情况下也使用 gdigrab
cargo run -- --output video.mp4 --use-gdigrab --duration 60
```

---

### 14. 高性能无鼠标
```powershell
# Desktop Duplication API（无鼠标，高性能）
cargo run -- --output video.mp4 --no-mouse --duration 60
```

---

## 📊 日志控制

### 15. 详细调试日志
```powershell
# Debug 模式（查看详细信息）
cargo run -- --output video.mp4 --duration 30 --log-level debug
```

---

### 16. 安静模式
```powershell
# 只显示错误
cargo run -- --output video.mp4 --duration 30 --log-level error
```

---

### 17. Trace 模式（最详细）
```powershell
# 包含所有底层调用信息
cargo run -- --output video.mp4 --duration 30 --log-level trace
```

---

## 🎬 实际应用场景

### 18. 教程录制（推荐配置）
```powershell
# 1080p 30fps，包含鼠标，5分钟
cargo run -- --output tutorial.mp4 --duration 300
```

---

### 19. 游戏录制（高帧率）
```powershell
# 1080p 60fps，高比特率
cargo run -- --output gameplay.mp4 --fps 60 --bitrate 12000k --duration 600
```

---

### 20. 远程监控推流
```powershell
# 720p 15fps，低带宽，无鼠标，持续推流
cargo run -- --output rtsp://192.168.1.100:8554/monitor --stream --resolution 1280x720 --fps 15 --bitrate 2000k --no-mouse --duration 0
```

---

### 21. 会议录制（含音频）
```powershell
# 1080p 30fps，录制麦克风
cargo run -- --output meeting.mp4 --audio --duration 3600
```

---

### 22. 快速截屏视频
```powershell
# 5秒短视频
cargo run -- --output clip.mp4 --duration 5
```

---

### 23. 直播推流（RTMP）
```powershell
# 推流到 Nginx RTMP 服务器
cargo run -- --output rtmp://localhost/live/mystream --stream --bitrate 6000k
```

---

### 24. 4K 高清录制
```powershell
# 4K 30fps，高比特率
cargo run -- --output 4k_video.mp4 --resolution 3840x2160 --bitrate 20000k --duration 120 --no-mouse
```

---

### 25. 低延迟推流
```powershell
# 使用 veryfast preset，降低延迟
cargo run -- --output rtsp://127.0.0.1:8554/stream --stream --no-mouse
```

---

## 🔧 编译和运行

### 26. Debug 模式运行
```powershell
cargo run -- --output video.mp4 --duration 30
```

---

### 27. Release 模式（推荐生产环境）
```powershell
# 编译优化版本
cargo build --release

# 运行
.\target\release\screen_recorder.exe --output video.mp4 --duration 30
```

---

### 28. 快速运行脚本
```powershell
# 使用辅助脚本
.\run.ps1
```

---

### 29. 编译脚本
```powershell
# Debug 编译
.\build.ps1 debug

# Release 编译
.\build.ps1 release
```

---

## 📋 完整参数列表

| 参数 | 简写 | 默认值 | 说明 |
|------|------|--------|------|
| `--output` | `-o` | *必填* | 输出文件路径或流地址 |
| `--device` | `-d` | `desktop` | 捕获设备（Windows: desktop） |
| `--fps` | `-f` | `30` | 帧率（FPS） |
| `--resolution` | `-r` | `1920x1080` | 分辨率（WxH） |
| `--bitrate` | `-b` | `5000k` | 比特率 |
| `--codec` | `-c` | `h264` | 编码器（h264/h265/libx264/libx265） |
| `--duration` | `-t` | `0` | 录制时长（秒，0=无限） |
| `--stream` | - | `false` | 推流模式 |
| `--audio` | - | `false` | 启用音频 |
| `--audio-device` | - | `None` | 指定音频设备名称 |
| `--no-mouse` | - | `false` | 禁用鼠标（高性能模式） |
| `--use-gdigrab` | - | `false` | 强制使用 gdigrab |
| `--log-level` | - | `info` | 日志级别（trace/debug/info/warn/error） |

---

## 🆘 帮助命令

### 30. 查看帮助
```powershell
cargo run -- --help
```

---

## ⚠️ 注意事项

1. **RTSP/RTMP 推流前**：确保服务器已启动（如 MediaMTX、Nginx-RTMP）
2. **音频设备名称**：Windows 使用 dshow，Linux 使用 pulse，macOS 使用 avfoundation
3. **分辨率**：建议不超过实际屏幕分辨率
4. **比特率**：4K 建议 15000k+，1080p 建议 5000-10000k，720p 建议 2000-5000k
5. **编码器**：h264 更通用，h265 压缩率更高但兼容性较差
6. **鼠标显示**：
   - 默认模式（gdigrab）：显示鼠标，本地可能闪烁
   - 高性能模式（--no-mouse）：不显示鼠标，无闪烁

---

## 🚀 性能优化建议

### 低配置电脑
```powershell
# 降低分辨率和帧率
cargo run -- --output video.mp4 --resolution 1280x720 --fps 15 --bitrate 2000k --no-mouse --duration 60
```

### 高配置电脑
```powershell
# 4K 60fps
cargo run -- --output video.mp4 --resolution 3840x2160 --fps 60 --bitrate 25000k --duration 60
```

### 网络推流优化
```powershell
# 降低比特率和分辨率
cargo run -- --output rtsp://server/stream --stream --resolution 1280x720 --fps 25 --bitrate 3000k --no-mouse
```

---

**更多问题？查看：**
- [README.md](README.md) - 项目概览
- [USAGE.md](USAGE.md) - 详细使用指南
- [MOUSE.md](MOUSE.md) - 鼠标显示说明
