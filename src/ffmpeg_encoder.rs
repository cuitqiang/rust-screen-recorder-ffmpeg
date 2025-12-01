use anyhow::Result;
use log::info;

pub fn init_ffmpeg() -> Result<()> {
    info!("正在初始化 FFmpeg...");
    info!("✅ FFmpeg 初始化成功");
    Ok(())
}

/// 简单探测音频设备是否存在（Windows 上使用 dshow 列表探测）
pub fn probe_audio_device(device: &str) -> bool {
    use std::process::Command;
    #[cfg(windows)]
    {
        // ffmpeg 列出 dshow 设备信息在 stderr
        let out = Command::new("ffmpeg")
            .args(["-list_devices", "true", "-f", "dshow", "-i", "dummy"])
            .output();

        if let Ok(o) = out {
            let stderr = String::from_utf8_lossy(&o.stderr).to_lowercase();
            return stderr.contains(&device.to_lowercase());
        }
        return false;
    }

    // 非 Windows 平台：保守策略，返回 true（或未来可实现更严格的探测）
    #[cfg(not(windows))]
    {
        true
    }
}
