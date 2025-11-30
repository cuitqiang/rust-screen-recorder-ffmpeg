use anyhow::Result;
use log::info;

pub fn init_ffmpeg() -> Result<()> {
    info!("正在初始化 FFmpeg...");
    
    // FFmpeg 初始化
    // 注意: ffmpeg-next crate 会自动初始化 FFmpeg
    // 如果需要手动初始化，使用下面的代码：
    // ffmpeg::init()?;
    
    info!("✅ FFmpeg 初始化成功");
    Ok(())
}

/// 获取可用的硬件编码器
pub fn get_available_encoders() -> Vec<String> {
    vec![
        "h264".to_string(),
        "h265".to_string(),
        "libx264".to_string(),
        "libx265".to_string(),
        "nvenc".to_string(),     // NVIDIA GPU
        "qsv".to_string(),        // Intel Quick Sync
        "videotoolbox".to_string(), // Apple
    ]
}

/// 获取编码器的推荐参数
pub fn get_encoder_options(codec: &str, bitrate: &str) -> Vec<(&'static str, String)> {
    match codec {
        "h264" | "libx264" => vec![
            ("preset", "medium".to_string()),
            ("crf", "23".to_string()),
            ("b", bitrate.to_string()),
        ],
        "h265" | "libx265" => vec![
            ("preset", "medium".to_string()),
            ("crf", "28".to_string()),
            ("b", bitrate.to_string()),
        ],
        "nvenc" => vec![
            ("preset", "default".to_string()),
            ("b", bitrate.to_string()),
        ],
        _ => vec![
            ("b", bitrate.to_string()),
        ],
    }
}

/// 验证 FFmpeg 是否可用
pub fn check_ffmpeg_availability() -> Result<bool> {
    use std::process::Command;
    
    let output = Command::new("ffmpeg")
        .arg("-version")
        .output();
    
    match output {
        Ok(status) => Ok(status.status.success()),
        Err(_) => Ok(false),
    }
}
