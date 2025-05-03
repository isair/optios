#!/bin/bash

echo "Setting up OptiOS development environment for macOS..."

# Check for Homebrew
if ! command -v brew &> /dev/null
then
    echo "Homebrew not found. Please install it first: https://brew.sh/"
    exit 1
fi

# Update Homebrew
echo "Updating Homebrew..."
brew update

# Install necessary packages
echo "Installing dependencies via Homebrew..."
brew install qemu           # Emulator
# brew install xorriso        # ISO creation tool - REMOVED
# brew install i686-elf-grub  # GRUB for BIOS boot - REMOVED

# Install x86_64 cross-compiler (if not already installed)
# Using a tap for a known working toolchain
echo "Checking for x86_64-elf-gcc..."
if ! command -v x86_64-elf-gcc &> /dev/null
then
    echo "x86_64-elf-gcc not found. Installing cross-compiler toolchain..."
    # Check if tap exists, add if not
    if ! brew tap | grep -q messense/homebrew-macos-cross-toolchains; then
      brew tap messense/homebrew-macos-cross-toolchains
    fi
    brew install x86_64-elf-gcc
else
    echo "x86_64-elf-gcc already installed."
fi

# Check/Install Rustup and Nightly toolchain
echo "Checking for Rustup..."
if ! command -v rustup &> /dev/null
then
    echo "Rustup not found. Installing Rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path
    # Add rustup to the current shell path - This won't affect the parent shell.
    # source "$HOME/.cargo/env"
    echo "Rustup installed. Please ensure \$HOME/.cargo/bin is in your PATH."
    echo "You might need to restart your terminal or run: source \"\$HOME/.cargo/env\""
else
    echo "Rustup already installed."
    rustup update
fi

echo "Installing Rust nightly toolchain and components..."
rustup install nightly
# Determine host triple for rust-src component
HOST_TRIPLE=$(rustc --version --verbose | grep host | cut -d: -f2 | tr -d '[:space:]')
rustup component add rust-src --toolchain nightly-$HOST_TRIPLE

echo "Setup complete!"
echo "Ensure Rust nightly is active in your project directory (run 'rustup override set nightly' if needed)." 