# OptiOS

A performance optimised and secure OS by default that can scale from small IOT devices to home PCs.

## Design

OptiOS is an event-driven operating system designed for determinism, performance, and transparency. It replaces traditional daemons and idle background computation with scoped, permissioned, snapshot-based tasks triggered by explicit events.

### Core Concepts

#### Foreground Execution

Foreground user programs behave like traditional processes, with full access to user input, rendering, and direct interaction. While the system defaults to running a single foreground process, this limit can be adjusted by the kernel (e.g., via a configuration file in the filesystem) to support multiple concurrent foreground applications, depending on platform capabilities.

#### Event-Driven Background Tasks

All background computation is initiated by events—timestamped system or application-level triggers (e.g. timers, file updates, sensor input, or network activity). These events launch **handlers**, which are compiled programs registered by the user.

Handlers:
- Run to completion (no yielding or suspension)
- Execute inside isolated memory snapshots
- Can define global variables scoped to that snapshot, enabling state persistence across runs
- Are versioned; changes to the handler's code or declared version reset its snapshot

There are no daemons, no polling loops, and no ambient computation. Everything runs because an event happened.

#### Memory Snapshots

Handlers are launched into fast-start memory snapshots that persist global state between runs. Snapshots are specific to a given handler version and program. This enables rapid, stateful computation without persistent processes or manual state rehydration.

#### Permissions & Sandboxing

OptiOS enforces strict sandboxing at the handler level. Each program declares:
- **Allowed Events**: What events it's permitted to register or respond to
- **Output Capabilities**: What kinds of effects it may produce (file writes, messages, etc.)
- **Execution Contexts**: Whether it can run in the foreground, background, or both

Handlers cannot subscribe to or observe events they have not been granted access to. This makes the permission model *event-centric* and declarative: the only way a program can do something is if it is explicitly allowed to do it *when* a known event occurs.

This system enables fine-grained control, secure defaults, and static reasoning about system behavior.

### Benefits

- **Security by Design**  
  No ambient computation. All effects and triggers are explicitly permissioned, sandboxed, and logged.

- **Determinism & Debuggability**  
  Snapshot-based memory, event timestamps, and versioned handlers make system behavior transparent, reproducible, and easy to test.

- **Performance**  
  Fast warm starts through snapshot loading; no need to reinitialize global state or reparse config on every trigger.

- **Portability**  
  Kernel policies (like allowed number of foreground processes) are configurable, enabling the same system to scale across embedded devices, desktops, and servers.

- **Developer Clarity**  
  Run-to-completion handlers with isolated memory mean no race conditions, no shared state bugs, and no surprises.

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
