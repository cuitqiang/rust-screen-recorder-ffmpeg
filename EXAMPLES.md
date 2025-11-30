# 🎬 Screen Recorder - 使用示例

本文档提供详细的使用示例和最佳实践。

## 基础用法

### 1. 本地录制（最简单）

最简单的录制命令：
```bash
cargo run --release -- --output output.mp4
```

输出说明：
- 自动使用默认设置（1920x1080, 30 FPS, H.264）
- 按 Ctrl+C 停止录制

### 2. 指定输出文件

```bash
# 保存为不同格式
cargo run --release -- --output myscreen.mp4
cargo run --release -- --output myscreen.avi
cargo run --release -- --output myscreen.mov
cargo run --release -- --output myscreen.mkv
```

### 3. 设置录制时间

```bash
# 录制 10 秒
cargo run --release -- --output output.mp4 --duration 10

# 录制 1 分钟
cargo run --release -- --output output.mp4 --duration 60

# 录制 5 分钟
cargo run --release -- --output output.mp4 --duration 300
```

## 视频质量设置

### 高质量录制

适合后期处理和存档：
```bash
cargo run --release -- \
  --output high_quality.mp4 \
  --codec h264 \
  --bitrate 8000k \
  --fps 60 \
  --resolution 1920x1080
```

**参数说明：**
- `--bitrate 8000k` - 8000 kbps 高比特率
- `--fps 60` - 60 帧每秒
- `--resolution 1920x1080` - 全高清分辨率

### 平衡设置（推荐）

适合大多数用途：
```bash
cargo run --release -- \
  --output balanced.mp4 \
  --codec h264 \
  --bitrate 5000k \
  --fps 30 \
  --resolution 1920x1080
```

### 低比特率（节省空间）

适合长时间录制或网络传输：
```bash
cargo run --release -- \
  --output low_bitrate.mp4 \
  --codec h265 \
  --bitrate 2500k \
  --fps 24 \
  --resolution 1280x720
```

## 编码器选择

### H.264 (兼容性最好)
```bash
cargo run --release -- \
  --output output.mp4 \
  --codec h264 \
  --bitrate 5000k
```

**优点：** 最广泛支持、最高兼容性
**缺点：** 文件较大、编码速度较慢

### H.265 (最佳压缩)
```bash
cargo run --release -- \
  --output output.mp4 \
  --codec h265 \
  --bitrate 3000k
```

**优点：** 最好的压缩率、更小的文件
**缺点：** 支持不如 H.264 广泛

### 硬件加速 (NVIDIA NVENC)
```bash
cargo run --release -- \
  --output output.mp4 \
  --codec nvenc \
  --bitrate 5000k
```

**优点：** 最快的编码速度、低 CPU 占用
**缺点：** 需要 NVIDIA GPU

## 推流用法

### 推流到 RTMP 服务器

基础推流：
```bash
cargo run --release -- \
  --output rtmp://your-server.com/live/stream \
  --stream
```

### 推流到 YouTube Live

1. 获取 YouTube 流密钥
2. 运行命令：
```bash
cargo run --release -- \
  --output rtmp://a.rtmp.youtube.com/live2/YOUR_STREAM_KEY \
  --stream \
  --fps 30 \
  --bitrate 6000k \
  --codec h264
```

### 推流到 Twitch

```bash
cargo run --release -- \
  --output rtmp://live-sin.twitch.tv/app/YOUR_STREAM_KEY \
  --stream \
  --fps 60 \
  --bitrate 8000k \
  --codec h264
```

### 推流到本地 RTMP 服务器

```bash
# 假设本地 RTMP 服务器在 localhost:1935
cargo run --release -- \
  --output rtmp://localhost:1935/live/stream \
  --stream \
  --fps 30 \
  --bitrate 5000k
```

## 多屏幕录制

### Windows 多显示器

```bash
# 列出所有显示器
ffmpeg -f gdigrab -list_devices true -i dummy

# 录制特定显示器 (例如 1920x1080 的第二个屏幕)
cargo run --release -- \
  --output screen2.mp4 \
  --device "title=\" - 1920x1080\""
```

### Linux 多屏幕

```bash
# 列出所有 X11 显示
echo $DISPLAY

# 录制特定显示 (:0 或 :1)
cargo run --release -- \
  --output desktop.mp4 \
  --device ":1"
```

## 性能优化

### 低端 CPU 优化

```bash
# 使用更快的编码预设和较低的分辨率
cargo run --release -- \
  --output output.mp4 \
  --codec h264 \
  --bitrate 2000k \
  --fps 24 \
  --resolution 1280x720
```

### 高性能系统

```bash
# 使用最高质量设置
cargo run --release -- \
  --output output.mp4 \
  --codec h264 \
  --bitrate 10000k \
  --fps 60 \
  --resolution 3840x2160
```

### 使用多线程

```bash
# 设置环境变量增加线程数
$env:FFMPEG_THREADS=8
cargo run --release -- --output output.mp4
```

## 脚本化使用

### PowerShell 脚本

```powershell
# 每小时自动录制一次
while ($true) {
    $timestamp = Get-Date -Format "yyyy-MM-dd_HH-mm-ss"
    $output = "recordings\recording_$timestamp.mp4"
    
    Write-Host "开始录制: $output"
    cargo run --release -- `
      --output $output `
      --duration 3600
    
    Write-Host "录制完成"
}
```

### Bash 脚本 (Linux/macOS)

```bash
#!/bin/bash

# 每 30 秒录制一个 10 秒的视频
for i in {1..100}; do
    OUTPUT="recording_$(date +%Y%m%d_%H%M%S).mp4"
    echo "Recording: $OUTPUT"
    
    cargo run --release -- \
      --output $OUTPUT \
      --duration 10
    
    sleep 20
done
```

## 故障排除

### 录制不开始

检查 FFmpeg 是否安装：
```bash
ffmpeg -version
```

如果未安装，请根据你的操作系统安装 FFmpeg。

### 低性能/卡顿

尝试降低设置：
```bash
# 降低帧率
--fps 24

# 降低分辨率
--resolution 1280x720

# 降低比特率
--bitrate 3000k
```

### 推流连接超时

```bash
# 检查网络连接
ping your-server.com

# 增加日志级别调试
cargo run --release -- ... --log-level debug
```

## 高级技巧

### 录制后自动压缩

```bash
# 使用 H.265 减小文件大小
ffmpeg -i output_h264.mp4 -c:v libx265 -crf 28 output_h265.mp4
```

### 提取音频

```bash
ffmpeg -i recording.mp4 -q:a 0 -map a audio.mp3
```

### 添加水印

```bash
ffmpeg -i recording.mp4 -i watermark.png \
  -filter_complex "overlay=10:10" output_watermarked.mp4
```

### 创建GIF

```bash
ffmpeg -i recording.mp4 -vf "fps=10,scale=640:-1" output.gif
```

## 最佳实践

1. **测试设置** - 先用 `--duration 10` 测试参数
2. **监控资源** - 使用任务管理器/top 监控 CPU/内存
3. **定期备份** - 重要的录制应该备份
4. **合理编码** - 录制和推流使用不同的设置
5. **网络优化** - 推流时限制其他网络使用

---

更多帮助，请查看 [README.md](README.md)
