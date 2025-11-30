use crate::config::RecorderConfig;
use anyhow::Result;
use log::{info, warn};
use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::time::sleep;

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
    cmd.arg("-i").arg(device);

    // 视频编码参数（优化推流）
    cmd.arg("-c:v").arg(&config.codec);
    cmd.arg("-b:v").arg(&config.bitrate);
    cmd.arg("-preset").arg("veryfast"); // 推流使用更快的预设
    cmd.arg("-maxrate").arg(&config.bitrate);
    cmd.arg("-bufsize").arg(format!("{}x2", config.bitrate));

    // 音频参数（如果需要）
    #[cfg(windows)]
    {
        cmd.arg("-f").arg("dshow")
            .arg("-i").arg("audio=\"Microphone\"");
    }

    // 其他参数
    cmd.arg("-flvflags").arg("no_duration_filesize") // RTMP/FLV 参数
        .arg("-rtmp_live").arg("live")
        .arg(config.output.clone());

    Ok(cmd)
}

/// 检测 RTMP 服务器连接
pub async fn check_rtmp_server(url: &str) -> Result<bool> {
    info!("🔍 检测 RTMP 服务器: {}", url);
    
    // 简单的连接检测
    // 注意: 这里只是一个占位符，实际需要实现 RTMP 握手协议
    
    Ok(true)
}

/// 获取推流统计信息
pub struct StreamStats {
    pub fps: f32,
    pub bitrate: u32,
    pub time: Duration,
}

impl StreamStats {
    pub fn from_ffmpeg_output(output: &str) -> Option<Self> {
        // 解析 FFmpeg 输出获取实时统计
        // 示例: "frame= 250 fps= 30 q=-1.0 Lsize=N/A time=00:00:08.33 bitrate=N/A"
        
        let mut fps = 30.0;
        let mut bitrate = 5000u32;
        let mut seconds = 0u64;

        for part in output.split_whitespace() {
            if let Some(val) = part.strip_prefix("fps=") {
                fps = val.parse().unwrap_or(30.0);
            } else if let Some(val) = part.strip_prefix("bitrate=") {
                if let Ok(b) = val.trim_end_matches("kbits/s").parse::<u32>() {
                    bitrate = b;
                }
            } else if let Some(val) = part.strip_prefix("time=") {
                // 简单解析 HH:MM:SS 格式
                if let Some(last_part) = val.split(':').last() {
                    seconds = last_part.parse().unwrap_or(0);
                }
            }
        }

        Some(StreamStats {
            fps,
            bitrate,
            time: Duration::from_secs(seconds),
        })
    }
}
