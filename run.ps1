# 快速运行脚本 - Screen Recorder FFmpeg
# 使用方法: .\run.ps1 [参数...]
# 示例: .\run.ps1 --output test.mp4 --duration 10

param(
    [Parameter(ValueFromRemainingArguments=$true)]
    [string[]]$Arguments
)

Write-Host "========================================" -ForegroundColor Cyan
Write-Host " Screen Recorder FFmpeg - 快速运行" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# 检查是否有参数
if ($Arguments.Count -eq 0) {
    Write-Host "使用示例:" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "  1. 录制到本地文件 (10秒):" -ForegroundColor Green
    Write-Host "     .\run.ps1 --output test.mp4 --duration 10" -ForegroundColor White
    Write-Host ""
    Write-Host "  2. 推流到 RTSP 服务器:" -ForegroundColor Green
    Write-Host "     .\run.ps1 --output rtsp://127.0.0.1:8554/stream --stream --duration 30" -ForegroundColor White
    Write-Host ""
    Write-Host "  3. 推流到 RTMP 服务器:" -ForegroundColor Green
    Write-Host "     .\run.ps1 --output rtmp://localhost/live/stream --stream" -ForegroundColor White
    Write-Host ""
    Write-Host "  4. 自定义分辨率和帧率:" -ForegroundColor Green
    Write-Host "     .\run.ps1 --output test.mp4 --resolution 1280x720 --fps 60 --duration 10" -ForegroundColor White
    Write-Host ""
    Write-Host "  5. 启用音频录制:" -ForegroundColor Green
    Write-Host "     .\run.ps1 --output test.mp4 --audio --audio-device Microphone --duration 10" -ForegroundColor White
    Write-Host ""
    Write-Host "  6. 使用 gdigrab 代替 Desktop Duplication API:" -ForegroundColor Green
    Write-Host "     .\run.ps1 --output test.mp4 --use-gdigrab --duration 10" -ForegroundColor White
    Write-Host ""
    Write-Host "更多参数请运行: cargo run -- --help" -ForegroundColor Cyan
    Write-Host ""
    exit 0
}

# 运行程序
Write-Host "启动程序..." -ForegroundColor Green
Write-Host "参数: $Arguments" -ForegroundColor Cyan
Write-Host ""

cargo run -- $Arguments
