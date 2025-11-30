use crate::config::RecorderConfig;
use anyhow::Result;
use log::{info, warn};
use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::time::sleep;

pub async fn start_recording(config: RecorderConfig) -> Result<()> {
    info!("🎥 开始屏幕录制...");

    let (width, height) = config.get_resolution_parts()?;
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
    cmd.arg("-i").arg(device);

    // 视频编码参数
    cmd.arg("-c:v").arg(&config.codec);
    cmd.arg("-b:v").arg(&config.bitrate);
    cmd.arg("-preset").arg("medium");

    // 音频参数 (可选)
    #[cfg(windows)]
    {
        cmd.arg("-f").arg("dshow")
            .arg("-i").arg("audio=\"Microphone\"");
    }

    // 其他参数
    cmd.arg("-y") // 覆盖输出文件
        .arg(&config.output);

    Ok(cmd)
}

/// 获取屏幕分辨率
#[cfg(windows)]
pub fn get_screen_resolution() -> Result<(u32, u32)> {
    use windows::Win32::System::Memory::*;
    use windows::Win32::Graphics::Gdi::*;
    use windows::Win32::Foundation::*;

    unsafe {
        let dc = GetDC(None);
        if dc.is_invalid() {
            return Err(anyhow::anyhow!("无法获取设备上下文"));
        }

        let width = GetDeviceCaps(dc, HORZRES) as u32;
        let height = GetDeviceCaps(dc, VERTRES) as u32;

        ReleaseDC(None, dc);

        info!("📐 屏幕分辨率: {}x{}", width, height);
        Ok((width, height))
    }
}

#[cfg(target_os = "linux")]
pub fn get_screen_resolution() -> Result<(u32, u32)> {
    // Linux 通过 xdpyinfo 获取分辨率
    let output = std::process::Command::new("xdpyinfo")
        .output()
        .map_err(|e| anyhow::anyhow!("获取分辨率失败: {}", e))?;

    let output_str = String::from_utf8(output.stdout)?;
    
    // 解析输出获取分辨率
    for line in output_str.lines() {
        if line.contains("dimensions") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok((w, h)) = parse_resolution(parts[1]) {
                    info!("📐 屏幕分辨率: {}x{}", w, h);
                    return Ok((w, h));
                }
            }
        }
    }

    Err(anyhow::anyhow!("无法解析屏幕分辨率"))
}

#[cfg(target_os = "macos")]
pub fn get_screen_resolution() -> Result<(u32, u32)> {
    // macOS 获取分辨率
    Err(anyhow::anyhow!("macOS 分辨率获取还未实现"))
}

fn parse_resolution(s: &str) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let parts: Vec<&str> = s.split('x').collect();
    Ok((parts[0].parse()?, parts[1].parse()?))
}
