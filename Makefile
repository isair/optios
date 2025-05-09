# Use cargo for build operations
CARGO = cargo

# Default target: build the UEFI application
all: build

build:
	@echo "Building UEFI application using Cargo..."
	$(CARGO) build

# Run the UEFI application in QEMU
run: build
	@echo "Preparing to run QEMU..."
	@mkdir -p qemu-testing/esp/efi/boot
	@if [ ! -f qemu-testing/OVMF_CODE.fd ] || [ ! -f qemu-testing/OVMF_VARS.fd ]; then \
	    echo "Error: OVMF_CODE.fd or OVMF_VARS.fd not found in ./qemu-testing/"; \
	    echo "Please ensure you have run the setup script for your OS (e.g., ./setup-macos.sh)"; \
	    echo "and followed the instructions to copy the OVMF files into ./qemu-testing/"; \
	    exit 1; \
	fi
	@ln -sf ../../../target/x86_64-unknown-uefi/debug/optios.efi qemu-testing/esp/efi/boot/bootx64.efi
	@echo "Starting QEMU... (Log output will appear here)"
	qemu-system-x86_64 \
	    -drive if=pflash,format=raw,readonly=on,file=qemu-testing/OVMF_CODE.fd \
	    -drive if=pflash,format=raw,file=qemu-testing/OVMF_VARS.fd \
	    -drive format=raw,file=fat:rw:qemu-testing/esp \
	    -net none \
	    -serial stdio

# Clean up build files and QEMU specific files
clean:
	$(CARGO) clean
	@rm -f qemu-testing/esp/efi/boot/bootx64.efi
	# Note: OVMF files in qemu-testing/ are not removed by clean, as they are manually placed.
	@echo "Cleaned build artifacts and QEMU symlink."

# Phony targets
.PHONY: all build run clean 