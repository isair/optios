# OptiOS

A performance optimised and secure OS by default that can scale from small IOT devices to home PCs.

## Core Philosophy

-   **No Background Processes:** Traditional background tasks are disallowed. Programs execute, complete their task, and terminate.
-   **Restricted Compilation for Handlers:** Programs can compile other, more restricted *handler programs*.
-   **Event-Driven Execution:** The OS executes compiled handler programs based on specific high-level event triggers (e.g., `background-task-tick`, `notification-action`, `screen-frame-capture`, etc.). The OS maintains logs of every event handled and the corresponding handler's output.
-   **Foreground Focus (User Processes):** The system prioritizes the currently active, user-facing task above all else, minimizing context switching and resource contention for *user processes*.
-   **User Process Sandboxing:** User processes operate within a sandboxed environment, restricted to their own designated folder within the filesystem. Access outside this folder requires explicit kernel grants.
-   **Kernel-Managed Permissions:** All system resource access (filesystem, network, hardware, etc.) is governed by a strict, granular permission system managed by the kernel, primarily based on granting handlers the capability to respond to specific events.

This design aims to eliminate overhead associated with traditional multitasking and background processing, dedicating system resources to the task at hand while maintaining security and control through the kernel.

## Setup & Building

### Prerequisites

Building and running OptiOS requires the following:

1.  **Rust Nightly Toolchain:** Install via [rustup](https://rustup.rs/). Run:
    ```bash
    rustup install nightly
    rustup override set nightly
    # Add the rust-src component needed for libcore on custom targets
    rustup component add rust-src --toolchain nightly-x86_64-unknown-none 
    ```
2.  **x86_64 Cross-Compilation Tools:**
    *   A cross-compiler (e.g., `x86_64-elf-gcc`, `x86_64-elf-as`, `x86_64-elf-ld`).
    *   GRUB utilities for the target (e.g., `i686-elf-grub`).
    *   `xorriso` (for creating the ISO).
    *   See `setup-macos.sh` or `setup-windows.ps1` (TODO) for specific installation commands.
3.  **QEMU:** For running the OS (`qemu-system-x86_64`).

### Setup Scripts

Convenience scripts are provided for setting up dependencies:

*   **macOS:** `./setup-macos.sh` (Installs via Homebrew)
*   **Windows:** (TODO: Create `setup-windows.ps1`)
*   **Linux (Debian/Ubuntu):** (TODO: Create `setup-linux.sh`)

### Building

Once prerequisites are installed, build the kernel and ISO image:

```bash
make all
```

### Running

Run the built OS in QEMU:

```bash
make run
``` 