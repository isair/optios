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
Write-Host "NEXT STEPS:"
Write-Host "1. Ensure OVMF files (UEFI firmware for QEMU) are accessible:"
Write-Host "   The 'make run' command (or direct QEMU execution) expects 'OVMF_CODE.fd' and"
Write-Host "   'OVMF_VARS.fd' to be in a 'qemu-testing/' directory at the root of your project."
Write-Host "   OVMF files are often NOT bundled with QEMU on Windows by default."
Write-Host "   You may need to download them separately. A common source for pre-built OVMF files is:"
Write-Host "     https://www.kraxel.org/repos/jenkins/edk2/ (look for edk2.git-ovmf-x64-*.noarch.rpm)"
Write-Host "     You can extract the .rpm (using 7-Zip) and find the .fd files within, typically in ./usr/share/edk2.git-ovmf-x64/."
Write-Host "     Place OVMF_CODE.fd and OVMF_VARS.fd into the 'qemu-testing/' directory."
Write-Host "     Alternatively, if your QEMU installation *did* include them, they might be in the QEMU installation directory (e.g., C:\Program Files\qemu\)."
Write-Host ""
Write-Host "2. Install Rust Nightly and rust-src (if Rustup is installed and in PATH - restart terminal if needed after Rustup install):"
Write-Host "   rustup install nightly"
Write-Host "   rustup component add rust-src"
Write-Host ""
Write-Host "3. Install x86_64-elf-gcc (Cross-compiler):"
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