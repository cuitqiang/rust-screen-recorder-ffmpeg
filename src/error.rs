use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum RecorderError {
    #[error("无效的分辨率格式: {0}，应为 WIDTHxHEIGHT")]
    InvalidResolution(String),

    #[error("无效的 FPS: {0}，应在 1-120 之间")]
    InvalidFps(u32),

    #[error("不支持的编码器: {0}")]
    UnsupportedCodec(String),

    #[error("无效的流地址: {0}，应以 rtmp:// 开头")]
    InvalidStreamUrl(String),

    #[error("输出路径为空")]
    EmptyOutput,

    #[error("FFmpeg 初始化失败: {0}")]
    FFmpegInitError(String),

    #[error("屏幕捕获失败: {0}")]
    CaptureError(String),

    #[error("编码错误: {0}")]
    EncodingError(String),

    #[error("推流错误: {0}")]
    StreamError(String),

    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),

    #[error("系统错误: {0}")]
    SystemError(String),
}
