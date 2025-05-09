#!/bin/bash

echo "Setting up OptiOS development environment for Linux (Debian/Ubuntu based)..."

# Check for sudo
if ! [ "$EUID" -eq 0 ]; then
  echo "Please run this script with sudo: sudo ./setup-linux.sh"
  exit 1
fi

# Update package lists
echo "Updating package lists..."
apt update -y

# Install necessary packages
echo "Installing dependencies via apt..."

# QEMU
if ! command -v qemu-system-x86_64 &> /dev/null; then
    echo "Installing QEMU..."
    apt install -y qemu-system-x86
else
    echo "QEMU (qemu-system-x86_64) already installed."
fi

# OVMF (UEFI firmware for QEMU)
if ! dpkg -s ovmf >/dev/null 2>&1 && ! dpkg -s edk2-ovmf >/dev/null 2>&1; then
    echo "Installing OVMF (UEFI firmware for QEMU)..."
    # Try 'ovmf' first, common on many systems. 'edk2-ovmf' is another name.
    if ! apt install -y ovmf; then 
        echo "'ovmf' package failed or not found, trying 'edk2-ovmf'..."
        apt install -y edk2-ovmf
    fi
else
    echo "OVMF package (ovmf or edk2-ovmf) already installed."
fi

# x86_64 cross-compiler
if ! command -v x86_64-elf-gcc &> /dev/null; then
    echo "Installing x86_64-elf-gcc cross-compiler..."
    apt install -y gcc-x86-64-elf binutils-x86-64-elf
else
    echo "x86_64-elf-gcc already installed."
fi

echo ""
echo "System dependencies installed."
echo "--------------------------------------------------------------------------------"
echo "NEXT STEPS:"
echo "1. Ensure OVMF files are accessible:"
echo "   This script attempted to install OVMF. The 'make run' command expects"
echo "   'OVMF_CODE.fd' and 'OVMF_VARS.fd' to be in the 'qemu-testing/' directory"
echo "   at the root of your project. 'make run' will no longer try to copy them itself."
echo "   Common locations for these files after installation are '/usr/share/OVMF/' or"
echo "   '/usr/share/edk2-ovmf/x64/'. If 'make run' fails to find them, please copy"
echo "   them manually: cp /usr/share/OVMF/*.fd qemu-testing/ (adjust path if needed)."
echo ""
echo "2. Install Rust and components (as your regular user, not root):"
echo "   Install Rustup (if you don't have it):"
echo "     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"
echo "     Follow instructions to add .cargo/bin to your PATH. You may need to restart your terminal."
echo "   Install Nightly toolchain and rust-src component:"
echo "     rustup install nightly"
echo "     rustup component add rust-src"
echo "--------------------------------------------------------------------------------"
echo "Linux setup script complete!" 