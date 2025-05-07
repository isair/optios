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
echo "NEXT STEPS: Install Rust and components (as your regular user, not root):"
echo "1. Install Rustup (if you don't have it):"
echo "   curl --proto \'=https\' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"
echo "   Follow instructions to add .cargo/bin to your PATH (e.g., source \$HOME/.cargo/env). You may need to restart your terminal."
echo "2. Install Nightly toolchain and rust-src component:"
echo "   rustup install nightly"
echo "   rustup component add rust-src"
echo "--------------------------------------------------------------------------------"
echo "Linux setup script complete!" 