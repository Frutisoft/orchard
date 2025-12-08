# Aero OS Kernel

**A microkernel operating system design study**

---

## Current Status: Design Phase

**This kernel is not yet implemented.** Aero OS is a comprehensive design specification for a capability-based microkernel operating system. Implementation will begin after the Fruti compiler reaches maturity.

**Current State:**
- [x] Complete architecture design
- [x] 120 system calls specified
- [x] Security model defined (capability-based)
- [x] IPC design documented
- [ ] Implementation: Not started (on hold)
- Timeline: Years 4-6 (after compiler is stable)

**This repository contains:**
- Design documentation
- Architecture specifications
- System call definitions
- Security model descriptions

---

## Overview

Aero is designed as a capability-based microkernel operating system targeting security, performance, and simplicity. The planned kernel is approximately 45,000 lines of code, providing 120 carefully designed system calls.

## Design Goals

- **Microkernel Architecture**: Minimal kernel, services in userspace
- **Capability-Based Security**: Fine-grained access control, no root user
- **High Performance Targets**: < 2s boot time goal, < 500ns IPC latency goal
- **Zero-Copy IPC**: Shared memory for efficient communication
- **Linux Compatibility Goal**: Run Linux binaries (long-term target)
- **Modern Design**: To be written in safe Fruti code (when compiler is ready)

---

## Building (Future Implementation)

**Note:** These instructions describe the intended build process. No buildable code exists yet.

### Prerequisites (When Ready)

- Fruti compiler (see [fruti-compiler](../fruti-compiler/))
- QEMU (for testing)
- Rust nightly (for bootstrapping)
- GNU Make

### Future Build Steps

```bash
cd aero-kernel

# Build kernel (when implemented)
make build

# Create bootable ISO
make iso

# Run in QEMU
make run
```

---

## Design Documentation

For complete design specifications, see:
- [Aero OS Technical Specification](../../docs/aero/Aero-OS-Technical-Spec.md)
- [Architecture Overview](../../docs/aero/os/Architecture/)
- [System Call Reference](../../docs/aero/os/Developer%20Guide/)

---

## Architecture

### Microkernel Design

```
┌─────────────────────────────────────────┐
│         Userspace Applications          │
├─────────────────────────────────────────┤
│     System Services (in userspace)      │
│  - File System Server                   │
│  - Network Stack                        │
│  - Device Drivers                       │
│  - Window Manager                       │
├─────────────────────────────────────────┤
│          Aero Microkernel               │
│  - Process Management                   │
│  - Memory Management                    │
│  - IPC (< 500ns latency)                │
│  - Capability System                    │
│  - Scheduler                            │
└─────────────────────────────────────────┘
         Hardware Abstraction Layer
```

### Key Components

**Kernel Space (45K LOC):**
- Process/thread management
- Virtual memory management
- IPC mechanisms
- System call interface (120 syscalls)
- Capability enforcement
- Hardware abstraction

**Userspace Services:**
- VFS (Virtual File System)
- Network stack (TCP/IP)
- Device drivers
- Window server
- Package manager

---

## System Calls

Aero provides 120 system calls organized into categories:

### Process Management (15 syscalls)
```fruti
syscall::process_create(path: String, args: Vec<String>) -> ProcessId
syscall::process_destroy(pid: ProcessId) -> Result<()>
syscall::process_wait(pid: ProcessId) -> Result<ExitCode>
```

### Memory Management (12 syscalls)
```fruti
syscall::mem_alloc(size: usize, flags: MemFlags) -> Result<*mut u8>
syscall::mem_free(ptr: *mut u8, size: usize) -> Result<()>
syscall::mem_map(addr: *mut u8, size: usize, prot: Protection) -> Result<()>
```

### IPC (18 syscalls)
```fruti
syscall::ipc_send(target: ProcessId, msg: &Message) -> Result<()>
syscall::ipc_recv(timeout: Duration) -> Result<Message>
syscall::ipc_call(target: ProcessId, msg: &Message) -> Result<Message>
```

See [System Call Reference](../../docs/aero/os/Developer%20Guide/System-Calls.md) for complete list.

---

## Security Model

### Capabilities

Every process has a set of capabilities defining what it can access:

```fruti
enum Capability {
    FileRead(path: String),
    FileWrite(path: String),
    NetworkBind(port: u16),
    ProcessSpawn,
    DeviceAccess(device: String),
    // ... 120 total capabilities
}
```

### No Root User

- No superuser with unrestricted access
- All access is capability-based
- Principle of least privilege enforced

### Sandboxing

- Processes sandboxed by default
- Explicit capability grants required
- Capability delegation controlled

---

## Performance

**Boot Time:**
- Target: Cold boot < 2 seconds
- Target: Warm boot < 1 second

**IPC Performance:**
- Target: Fast message passing (sub-microsecond)
- Shared memory: Zero-copy design
- Target: Low synchronous call overhead

**System Call Overhead:**
- Target: Minimal overhead (comparable to Linux)

**Compatibility:**
- Linux binaries: Target near-native performance
- Windows (via Wine): Target good compatibility

---

## Directory Structure

```
aero-kernel/
├── src/
│   ├── arch/           # Architecture-specific code
│   ├── memory/         # Memory management
│   ├── process/        # Process/thread management
│   ├── ipc/            # Inter-process communication
│   ├── syscalls/       # System call handlers
│   ├── capability/     # Capability system
│   ├── scheduler/      # Process scheduler
│   └── boot/           # Boot code
├── drivers/            # Device drivers
│   ├── block/
│   ├── network/
│   ├── input/
│   └── graphics/
├── userspace/          # Userspace services
│   ├── init/           # Init system
│   ├── vfs/            # Virtual file system
│   └── netstack/       # Network stack
└── tests/              # Kernel tests
```

---

## Testing

```bash
# Run all kernel tests
make test

# Run specific test suites
make test-memory
make test-ipc
make test-scheduler
make test-syscalls

# Run in QEMU with test kernel
make test-qemu

# Stress tests
make stress-test
```

---

## Contributing

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for guidelines.

### Development Workflow

```bash
# Create feature branch
git checkout -b feature/new-syscall

# Make changes
vim src/syscalls/mod.rs

# Build and test
make build
make test

# Submit PR
```

---

## Linux Compatibility

Aero provides Linux binary compatibility through:

1. **Linux syscall translation layer**
2. **Compatible ELF loader**
3. **POSIX API implementation**
4. **Emulated /proc and /sys filesystems**

Performance: 95-100% of native Linux.

---

## Documentation

- [Kernel Architecture](../../docs/aero/os/Architecture/)
- [System Call Reference](../../docs/aero/os/Developer%20Guide/)
- [Driver Development](../../docs/aero/os/Developer%20Guide/Driver-Development.md) (coming soon)
- [Security Model](../../docs/aero/Aero-OS-Technical-Spec.md)

---

## Roadmap

**Near Term:**
- Compiler Phase 2 completion
- Fruti self-hosting experiments

**Mid Term:**
- Begin Aero kernel design refinement
- Prototype basic kernel structures
- Memory management proof-of-concept

**Later:**
- Kernel implementation in Fruti
- Core system calls
- Basic IPC mechanisms

---

## License

MIT License - see [LICENSE](../../LICENSE)

---

**Frutisoft © 2025 - Fresh code, crisp ideas**
