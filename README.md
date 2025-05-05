# OptiOS

A performance optimised and secure OS by default that can scale from small IOT devices to home PCs.

OptiOS is an event-driven operating system built around the principle of minimal persistent computation, fast startup, and deterministic state handling. It rethinks background tasks and system processes, while retaining traditional semantics for interactive foreground execution.

## Core Concepts

**Foreground Execution:**
Foreground user processes run continuously like in traditional OSes. By default, a single foreground process is allowed, but the kernel may be configured (e.g. via a filesystem config) to support multiple concurrent foreground processes depending on the platform’s capabilities.

**Event-Driven Background Handlers:**
Background work is handled by handlers, which are small programs compiled from user code and triggered by explicit events (like timers, network packets, or app-defined triggers). These handlers run in isolated memory snapshots.

**Snapshot-Based State:**
Each handler is launched into a memory snapshot that preserves its own global state. Handlers define a version string; any change to this version causes the snapshot to reset. Global variables defined at the top level of the handler file persist between runs, enabling stateful behavior without external storage.

**Run-to-Completion Semantics:**
Handlers do not block, pause, or yield. They run to completion and terminate. Any required state must be written into snapshot-persistent memory or external files.

**No Background Daemons:**
There are no idle polling services, message queues, or background daemons. All background computation is explicit, event-bound, and time-scoped.

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
