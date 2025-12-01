@echo off
chcp 65001 >nul
title 屏幕录制工具 - 快速启动

echo.
echo ========================================
echo   🎥 屏幕录制工具 v1.0.0
echo   开发者：崔哥 @ 辰粤科技
echo ========================================
echo.
echo 请选择操作：
echo.
echo [1] 录制视频 (30秒，包含鼠标)
echo [2] 录制视频 (60秒，高性能无鼠标)
echo [3] RTSP 推流 (1小时)
echo [4] RTSP 推流 (无限时长)
echo [5] 自定义命令
echo [0] 退出
echo.
set /p choice=请输入选项 (0-5): 

if "%choice%"=="1" (
    echo.
    echo 📹 开始录制 30 秒视频（包含鼠标）...
    screen_recorder.exe --output video_%date:~0,4%%date:~5,2%%date:~8,2%_%time:~0,2%%time:~3,2%%time:~6,2%.mp4 --duration 30
    pause
    goto :start
)

if "%choice%"=="2" (
    echo.
    echo ⚡ 开始录制 60 秒视频（高性能，无鼠标）...
    screen_recorder.exe --output video_%date:~0,4%%date:~5,2%%date:~8,2%_%time:~0,2%%time:~3,2%%time:~6,2%.mp4 --duration 60 --no-mouse
    pause
    goto :start
)

if "%choice%"=="3" (
    echo.
    set /p rtsp_url=请输入 RTSP 地址 (默认: rtsp://127.0.0.1:8554/stream): 
    if "%rtsp_url%"=="" set rtsp_url=rtsp://127.0.0.1:8554/stream
    echo.
    echo 🌐 开始推流到 %rtsp_url% (1小时)...
    screen_recorder.exe --output %rtsp_url% --stream --duration 3600
    pause
    goto :start
)

if "%choice%"=="4" (
    echo.
    set /p rtsp_url=请输入 RTSP 地址 (默认: rtsp://127.0.0.1:8554/stream): 
    if "%rtsp_url%"=="" set rtsp_url=rtsp://127.0.0.1:8554/stream
    echo.
    echo 🌐 开始无限时长推流到 %rtsp_url%...
    echo 按 Ctrl+C 停止推流
    screen_recorder.exe --output %rtsp_url% --stream --duration 0
    pause
    goto :start
)

if "%choice%"=="5" (
    echo.
    echo 📝 自定义命令模式
    echo.
    set /p custom_cmd=请输入完整命令 (例: --output video.mp4 --duration 30): 
    echo.
    screen_recorder.exe %custom_cmd%
    pause
    goto :start
)

if "%choice%"=="0" (
    echo.
    echo 👋 再见！
    timeout /t 2 >nul
    exit
)

echo.
echo ❌ 无效选项，请重新选择！
timeout /t 2 >nul
:start
cls
goto :eof
