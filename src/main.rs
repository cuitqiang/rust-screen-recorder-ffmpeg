mod screen_capture;
mod ffmpeg_encoder;
mod stream;
mod config;
mod error;

use anyhow::Result;
use clap::Parser;
use log::info;
use tracing_subscriber;

use crate::config::RecorderConfig;

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

    // 创建配置
    let config = RecorderConfig {
        output: args.output.clone(),
        device: args.device.clone(),
        fps: args.fps,
        resolution: args.resolution.clone(),
        bitrate: args.bitrate.clone(),
        codec: args.codec.clone(),
        duration: args.duration,
        is_stream: args.stream,
    };

    // 验证配置
    config.validate()?;

    // 初始化 FFmpeg
    info!("⚙️  初始化 FFmpeg...");
    ffmpeg_encoder::init_ffmpeg()?;

    // 根据模式选择操作
    if args.stream {
        info!("🌐 推流模式: {}", args.output);
        stream::start_streaming(config).await?;
    } else {
        info!("💾 录制模式: {}", args.output);
        screen_capture::start_recording(config).await?;
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
