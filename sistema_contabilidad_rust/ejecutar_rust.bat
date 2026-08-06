@echo off
title Sistema Contabilidad Rust
cd /d "%~dp0"

:: Verificar que Rust esta instalado
rustc --version >nul 2>&1
if %errorlevel% neq 0 (
    echo.
    echo [ERROR] Rust no esta instalado.
    echo.
    echo Para instalar Rust:
    echo   1. Ve a https://rustup.rs/
    echo   2. Descarga y ejecuta rustup-init.exe
    echo   3. Despues abre una terminal NUEVA y ejecuta este bat
    echo.
    pause
    exit /b 1
)

echo Compilando e iniciando Sistema Contabilidad Rust...
echo.
cargo run --release
if %errorlevel% neq 0 (
    echo.
    echo [ERROR] La compilacion fallo. Revisa los errores arriba.
    pause
)
