# 打包脚本
# 用法: .\package.ps1

$version = "v1.0.0"
$packageName = "screen_recorder_$version"
$releaseDir = "release"
$zipFile = "$packageName.zip"

Write-Host "=== 开始打包 ===" -ForegroundColor Green
Write-Host ""

# 检查 release 目录
if (-not (Test-Path $releaseDir)) {
    Write-Host "[ERROR] 找不到 release 目录" -ForegroundColor Red
    exit 1
}

# 检查 EXE 文件
if (-not (Test-Path "$releaseDir\screen_recorder.exe")) {
    Write-Host "[ERROR] 找不到 screen_recorder.exe" -ForegroundColor Red
    exit 1
}

# 删除旧的压缩包
if (Test-Path $zipFile) {
    Remove-Item $zipFile -Force
    Write-Host "[INFO] 删除旧压缩包" -ForegroundColor Yellow
}

# 创建临时打包目录
$tempDir = "temp_package"
if (Test-Path $tempDir) {
    Remove-Item $tempDir -Recurse -Force
}
New-Item -ItemType Directory -Path $tempDir | Out-Null

# 复制文件
Write-Host "[INFO] 复制文件..." -ForegroundColor Cyan
Copy-Item "$releaseDir\screen_recorder.exe" "$tempDir\"
Copy-Item "$releaseDir\使用说明.txt" "$tempDir\"
Copy-Item "$releaseDir\快速启动.bat" "$tempDir\"
Copy-Item "README.md" "$tempDir\"
Copy-Item "LICENSE" "$tempDir\"

# 创建版本信息文件
@"
屏幕录制工具 $version
====================

构建时间: $(Get-Date -Format "yyyy-MM-dd HH:mm:ss")
平台: Windows x64
编译器: Rust (Release)

文件列表:
- screen_recorder.exe (主程序)
- 使用说明.txt (使用文档)
- 快速启动.bat (交互式启动)
- README.md (项目说明)
- LICENSE (MIT许可证)

⚠️ 重要提示:
程序依赖 FFmpeg，请确保：
1. FFmpeg 在系统 PATH 中，或
2. 将 ffmpeg.exe 放到程序同目录

下载 FFmpeg: https://github.com/BtbN/FFmpeg-Builds/releases

GitHub: https://github.com/cuitqiang/rust-screen-recorder-ffmpeg
作者: 崔哥 @ 辰粤科技
"@ | Out-File "$tempDir\版本信息.txt" -Encoding UTF8

# 压缩
Write-Host "[INFO] 压缩文件..." -ForegroundColor Cyan
Compress-Archive -Path "$tempDir\*" -DestinationPath $zipFile -Force

# 清理临时目录
Remove-Item $tempDir -Recurse -Force

# 显示结果
$zipSize = (Get-Item $zipFile).Length / 1MB
Write-Host ""
Write-Host "=== 打包完成！===" -ForegroundColor Green
Write-Host ""
Write-Host "文件名: $zipFile" -ForegroundColor Yellow
Write-Host "大小: $([math]::Round($zipSize, 2)) MB" -ForegroundColor Yellow
Write-Host ""
Write-Host "可以发布了！" -ForegroundColor Green
