use crate::error::RecorderError;
use anyhow::Result;
use url::Url;

#[derive(Clone, Debug)]
pub struct RecorderConfig {
    pub output: String,
    pub device: String,
    /// 是否启用音频采集
    pub audio_enabled: bool,
    /// 可选音频设备名称（跨平台，Windows 示例: "Microphone"）
    pub audio_device: Option<String>,
    /// 是否在采集时绘制鼠标指针（仅对 gdigrab 生效）
    pub draw_mouse: bool,
    pub fps: u32,
    pub resolution: String,
    pub bitrate: String,
    pub codec: String,
    pub duration: u32,
    pub is_stream: bool,

    /// 流媒体协议类型
    pub protocol: StreamProtocol,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StreamProtocol {
    RTMP,
    RTSP,
    File,
}

impl RecorderConfig {
    pub fn validate(&self) -> Result<()> {
        // 验证分辨率格式
        self.get_resolution_parts()?;

        // 验证 FPS
        if self.fps < 1 || self.fps > 120 {
            return Err(RecorderError::InvalidFps(self.fps).into());
        }

        // 验证编码器
        let valid_codecs = ["h264", "h265", "libx264", "libx265", "nvenc", "qsv"];
        if !valid_codecs.contains(&self.codec.as_str()) {
            return Err(RecorderError::UnsupportedCodec(self.codec.clone()).into());
        }

        // 验证输出
        if self.output.is_empty() {
            return Err(RecorderError::EmptyOutput.into());
        }

        // 根据协议类型验证
        match self.protocol {
            StreamProtocol::RTMP => self.validate_rtmp_url()?,
            StreamProtocol::RTSP => self.validate_rtsp_url()?,
            StreamProtocol::File => {},
        }

        // 验证音频设备（如果启用）
        if self.audio_enabled {
            if let Some(dev) = &self.audio_device {
                if dev.trim().is_empty() {
                    return Err(RecorderError::SystemError("空的音频设备名称".to_string()).into());
                }
            }
        }

        Ok(())
    }

    fn validate_rtmp_url(&self) -> Result<()> {
        if !self.output.starts_with("rtmp://") && !self.output.starts_with("rtmps://") {
            return Err(RecorderError::InvalidStreamUrl(self.output.clone()).into());
        }
        match Url::parse(&self.output) {
            Ok(_) => Ok(()),
            Err(_) => Err(RecorderError::InvalidStreamUrl(self.output.clone()).into()),
        }
    }

    fn validate_rtsp_url(&self) -> Result<()> {
        if !self.output.starts_with("rtsp://") && !self.output.starts_with("rtsps://") {
            return Err(RecorderError::InvalidStreamUrl(self.output.clone()).into());
        }
        match Url::parse(&self.output) {
            Ok(_) => Ok(()),
            Err(_) => Err(RecorderError::InvalidStreamUrl(self.output.clone()).into()),
        }
    }

    pub fn detect_protocol(&mut self) {
        if self.output.starts_with("rtmp://") || self.output.starts_with("rtmps://") {
            self.protocol = StreamProtocol::RTMP;
        } else if self.output.starts_with("rtsp://") || self.output.starts_with("rtsps://") {
            self.protocol = StreamProtocol::RTSP;
        } else {
            self.protocol = StreamProtocol::File;
        }
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
