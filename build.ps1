# 构建脚本 - Screen Recorder FFmpeg
# 使用方法: .\build.ps1 [debug|release]

param(
    [string]$Mode = "debug"
)

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "   Screen Recorder FFmpeg - 构建工具" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# 检查 Rust 环境
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "❌ 错误: 未检测到 Cargo (Rust 工具链)" -ForegroundColor Red
    Write-Host "请先安装 Rust: https://rustup.rs/" -ForegroundColor Yellow
    exit 1
}

# 检查 FFmpeg
if (-not (Get-Command ffmpeg -ErrorAction SilentlyContinue)) {
    Write-Host "⚠️  警告: 未检测到 FFmpeg" -ForegroundColor Yellow
    Write-Host "运行时需要 FFmpeg，请确保已安装并添加到 PATH" -ForegroundColor Yellow
    Write-Host ""
}

# 显示构建信息
Write-Host "📦 构建模式: $Mode" -ForegroundColor Green
Write-Host "🔧 开始构建..." -ForegroundColor Green
Write-Host ""

# 执行构建
if ($Mode -eq "release") {
    cargo build --release
    if ($LASTEXITCODE -eq 0) {
        Write-Host ""
        Write-Host "✅ 构建成功！" -ForegroundColor Green
        Write-Host "📁 可执行文件位置: target\release\screen_recorder.exe" -ForegroundColor Cyan
        Write-Host ""
        Write-Host "运行示例:" -ForegroundColor Yellow
        Write-Host "  .\target\release\screen_recorder.exe --output test.mp4 --duration 10" -ForegroundColor White
    } else {
        Write-Host ""
        Write-Host "❌ 构建失败" -ForegroundColor Red
        exit 1
    }
} else {
    cargo build
    if ($LASTEXITCODE -eq 0) {
        Write-Host ""
        Write-Host "✅ 构建成功！" -ForegroundColor Green
        Write-Host "📁 可执行文件位置: target\debug\screen_recorder.exe" -ForegroundColor Cyan
        Write-Host ""
        Write-Host "运行示例:" -ForegroundColor Yellow
        Write-Host "  .\target\debug\screen_recorder.exe --output test.mp4 --duration 10" -ForegroundColor White
    } else {
        Write-Host ""
        Write-Host "❌ 构建失败" -ForegroundColor Red
        exit 1
    }
}
