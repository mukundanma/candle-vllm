@echo off
setlocal

REM Install dependencies
winget install -e --id Microsoft.VisualStudio.2022.BuildTools
winget install -e --id Rustlang.Rustup-msvc

REM Build the project
cargo build --release
endlocal
