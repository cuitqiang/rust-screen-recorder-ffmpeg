use crate::config::RecorderConfig;
use anyhow::Result;
use log::info;
use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::time::sleep;

pub async fn start_recording(config: RecorderConfig) -> Result<()> {
    info!("🎥 开始屏幕录制...");

    let (_width, _height) = config.get_resolution_parts()?;
    let input_format = config.get_input_format();
    let device = config.get_capture_device();

    // 构建 FFmpeg 命令
    let mut ffmpeg_cmd = build_ffmpeg_command(&config, input_format, &device)?;

    info!("📝 FFmpeg 命令: {:?}", ffmpeg_cmd);

    // 执行 FFmpeg
    let mut child = ffmpeg_cmd
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // 如果设置了时间限制，等待指定时间后终止
    if config.duration > 0 {
        let duration = Duration::from_secs(config.duration as u64);
        info!("⏱️  录制时间限制: {:?}", duration);

        sleep(duration).await;
        
        info!("⏸️  停止录制...");
        child.kill()?;
    } else {
        info!("⏳ 无时间限制，按 Ctrl+C 停止录制");
        let _ = child.wait()?;
    }

    info!("✅ 录制完成: {}", config.output);
    Ok(())
}

fn build_ffmpeg_command(config: &RecorderConfig, input_format: &str, device: &str) -> Result<Command> {
    let mut cmd = Command::new("ffmpeg");

    // 输入参数
    cmd.arg("-f").arg(input_format);
    cmd.arg("-framerate").arg(config.fps.to_string());

    // 指定采集尺寸，避免采集到超高分辨率导致编码或播放异常
    if let Ok((w, h)) = config.get_resolution_parts() {
        if input_format == "gdigrab" || input_format == "x11grab" {
            // 在 gdigrab 上可选择是否绘制鼠标指针
            if input_format == "gdigrab" && !config.draw_mouse {
                cmd.arg("-draw_mouse").arg("0");
            }
            cmd.arg("-video_size").arg(format!("{}x{}", w, h));
        }
    }

    cmd.arg("-i").arg(device);

    // 音频参数 (可选) - 在输入之后、编码参数之前添加音频输入
    if config.audio_enabled {
        let audio_dev = config.audio_device.clone().unwrap_or_else(|| "Microphone".to_string());
        #[cfg(windows)]
        {
            if crate::ffmpeg_encoder::probe_audio_device(&audio_dev) {
                cmd.arg("-f").arg("dshow");
                cmd.arg("-i").arg(format!("audio=\"{}\"", audio_dev));
            } else {
                log::warn!("未检测到音频设备 '{}'，跳过音频采集", audio_dev);
            }
        }

        #[cfg(target_os = "linux")]
        {
            cmd.arg("-f").arg("pulse").arg("-i").arg(audio_dev);
        }

        #[cfg(target_os = "macos")]
        {
            if !audio_dev.is_empty() {
                cmd.arg("-f").arg("avfoundation").arg("-i").arg(format!(":{}", audio_dev));
            }
        }
    }

    // 视频缩放与像素格式
    if let Ok((w, h)) = config.get_resolution_parts() {
        cmd.arg("-vf").arg(format!("scale={}:{}:flags=lanczos,format=yuv420p", w, h));
    } else {
        cmd.arg("-vf").arg("format=yuv420p");
    }

    // map codec
    let codec_name = match config.codec.as_str() {
        "h264" => "libx264",
        "h265" => "libx265",
        other => other,
    };

    // 视频编码参数
    cmd.arg("-c:v").arg(codec_name);
    cmd.arg("-b:v").arg(&config.bitrate);
    cmd.arg("-preset").arg("medium");

    // 其他参数
    cmd.arg("-y") // 覆盖输出文件
        .arg(&config.output);

    Ok(cmd)
}
