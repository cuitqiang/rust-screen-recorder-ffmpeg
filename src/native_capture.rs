/// 使用 Desktop Duplication API 进行原生屏幕捕获
/// 相比 gdigrab，性能更好且无鼠标闪烁问题
use anyhow::{Result, Context};
use log::{info, warn, error};
use scrap::{Capturer, Display};
use std::io::Write;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use crate::config::RecorderConfig;

/// 原生捕获并通过管道传递给 FFmpeg
pub async fn start_native_capture_streaming(config: RecorderConfig) -> Result<()> {
    info!("🎯 使用 Desktop Duplication API 捕获屏幕");
    
    // 获取主显示器
    let display = Display::primary().context("无法获取主显示器")?;
    let (width, height) = (display.width(), display.height());
    
    info!("📺 显示器尺寸: {}x{}", width, height);
    
    // 创建捕获器
    let mut capturer = Capturer::new(display).context("无法创建屏幕捕获器")?;
    
    // 构建 FFmpeg 命令（从 stdin 读取原始帧）
    let mut ffmpeg_cmd = build_ffmpeg_pipe_command(&config, width, height)?;
    
    info!("📝 FFmpeg 命令: {:?}", ffmpeg_cmd);
    
    // 启动 FFmpeg 进程
    let mut child = ffmpeg_cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("无法启动 FFmpeg 进程")?;
    
    let mut stdin = child.stdin.take().context("无法获取 FFmpeg stdin")?;
    
    // 在单独线程中处理 FFmpeg stderr
    if let Some(stderr) = child.stderr.take() {
        thread::spawn(move || {
            use std::io::{BufRead, BufReader};
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                if let Ok(l) = line {
                    log::info!("ffmpeg | {}", l);
                }
            }
        });
    }
    
    let start_time = Instant::now();
    let target_frame_time = Duration::from_secs_f64(1.0 / config.fps as f64);
    let max_duration = if config.duration > 0 {
        Some(Duration::from_secs(config.duration as u64))
    } else {
        None
    };
    
    info!("🎬 开始捕获，目标帧率: {} FPS", config.fps);
    
    let mut frame_count = 0u64;
    let mut last_frame_time = Instant::now();
    let mut last_success_time = Instant::now();
    
    loop {
        // 检查是否超过时间限制
        if let Some(max_dur) = max_duration {
            if start_time.elapsed() >= max_dur {
                info!("⏹️  达到时间限制，停止捕获");
                break;
            }
        }
        
        // 捕获帧
        match capturer.frame() {
            Ok(frame) => {
                last_success_time = Instant::now();
                
                // 将 BGRA 帧数据写入 FFmpeg stdin
                if let Err(e) = stdin.write_all(&frame) {
                    error!("写入 FFmpeg 失败: {}", e);
                    break;
                }
                
                frame_count += 1;
                
                // 每 100 帧输出一次状态
                if frame_count % 100 == 0 {
                    let elapsed = start_time.elapsed().as_secs_f64();
                    let actual_fps = frame_count as f64 / elapsed;
                    info!("📊 已捕获 {} 帧，实际 FPS: {:.2}", frame_count, actual_fps);
                }
                
                // 精确的帧率控制
                let frame_elapsed = last_frame_time.elapsed();
                if frame_elapsed < target_frame_time {
                    let sleep_time = target_frame_time - frame_elapsed;
                    // 使用 spin_sleep 提高精度（小于 2ms 用 spin）
                    if sleep_time > Duration::from_millis(2) {
                        thread::sleep(sleep_time - Duration::from_millis(1));
                    }
                    // 剩余时间自旋等待，提高精度
                    while last_frame_time.elapsed() < target_frame_time {
                        thread::yield_now();
                    }
                }
                last_frame_time = Instant::now();
            }
            Err(e) => {
                // WouldBlock 表示当前没有新帧，这是正常现象，稍后重试
                if e.kind() == std::io::ErrorKind::WouldBlock {
                    // 如果超过 5 秒没有成功捕获，发出警告
                    if last_success_time.elapsed() > Duration::from_secs(5) {
                        warn!("已 5 秒未捕获到新帧，可能显示器进入休眠或捕获被阻塞");
                        last_success_time = Instant::now(); // 重置，避免重复警告
                    }
                    thread::sleep(Duration::from_micros(500));
                    continue;
                } else {
                    error!("捕获帧失败: {}", e);
                    break;
                }
            }
        }
    }
    
    // 关闭 stdin 以通知 FFmpeg 输入结束
    drop(stdin);
    
    info!("⏱️  总共捕获 {} 帧，耗时 {:.2}s", frame_count, start_time.elapsed().as_secs_f64());
    
    // 等待 FFmpeg 进程结束
    let status = child.wait()?;
    if !status.success() {
        warn!("FFmpeg 进程异常退出: {}", status);
    }
    
    Ok(())
}

/// 构建从管道读取的 FFmpeg 命令
fn build_ffmpeg_pipe_command(config: &RecorderConfig, width: usize, height: usize) -> Result<Command> {
    let mut cmd = Command::new("ffmpeg");
    
    // 从 stdin 读取原始 BGRA 帧
    cmd.arg("-f").arg("rawvideo");
    cmd.arg("-pix_fmt").arg("bgra");
    cmd.arg("-video_size").arg(format!("{}x{}", width, height));
    cmd.arg("-framerate").arg(config.fps.to_string());
    cmd.arg("-i").arg("pipe:0");
    
    // 添加音频输入（如果启用）
    if config.audio_enabled {
        #[cfg(target_os = "windows")]
        {
            let audio_device = config.audio_device.as_deref().unwrap_or("Microphone");
            cmd.arg("-f").arg("dshow");
            cmd.arg("-i").arg(format!("audio={}", audio_device));
        }
        
        #[cfg(target_os = "linux")]
        {
            let audio_device = config.audio_device.as_deref().unwrap_or("default");
            cmd.arg("-f").arg("pulse");
            cmd.arg("-i").arg(audio_device);
        }
        
        #[cfg(target_os = "macos")]
        {
            let audio_device = config.audio_device.as_deref().unwrap_or(":0");
            cmd.arg("-f").arg("avfoundation");
            cmd.arg("-i").arg(audio_device);
        }
    }
    
    // 视频编码参数
    let codec = if config.codec == "h264" {
        "libx264"
    } else if config.codec == "h265" {
        "libx265"
    } else {
        &config.codec
    };
    
    cmd.arg("-c:v").arg(codec);
    cmd.arg("-b:v").arg(&config.bitrate);
    cmd.arg("-preset").arg("veryfast");
    cmd.arg("-maxrate").arg(&config.bitrate);
    cmd.arg("-bufsize").arg(&config.bitrate);
    
    // 像素格式转换（BGRA -> YUV420P）
    cmd.arg("-pix_fmt").arg("yuv420p");
    
    // 音频编码参数（如果启用）
    if config.audio_enabled {
        cmd.arg("-c:a").arg("aac");
        cmd.arg("-b:a").arg("128k");
        cmd.arg("-ar").arg("44100");
    }
    
    // 输出格式和地址
    match config.protocol {
        crate::config::StreamProtocol::RTSP => {
            cmd.arg("-rtsp_transport").arg("tcp");
            cmd.arg("-f").arg("rtsp");
        }
        crate::config::StreamProtocol::RTMP => {
            cmd.arg("-f").arg("flv");
            cmd.arg("-flvflags").arg("no_duration_filesize");
            cmd.arg("-rtmp_live").arg("live");
        }
        crate::config::StreamProtocol::File => {
            cmd.arg("-f").arg("mp4");
            cmd.arg("-movflags").arg("faststart");
        }
    }
    
    cmd.arg(&config.output);
    
    // 覆盖已存在的文件
    if config.protocol == crate::config::StreamProtocol::File {
        cmd.arg("-y");
    }
    
    Ok(cmd)
}

/// 检查系统是否支持 Desktop Duplication API
pub fn is_desktop_duplication_available() -> bool {
    #[cfg(target_os = "windows")]
    {
        // 尝试创建捕获器来检测是否支持
        match Display::primary() {
            Ok(display) => {
                match Capturer::new(display) {
                    Ok(_) => true,
                    Err(_) => false,
                }
            }
            Err(_) => false,
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        // 非 Windows 平台使用 scrap 的默认实现
        Display::primary().is_ok()
    }
}
