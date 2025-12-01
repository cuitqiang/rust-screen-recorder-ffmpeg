use crate::config::{RecorderConfig, StreamProtocol};
use anyhow::Result;
use log::{info, warn};
use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::time::sleep;
use std::io::{BufRead, BufReader};

pub async fn start_streaming(config: RecorderConfig) -> Result<()> {
    info!("🌐 开始推流到: {}", config.output);

    let input_format = config.get_input_format();
    let device = config.get_capture_device();

    // 构建推流命令
    let mut ffmpeg_cmd = build_streaming_command(&config, input_format, &device)?;

    info!("📝 FFmpeg 推流命令: {:?}", ffmpeg_cmd);

    // 执行 FFmpeg
    let mut child = ffmpeg_cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // 将 FFmpeg stderr 打印到日志，便于调试连接错误
    if let Some(stderr) = child.stderr.take() {
        std::thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                if let Ok(l) = line {
                    log::info!("ffmpeg | {}", l);
                }
            }
        });
    }

    // 推流监控
    let start_time = std::time::Instant::now();
    let mut stats_interval = tokio::time::interval(Duration::from_secs(10));

    if config.duration > 0 {
        let duration = Duration::from_secs(config.duration as u64);
        info!("⏱️  推流时间限制: {:?}", duration);

        loop {
            tokio::select! {
                _ = sleep(duration) => {
                    info!("⏹️  停止推流...");
                    child.kill()?;
                    break;
                }
                _ = stats_interval.tick() => {
                    let elapsed = start_time.elapsed();
                    info!("📊 推流统计: 已运行 {:?}", elapsed);
                }
                _ = async { child.wait() } => {
                    break;
                }
            }
        }
    } else {
        info!("⏳ 无时间限制，按 Ctrl+C 停止推流");
        loop {
            tokio::select! {
                _ = stats_interval.tick() => {
                    let elapsed = start_time.elapsed();
                    info!("📊 推流统计: 已运行 {:?}", elapsed);
                }
                status = async { child.wait() } => {
                    let _ = status?;
                    break;
                }
            }
        }
    }

    info!("✅ 推流已停止");
    Ok(())
}

fn build_streaming_command(config: &RecorderConfig, input_format: &str, device: &str) -> Result<Command> {
    let mut cmd = Command::new("ffmpeg");

    // 输入参数
    cmd.arg("-f").arg(input_format);
    cmd.arg("-framerate").arg(config.fps.to_string());

    // 为 gdigrab/x11grab 指定采集尺寸，避免高分辨率原始输入导致编码或播放异常
    if let Ok((w, h)) = config.get_resolution_parts() {
        if input_format == "gdigrab" || input_format == "x11grab" {
            // 如果不绘制鼠标，则在 gdigrab 上关闭鼠标绘制
            if input_format == "gdigrab" && !config.draw_mouse {
                cmd.arg("-draw_mouse").arg("0");
            }
            cmd.arg("-video_size").arg(format!("{}x{}", w, h));
        }
    }

    cmd.arg("-i").arg(device);

    // 音频参数（可选）：当用户启用音频时，尝试探测并添加音频输入
    if config.audio_enabled {
        let audio_dev = config.audio_device.clone().unwrap_or_else(|| "Microphone".to_string());
        // Windows 使用 dshow
        #[cfg(windows)]
        {
            if crate::ffmpeg_encoder::probe_audio_device(&audio_dev) {
                cmd.arg("-f").arg("dshow")
                    .arg("-i")
                    .arg(format!("audio=\"{}\"", audio_dev));
            } else {
                warn!("未检测到音频设备 '{}'，将跳过音频采集", audio_dev);
            }
        }

        // Linux: 使用 PulseAudio 的 default 名称（如需自定义，传入 audio_device）
        #[cfg(target_os = "linux")]
        {
            let dev = audio_dev.clone();
            // 简单尝试，假设 PulseAudio/ALSA 可用；未探测则仍加入
            cmd.arg("-f").arg("pulse").arg("-i").arg(dev);
        }

        // macOS: avfoundation 需要索引或名称，如用户提供则尝试使用
        #[cfg(target_os = "macos")]
        {
            // macOS 音频输入通常为 avfoundation 索引，用户需传入正确值
            if !audio_dev.is_empty() {
                cmd.arg("-f").arg("avfoundation").arg("-i").arg(format!(":{}", audio_dev));
            }
        }
    }

    // 视频缩放与像素格式（在所有输入之后，编码参数之前）
    if let Ok((w, h)) = config.get_resolution_parts() {
        // 使用 filter 同时设定分辨率与像素格式，避免编码器使用不兼容的色彩空间
        cmd.arg("-vf").arg(format!("scale={}:{}:flags=lanczos,format=yuv420p", w, h));
    } else {
        // 即使未能获取分辨率，也强制像素格式
        cmd.arg("-vf").arg("format=yuv420p");
    }

    // map common codec names to ffmpeg encoder names
    let codec_name = match config.codec.as_str() {
        "h264" => "libx264",
        "h265" => "libx265",
        other => other,
    };

    // 视频编码参数（优化推流）
    cmd.arg("-c:v").arg(codec_name);
    cmd.arg("-b:v").arg(&config.bitrate);
    cmd.arg("-preset").arg("veryfast"); // 推流使用更快的预设
    cmd.arg("-maxrate").arg(&config.bitrate);
    cmd.arg("-bufsize").arg(&config.bitrate);

    // 根据协议添加不同的输出相关参数
    match config.protocol {
        StreamProtocol::RTMP => {
            // RTMP 使用 FLV 容器
            cmd.arg("-flvflags").arg("no_duration_filesize")
                .arg("-rtmp_live").arg("live")
                .arg(config.output.clone());
        }
        StreamProtocol::RTSP => {
            // RTSP 推流，使用 TCP 传输以提高可靠性
            cmd.arg("-rtsp_transport").arg("tcp");
            cmd.arg("-f").arg("rtsp");
            cmd.arg(config.output.clone());
        }
        StreamProtocol::File => {
            // 文件输出
            cmd.arg("-y").arg(config.output.clone());
        }
    }

    Ok(cmd)
}
