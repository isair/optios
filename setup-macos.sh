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

# Install QEMU (which includes OVMF files)
echo "Checking for QEMU..."
if ! brew list qemu &>/dev/null; then
    echo "Installing QEMU (this will include OVMF firmware files)..."
    brew install qemu
else
    echo "QEMU already installed."
fi

# Install x86_64 cross-compiler
echo "Checking for x86_64-elf-gcc..."
if ! command -v x86_64-elf-gcc &> /dev/null
then
    echo "x86_64-elf-gcc not found. Installing cross-compiler toolchain..."
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
    echo "Rustup installed. Please ensure \$HOME/.cargo/bin is in your PATH."
    echo "You might need to restart your terminal or run: source \"\$HOME/.cargo/env\""
else
    echo "Rustup already installed."
    rustup update
fi

echo "Installing Rust nightly toolchain and components..."
rustup install nightly
HOST_TRIPLE=$(rustc --version --verbose | grep host | cut -d: -f2 | tr -d '[:space:]')
rustup component add rust-src --toolchain nightly-$HOST_TRIPLE

echo ""
echo "--------------------------------------------------------------------------------"
echo "NEXT STEPS: Manually copy OVMF files for QEMU"
echo "The 'make run' command expects 'OVMF_CODE.fd' and 'OVMF_VARS.fd' to be in the"
echo "'qemu-testing/' directory at the root of your OptiOS project."

QEMU_PREFIX_PATH=$(brew --prefix qemu)
# Common path for OVMF file within Homebrew's QEMU package
OVMF_CODE_FILE_PATH="$QEMU_PREFIX_PATH/share/qemu/edk2-x86_64-code.fd"

echo ""
echo "Homebrew's QEMU package provides the necessary UEFI firmware file. Typically, it is located at:"
echo "  $OVMF_CODE_FILE_PATH"
echo ""
echo "Please copy this file to your 'qemu-testing/' directory, once as 'OVMF_CODE.fd'"
echo "and again as 'OVMF_VARS.fd': "
echo "  mkdir -p qemu-testing"
echo "  cp \"$OVMF_CODE_FILE_PATH\" qemu-testing/OVMF_CODE.fd"
echo "  cp \"$OVMF_CODE_FILE_PATH\" qemu-testing/OVMF_VARS.fd"
echo ""
echo "If the file \"$OVMF_CODE_FILE_PATH\" does not exist, please locate the 'edk2-x86_64-code.fd'"
echo "within your QEMU installation directory (usually under \"$QEMU_PREFIX_PATH/share/qemu/\") and perform the copies manually."
echo "--------------------------------------------------------------------------------"
echo "Setup complete!"
echo "Ensure Rust nightly is active in your project directory (run 'rustup override set nightly' if needed)." 