#Requires -RunAsAdministrator

Write-Host "Setting up OptiOS development environment for Windows..."

# Function to check if a command exists
function Test-CommandExists {
    param ($command)
    return (Get-Command $command -ErrorAction SilentlyContinue) -ne $null
}

# Winget check
if (-not (Test-CommandExists "winget")) {
    Write-Error "Winget is not installed or not in PATH. Please install App Installer from the Microsoft Store or ensure winget is in your PATH: https://aka.ms/getwinget"
    exit 1
}

# Install Rustup
Write-Host "Checking for Rustup..."
if (-not (Test-CommandExists "rustup")) {
    Write-Host "Rustup not found. Attempting to install Rustup via winget..."
    try {
        winget install --id Rustlang.Rustup -e --accept-package-agreements --accept-source-agreements
        Write-Host "Rustup installation requested via winget. Please follow any prompts."
        Write-Host "IMPORTANT: After Rustup installs, you may need to RESTART your terminal for 'rustup' to be available in PATH."
        Write-Host "Then, re-run the parts of this script for installing Rust components, or run them manually:"
        Write-Host "  rustup install nightly"
        Write-Host "  rustup component add rust-src"
    } catch {
        Write-Error "Winget command for Rustup failed: $($_.Exception.Message)"
        Write-Host "Please install Rustup manually from https://rustup.rs/"
    }
} else {
    Write-Host "Rustup already installed. Updating..."
    rustup update
}

# Install QEMU
Write-Host "Checking for QEMU..."
if (-not (Test-CommandExists "qemu-system-x86_64")) {
    Write-Host "QEMU not found. Attempting to install QEMU via winget..."
    try {
        winget install --id QEMU.QEMU -e --accept-package-agreements --accept-source-agreements
        Write-Host "QEMU installation requested via winget. Please follow any prompts."
    } catch {
        Write-Error "Winget command for QEMU failed: $($_.Exception.Message)"
        Write-Host "Please install QEMU manually from https://www.qemu.org/download/#windows"
    }
} else {
    Write-Host "QEMU (qemu-system-x86_64) already installed."
}

Write-Host ""
Write-Host "System dependencies check/installation initiated."
Write-Host "--------------------------------------------------------------------------------"
Write-Host "NEXT STEPS (if Rustup was newly installed, ensure it's in PATH first - restart terminal if needed):"
Write-Host "1. Install Rust Nightly and rust-src (if Rustup is installed and in PATH):"
Write-Host "   rustup install nightly"
Write-Host "   rustup component add rust-src"
Write-Host ""
Write-Host "2. Install x86_64-elf-gcc (Cross-compiler):"
Write-Host "   This script does not automatically install x86_64-elf-gcc due to complexity."
Write-Host "   Recommended options:"
Write-Host "   a) Use Windows Subsystem for Linux (WSL) and install 'gcc-x86-64-elf' via apt (see setup-linux.sh)."
Write-Host "   b) Download a prebuilt GCC toolchain for x86_64-elf. A common source is the xPack GCC:"
Write-Host "      https://github.com/xpack-dev-tools/x86_64-elf-gcc-xpack/releases/"
Write-Host "      Download the .zip, extract it, and add the 'bin' directory to your system PATH."
Write-Host "   c) Use MSYS2: Install MSYS2 (https://www.msys2.org/), then use pacman to install the toolchain:"
Write-Host "      pacman -S mingw-w64-x86_64-x86_64-elf-gcc mingw-w64-x86_64-x86_64-elf-binutils"
Write-Host "   Verify installation by opening a NEW terminal and typing: x86_64-elf-gcc --version"
Write-Host "--------------------------------------------------------------------------------"
Write-Host "Windows setup script complete!" 