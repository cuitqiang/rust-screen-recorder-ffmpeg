use crate::error::RecorderError;
use anyhow::Result;

#[derive(Clone, Debug)]
pub struct RecorderConfig {
    pub output: String,
    pub device: String,
    pub fps: u32,
    pub resolution: String,
    pub bitrate: String,
    pub codec: String,
    pub duration: u32,
    pub is_stream: bool,
}

impl RecorderConfig {
    pub fn validate(&self) -> Result<()> {
        // 验证分辨率格式
        if !self.resolution.contains('x') {
            return Err(RecorderError::InvalidResolution(self.resolution.clone()).into());
        }

        // 验证 FPS
        if self.fps == 0 || self.fps > 120 {
            return Err(RecorderError::InvalidFps(self.fps).into());
        }

        // 验证编码器
        match self.codec.as_str() {
            "h264" | "h265" | "libx264" | "libx265" => {}
            _ => return Err(RecorderError::UnsupportedCodec(self.codec.clone()).into()),
        }

        // 验证输出路径或 RTMP 地址
        if self.is_stream {
            if !self.output.starts_with("rtmp://") {
                return Err(RecorderError::InvalidStreamUrl(self.output.clone()).into());
            }
        } else {
            if self.output.is_empty() {
                return Err(RecorderError::EmptyOutput.into());
            }
        }

        Ok(())
    }

    pub fn get_resolution_parts(&self) -> Result<(u32, u32)> {
        let parts: Vec<&str> = self.resolution.split('x').collect();
        if parts.len() != 2 {
            return Err(RecorderError::InvalidResolution(self.resolution.clone()).into());
        }

        let width: u32 = parts[0].parse()?;
        let height: u32 = parts[1].parse()?;

        Ok((width, height))
    }

    pub fn get_capture_device(&self) -> String {
        #[cfg(windows)]
        {
            if self.device == "desktop" {
                "desktop".to_string()
            } else {
                self.device.clone()
            }
        }

        #[cfg(target_os = "linux")]
        {
            if self.device == "desktop" {
                ":0".to_string()
            } else {
                self.device.clone()
            }
        }

        #[cfg(target_os = "macos")]
        {
            "1".to_string() // macOS 屏幕编号
        }

        #[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
        {
            self.device.clone()
        }
    }

    pub fn get_input_format(&self) -> &'static str {
        #[cfg(windows)]
        {
            "gdigrab"
        }

        #[cfg(target_os = "linux")]
        {
            "x11grab"
        }

        #[cfg(target_os = "macos")]
        {
            "avfoundation"
        }

        #[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
        {
            "unknown"
        }
    }
}
