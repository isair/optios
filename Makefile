# Use cargo build and run
CARGO = cargo

# Default target: build the kernel
all: build

build:
	@echo "Building kernel using Cargo..."
	$(CARGO) build

# Run the OS in QEMU using cargo run (requires .cargo/config.toml runner)
run:
	@echo "Running QEMU via cargo run..."
	$(CARGO) run

# Clean up build files
clean:
	$(CARGO) clean
	@echo "Cleaned build artifacts."

# Phony targets
.PHONY: all build run clean 