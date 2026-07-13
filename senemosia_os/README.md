# Senemosìa Punto Zero - Quasicrystalline Operating System

A bare-metal operating system kernel built in Rust featuring **6D→3D geometric projection**, **φ-based memory allocation**, **Laplacian spectral process scheduling**, and **wave-based IPC**.

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    LAYER 4: NETWORK                         │
│           Wave-based IPC + Ethernet Layer 2                   │
├─────────────────────────────────────────────────────────────┤
│                    LAYER 3: FILE SYSTEM                     │
│         Spatial Node File System (SNFS)                      │
├─────────────────────────────────────────────────────────────┤
│                    LAYER 2: SCHEDULER                       │
│          Spectral Scheduling (Laplacian Matrix)              │
├─────────────────────────────────────────────────────────────┤
│                    LAYER 1: MEMORY                          │
│         φ-Allocator + Geometric Addressing                  │
├─────────────────────────────────────────────────────────────┤
│                    LAYER 0: GEOMETRY                        │
│          6D→3D Projection + Conflict Resolution             │
└─────────────────────────────────────────────────────────────┘
```

## 📁 Project Structure

```
senemosia_os/
├── Cargo.toml              # Project configuration
├── linker.ld              # Custom linker script
├── limine.cfg             # Bootloader configuration
├── build.rs               # Build script
├── src/
│   ├── main.rs            # Kernel entry point (_start)
│   ├── geometry/
│   │   ├── mod.rs
│   │   ├── projection.rs  # 6D→3D projection matrix
│   │   └── conflict.rs    # 6D Givens rotation
│   ├── memory/
│   │   ├── mod.rs
│   │   └── allocator.rs   # φ-based memory allocator
│   ├── sched/
│   │   ├── mod.rs
│   │   └── spectral.rs    # Laplacian spectral scheduler
│   ├── fs/
│   │   ├── mod.rs
│   │   └── snfs.rs        # Spatial Node File System
│   ├── drivers/
│   │   ├── mod.rs
│   │   └── video.rs       # Framebuffer driver
│   ├── interrupts/
│   │   ├── mod.rs
│   │   ├── apic.rs        # APIC timer driver
│   │   └── handlers.rs    # Interrupt handlers
│   ├── syscall/
│   │   └── mod.rs         # System call interface
│   ├── ipc/
│   │   ├── mod.rs
│   │   └── wave_channel.rs # Wave-based IPC
│   └── net/
│       ├── mod.rs
│       └── wave_nic.rs    # Network driver
└── dev/
    ├── build.sh           # Build script
    └── make_iso.sh        # ISO generation
```

## 🔢 Key Concepts

### Golden Ratio (φ)
- **φ** = 1.618033988749895...
- **1/φ** = 0.618033988749895...
- Used for memory partitioning, scheduling weights, and wave frequencies

### 6D→3D Projection
Coordinates in 6D hyperdimensional space are projected to 3D physical space using an icosahedral projection matrix based on φ.

### Spectral Scheduling
Process execution order is determined by the **Fiedler vector** (second smallest eigenvalue of the Laplacian matrix), maximizing system resonance.

### Wave-Based IPC
Processes communicate through resonant wave channels where data is encoded as wave phase modulation, allowing constructive/destructive interference.

## 🚀 Building

```bash
# Install Rust target
rustup target add x86_64-unknown-none

# Build
cargo build --release

# Or use the build script
./dev/build.sh
```

Output: `target/x86_64-unknown-none/release/senemosia_kernel`

## 📦 Dependencies

- **libm** (0.2) - Mathematical functions for no_std environment
- **Rust 2021 Edition** with x86_64-unknown-none target

## 🔧 Features

| Module | Feature | Status |
|--------|---------|--------|
| Geometry | 6D→3D Icosahedral Projection | ✅ |
| Geometry | 6D Givens Rotation | ✅ |
| Memory | φ-based Allocator | ✅ |
| Memory | Spectral Conflict Resolution | ✅ |
| Scheduler | Laplacian Spectrum Computation | ✅ |
| Scheduler | Fiedler Vector Scheduling | ✅ |
| File System | Spatial Node File System (SNFS) | ✅ |
| Drivers | Framebuffer Video | ✅ |
| Drivers | APIC Timer | ✅ |
| IPC | Wave Channel | ✅ |
| Network | Wave NIC Driver | ✅ |

## 📜 License

MIT License

## 👥 Authors

Senemosìa Cooperative

---

**Status**: In Development  
**Version**: 0.1.0
