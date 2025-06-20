@echo off
setlocal enabledelayedexpansion

:: SpaceRock Quick Start Script for Windows
:: This script will download, build, and run the SpaceRock game

echo.
echo 🚀 SpaceRock Quick Start Script 🌌
echo ==================================
echo.

set "deps_ok=true"
set "port=8080"

:: Function to check if command exists
:command_exists
where "%~1" >nul 2>&1
exit /b %errorlevel%

:: Check for Rust installation
echo [INFO] Checking for Rust installation...
call :command_exists rustc
if !errorlevel! equ 0 (
    call :command_exists cargo
    if !errorlevel! equ 0 (
        for /f "tokens=*" %%i in ('rustc --version 2^>nul') do set "rust_version=%%i"
        echo [SUCCESS] Rust found: !rust_version!
    ) else (
        goto :rust_missing
    )
) else (
    :rust_missing
    echo [ERROR] Rust is not installed!
    echo.
    echo Please install Rust by:
    echo 1. Visit: https://rustup.rs/
    echo 2. Download and run rustup-init.exe
    echo 3. Restart your command prompt and run this script again
    echo.
    set "deps_ok=false"
)

:: Check for wasm-pack installation
echo [INFO] Checking for wasm-pack installation...
call :command_exists wasm-pack
if !errorlevel! equ 0 (
    for /f "tokens=*" %%i in ('wasm-pack --version 2^>nul') do set "wasm_version=%%i"
    echo [SUCCESS] wasm-pack found: !wasm_version!
) else (
    echo [WARNING] wasm-pack is not installed. Attempting to install...
    
    :: Try to install wasm-pack using cargo
    call :command_exists cargo
    if !errorlevel! equ 0 (
        echo [INFO] Installing wasm-pack via cargo...
        cargo install wasm-pack
        if !errorlevel! equ 0 (
            echo [SUCCESS] wasm-pack installed successfully!
        ) else (
            echo [ERROR] Failed to install wasm-pack automatically.
            echo Please install manually: https://rustwasm.github.io/wasm-pack/installer/
            set "deps_ok=false"
        )
    ) else (
        echo [ERROR] Cannot install wasm-pack automatically.
        echo Please install manually: https://rustwasm.github.io/wasm-pack/installer/
        set "deps_ok=false"
    )
)

:: Check for Git
echo [INFO] Checking for Git installation...
call :command_exists git
if !errorlevel! equ 0 (
    echo [SUCCESS] Git is available
) else (
    echo [ERROR] Git is not installed!
    echo Please install Git first: https://git-scm.com/downloads
    set "deps_ok=false"
)

:: Check for web server capability
echo [INFO] Checking for web server capability...
set "server_cmd="

:: Priority: Python 3 (most common) > PHP (often pre-installed) > Node.js (optional)

:: Check for Python 3
call :command_exists python
if !errorlevel! equ 0 (
    python -c "import sys; exit(0 if sys.version_info[0] == 3 else 1)" >nul 2>&1
    if !errorlevel! equ 0 (
        echo [SUCCESS] Python 3 found: will use built-in HTTP server
        set "server_cmd=python"
        goto :server_found
    )
)

:: Check for Python 3 explicitly
call :command_exists python3
if !errorlevel! equ 0 (
    echo [SUCCESS] Python 3 found: will use built-in HTTP server
    set "server_cmd=python3"
    goto :server_found
)

:: Check for PHP
call :command_exists php
if !errorlevel! equ 0 (
    echo [SUCCESS] PHP found: will use built-in server
    set "server_cmd=php"
    goto :server_found
)

:: Check for Node.js/npm (optional)
call :command_exists npm
if !errorlevel! equ 0 (
    echo [SUCCESS] Node.js/npm found: will use http-server (optional)
    set "server_cmd=npm"
    goto :server_found
)

:: No server found
echo [WARNING] No suitable web server found.
echo Please install Python 3 or PHP to run the local server.
echo Node.js is optional but can also provide a web server.
set "deps_ok=false"

:server_found

:: Check if all dependencies are available
if "%deps_ok%"=="false" (
    echo.
    echo [ERROR] Some dependencies are missing. Please install them and try again.
    pause
    exit /b 1
)

echo.
echo [SUCCESS] All dependencies are available!
echo.

:: Clone or update repository
set "repo_dir=spacerock"
if exist "%repo_dir%" (
    echo [INFO] Repository directory exists. Updating...
    cd "%repo_dir%"
    git pull origin main
    if !errorlevel! neq 0 (
        echo [ERROR] Failed to update repository
        pause
        exit /b 1
    )
) else (
    echo [INFO] Cloning SpaceRock repository...
    git clone https://github.com/enu235/spacerock.git "%repo_dir%"
    if !errorlevel! neq 0 (
        echo [ERROR] Failed to clone repository
        pause
        exit /b 1
    )
    cd "%repo_dir%"
)

echo [SUCCESS] Repository ready!
echo.

:: Build the project
echo [INFO] Building WebAssembly package...
wasm-pack build --target web

if !errorlevel! equ 0 (
    echo [SUCCESS] Build completed successfully!
) else (
    echo [ERROR] Build failed!
    pause
    exit /b 1
)

echo.
echo [SUCCESS] 🎮 SpaceRock is ready to play!
echo.

:: Start web server
echo [INFO] Starting web server on port %port%...
echo.
echo [WARNING] The game will open in your browser shortly...
echo [WARNING] Press Ctrl+C to stop the server when you're done playing.
echo.

:: Open browser after a short delay
start "" cmd /c "timeout /t 3 /nobreak >nul && echo [INFO] Opening browser... && start http://localhost:%port%"

:: Start appropriate server
if "%server_cmd%"=="python" (
    python -m http.server %port%
) else if "%server_cmd%"=="python3" (
    python3 -m http.server %port%
) else if "%server_cmd%"=="npm" (
    :: Install http-server if not present
    call :command_exists http-server
    if !errorlevel! neq 0 (
        echo [INFO] Installing http-server...
        npm install -g http-server
    )
    npx http-server -p %port% -o
) else if "%server_cmd%"=="php" (
    php -S localhost:%port%
) else (
    echo [ERROR] No web server available
    pause
    exit /b 1
)

pause 