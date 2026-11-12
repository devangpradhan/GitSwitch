@echo off
title GitSwitch v1.0.0 - Precision Git & SSH Switcher
if exist "%~dp0src-tauri\target\release\gitswitch.exe" (
    start "" "%~dp0src-tauri\target\release\gitswitch.exe"
) else (
    echo Native binary not found. Starting fallback web runner...
    node "%~dp0desktop-server.js"
    pause
)
