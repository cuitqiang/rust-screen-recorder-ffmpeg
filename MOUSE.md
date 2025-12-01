# 鼠标显示说明

## 🖱️ 鼠标显示的工作原理

### 两种捕获模式对比

| 特性 | Desktop Duplication API | gdigrab |
|------|------------------------|---------|
| **性能** | ⚡ 高性能（GPU加速） | ⚠️ 中等性能 |
| **本地鼠标闪烁** | ✅ 无闪烁 | ⚠️ 可能闪烁 |
| **视频中显示鼠标** | ❌ 不显示 | ✅ 显示 |
| **CPU占用** | 💚 5-15% | 💛 10-20% |
| **推荐场景** | 监控、无需鼠标的录制 | 教程、演示、需要鼠标的录制 |

---

## 📖 使用方法

### 1. 默认行为（显示鼠标）

程序默认使用 **gdigrab** 模式，鼠标会显示在视频中：

```powershell
# 录制视频（包含鼠标）
cargo run -- --output video.mp4 --duration 30

# 推流 RTSP（包含鼠标）
cargo run -- --output rtsp://127.0.0.1:8554/stream --stream --duration 3600

# 推流 RTMP（包含鼠标）
cargo run -- --output rtmp://127.0.0.1/live/stream --stream

# 无限时长推流
cargo run -- --output rtsp://127.0.0.1:8554/stream --stream --duration 0
```

**特点**：
- ✅ 鼠标在视频中可见
- ⚠️ 本地使用时鼠标可能轻微闪烁（不影响录制质量）

---

### 2. 不显示鼠标（高性能模式）

使用 Desktop Duplication API，性能更高但不显示鼠标：

```powershell
# 录制视频（无鼠标，高性能）
cargo run -- --output video.mp4 --duration 60 --no-mouse

# RTSP 推流（无鼠标，高性能）
cargo run -- --output rtsp://127.0.0.1:8554/stream --stream --duration 3600 --no-mouse

# RTMP 推流（无鼠标，高性能）
cargo run -- --output rtmp://127.0.0.1/live/stream --stream --no-mouse
```

**特点**：
- ✅ 无本地鼠标闪烁
- ✅ 性能更高（CPU 占用低 30-50%）
- ❌ 视频中不显示鼠标

---

### 3. 自定义参数

#### 自定义分辨率和帧率
```powershell
# 720p 60fps 录制
cargo run -- --output video.mp4 --resolution 1280x720 --fps 60 --bitrate 8000k --duration 120

# 4K 30fps 推流
cargo run -- --output rtsp://127.0.0.1:8554/stream --stream --resolution 3840x2160 --bitrate 15000k
```

#### 带音频录制
```powershell
# 录制视频 + 麦克风音频
cargo run -- --output video.mp4 --audio --duration 60

# 指定音频设备
cargo run -- --output video.mp4 --audio --audio-device "麦克风 (Realtek High Definition Audio)" --duration 60
```

#### 强制使用 gdigrab（有鼠标）
```powershell
cargo run -- --output video.mp4 --use-gdigrab --duration 30
```

#### 调整日志级别
```powershell
# 调试模式（详细日志）
cargo run -- --output video.mp4 --duration 30 --log-level debug

# 安静模式（只显示错误）
cargo run -- --output video.mp4 --duration 30 --log-level error
```

---

## 🎯 使用场景推荐

### 需要显示鼠标的场景
✅ **教程录制** - 展示操作步骤  
✅ **软件演示** - 演示功能使用  
✅ **游戏录制** - 游戏操作展示  

```powershell
# 默认配置即可，或明确指定
cargo run -- --output tutorial.mp4 --duration 300
cargo run -- --output demo.mp4 --use-gdigrab --duration 60
```

---

### 不需要显示鼠标的场景
✅ **监控录制** - 远程监控  
✅ **自动化测试** - 后台录制  
✅ **性能敏感场景** - CPU占用低  

```powershell
# 禁用鼠标显示
cargo run -- --output monitor.mp4 --no-mouse --duration 0
cargo run -- --output rtsp://192.168.1.100:8554/stream --stream --no-mouse
```

---

## 🐛 故障排除

### 问题1: 本地鼠标闪烁（使用 gdigrab 时）

这是 **Windows GDI 捕获的已知限制**，不影响录制质量。

**解决方案**：
```powershell
# 方案1: 改用 Desktop Duplication API（无鼠标）
cargo run -- --output video.mp4 --no-mouse --duration 30

# 方案2: 忽略闪烁（仅本地显示问题，录制文件正常）
cargo run -- --output video.mp4 --duration 30
```

---

### 问题2: Desktop Duplication API 不可用

**错误提示**：
```
⚠️  Desktop Duplication API 不可用，回退到 gdigrab
```

**原因**：
- 显卡驱动过旧
- 远程桌面会话中
- 虚拟机环境

**解决方案**：
```powershell
# 程序会自动回退到 gdigrab
cargo run -- --output video.mp4 --duration 30
```

---

### 问题3: 视频中看不到鼠标

**原因**：程序使用了 Desktop Duplication API

**解决方案**：
```powershell
# 确保没有禁用鼠标（默认启用）
cargo run -- --output video.mp4 --duration 30

# 或强制使用 gdigrab
cargo run -- --output video.mp4 --use-gdigrab --duration 30
```

---

## 💡 性能对比

### Desktop Duplication API（无鼠标）
```
CPU占用:   5-15%
延迟:      <10ms
闪烁:      无
鼠标显示:  ❌
```

### gdigrab（有鼠标）
```
CPU占用:   10-20%
延迟:      10-30ms
闪烁:      可能
鼠标显示:  ✅
```

---

## 📝 总结

- **默认行为**: 显示鼠标（使用 gdigrab）
- **高性能模式**: 使用 `--draw-mouse false`（无鼠标，无闪烁）
- **录制教程**: 保持默认设置即可
- **监控场景**: 使用 `--draw-mouse false`

**推荐配置**：
```powershell
# 教程录制（有鼠标）
cargo run -- --output tutorial.mp4 --duration 300

# 监控推流（无鼠标，高性能）
cargo run -- --output rtsp://127.0.0.1:8554/stream --stream --no-mouse
```
