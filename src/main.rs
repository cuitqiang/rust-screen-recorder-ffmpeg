mod screen_capture;
mod ffmpeg_encoder;
mod stream;
mod config;
mod error;
mod native_capture;

use anyhow::Result;
use clap::Parser;
use log::info;

use crate::config::{RecorderConfig, StreamProtocol};

#[derive(Parser, Debug)]
#[command(name = "Screen Recorder")]
#[command(about = "FFmpeg-based screen recorder and streaming tool", long_about = None)]
struct Args {
    /// 输出文件路径或 RTMP 流地址
    #[arg(short, long)]
    output: String,

    /// 屏幕捕获设备 (Windows: desktop, Linux: :0)
    #[arg(short = 'd', long, default_value = "desktop")]
    device: String,

    /// 帧率 (FPS)
    #[arg(short, long, default_value = "30")]
    fps: u32,

    /// 分辨率 (格式: WIDTHxHEIGHT，例如: 1920x1080)
    #[arg(short = 'r', long, default_value = "1920x1080")]
    resolution: String,

    /// 比特率 (格式: 5000k, 1M 等)
    #[arg(short = 'b', long, default_value = "5000k")]
    bitrate: String,

    /// 编码器 (h264, h265, libx264, libx265)
    #[arg(short = 'c', long, default_value = "h264")]
    codec: String,

    /// 录制时间 (秒，0 表示无限制)
    #[arg(short = 't', long, default_value = "0")]
    duration: u32,

    /// 是否为推流模式 (RTMP)
    #[arg(long, default_value = "false")]
    stream: bool,

    /// 是否启用音频采集
    #[arg(long, default_value_t = false)]
    audio: bool,

    /// 是否在采集时绘制鼠标指针。默认启用（视频中可见鼠标）。
    /// 注意：Windows gdigrab 捕获时本地鼠标可能会闪烁，这是正常现象，不影响录制质量。
    #[arg(long, default_value_t = true)]
    draw_mouse: bool,

    /// 音频设备名称（可选）
    #[arg(long)]
    audio_device: Option<String>,

    /// 使用 gdigrab 而不是 Desktop Duplication API（仅 Windows）
    #[arg(long, default_value_t = false)]
    use_gdigrab: bool,

    /// 日志级别 (trace, debug, info, warn, error)
    #[arg(long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // 初始化日志
    init_logger(&args.log_level);

    info!("🎬 屏幕录制器启动");
    info!("📹 输出: {}", args.output);
    info!("📊 分辨率: {}", args.resolution);
    info!("⏱️  帧率: {} FPS", args.fps);
    info!("🎥 比特率: {}", args.bitrate);
    info!("🔧 编码器: {}", args.codec);

    // 创建配置 (mutable 用于自动检测协议)
    let mut config = RecorderConfig {
        output: args.output.clone(),
        device: args.device.clone(),
        audio_enabled: args.audio,
        audio_device: args.audio_device.clone(),
        draw_mouse: args.draw_mouse,
        fps: args.fps,
        resolution: args.resolution.clone(),
        bitrate: args.bitrate.clone(),
        codec: args.codec.clone(),
        duration: args.duration,
        is_stream: args.stream,
        protocol: StreamProtocol::File,
    };

    // 根据输出自动检测协议 (RTMP / RTSP / File)
    config.detect_protocol();

    // 验证配置
    config.validate()?;

    // 初始化 FFmpeg
    info!("⚙️  初始化 FFmpeg...");
    ffmpeg_encoder::init_ffmpeg()?;

    // 根据模式选择操作
    if args.stream {
        info!("🌐 推流模式: {}", args.output);
        
        // 优先使用原生捕获（Desktop Duplication API），但需要鼠标时使用 gdigrab
        #[cfg(target_os = "windows")]
        {
            // 如果需要显示鼠标，使用 gdigrab（支持鼠标绘制）
            if args.draw_mouse && !args.use_gdigrab {
                info!("🖱️  需要显示鼠标，使用 gdigrab（包含鼠标指针）");
                stream::start_streaming(config).await?;
            } else if !args.use_gdigrab && native_capture::is_desktop_duplication_available() {
                info!("✨ 使用 Desktop Duplication API（高性能，无鼠标闪烁，但不显示鼠标）");
                native_capture::start_native_capture_streaming(config).await?;
            } else {
                if args.use_gdigrab {
                    info!("⚠️  使用 gdigrab 模式");
                } else {
                    info!("⚠️  Desktop Duplication API 不可用，回退到 gdigrab");
                }
                stream::start_streaming(config).await?;
            }
        }
        
        #[cfg(not(target_os = "windows"))]
        {
            // 非 Windows 平台尝试使用原生捕获
            if !args.use_gdigrab && native_capture::is_desktop_duplication_available() {
                info!("✨ 使用原生屏幕捕获");
                native_capture::start_native_capture_streaming(config).await?;
            } else {
                stream::start_streaming(config).await?;
            }
        }
    } else {
        info!("💾 录制模式: {}", args.output);
        
        // 录制模式也可以使用原生捕获
        #[cfg(target_os = "windows")]
        {
            // 如果需要显示鼠标，使用 gdigrab
            if args.draw_mouse && !args.use_gdigrab {
                info!("🖱️  需要显示鼠标，使用 gdigrab（包含鼠标指针）");
                screen_capture::start_recording(config).await?;
            } else if !args.use_gdigrab && native_capture::is_desktop_duplication_available() {
                info!("✨ 使用 Desktop Duplication API（高性能，无鼠标闪烁，但不显示鼠标）");
                native_capture::start_native_capture_streaming(config).await?;
            } else {
                if args.use_gdigrab {
                    info!("⚠️  使用 gdigrab 模式");
                } else {
                    info!("⚠️  Desktop Duplication API 不可用，回退到 gdigrab");
                }
                screen_capture::start_recording(config).await?;
            }
        }
        
        #[cfg(not(target_os = "windows"))]
        {
            if !args.use_gdigrab && native_capture::is_desktop_duplication_available() {
                info!("✨ 使用原生屏幕捕获");
                native_capture::start_native_capture_streaming(config).await?;
            } else {
                screen_capture::start_recording(config).await?;
            }
        }
    }

    info!("✅ 完成");
    Ok(())
}

fn init_logger(level: &str) {
    let level_filter = match level.to_lowercase().as_str() {
        "trace" => log::LevelFilter::Trace,
        "debug" => log::LevelFilter::Debug,
        "warn" => log::LevelFilter::Warn,
        "error" => log::LevelFilter::Error,
        _ => log::LevelFilter::Info,
    };

    env_logger::Builder::new()
        .filter_level(level_filter)
        .try_init()
        .ok();
}
