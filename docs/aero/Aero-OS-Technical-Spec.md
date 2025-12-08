# Aero Operating System - Technical Specification

**Last Updated:** December 7, 2025

**Status:** Design Phase - No Implementation Yet

**Reality Check:** This document describes the design for Aero OS. Currently, only minimal kernel scaffolding exists. Actual OS development begins after Fruti compiler Phase 3.

---

## Implementation Roadmap

### Current Phase: Design Complete - READY

All specifications finalized, ready for implementation when compiler matures.

### Phase 1: Kernel Foundation (Years 4-5)

**After Fruti Compiler Phase 3 completion**

- Bootloader (UEFI support)
- Kernel initialization
- Memory management (paging, heap)
- Interrupt handling
- Basic process management

**Deliverable:** Bootable kernel with core services

### Phase 2: Core Services (Mid Phase)

- Virtual file system
- Device driver framework
- System call interface
- Inter-process communication
- Scheduler improvements

**Deliverable:** Functional microkernel

### Phase 3: User Space (Later Phase)

- User space runtime
- Standard library integration
- Shell and basic utilities
- Package manager
- Development tools

**Deliverable:** Usable developer environment

### Phase 4: Production Ready (Year 7+)

- Hardware support expansion
- Performance optimization
- Security hardening
- Documentation and tooling

**Deliverable:** Production-ready OS

---

## Table of Contents

1. [Vision and Goals](#vision-and-goals)
2. [Architecture Overview](#architecture-overview)
3. [Kernel Design](#kernel-design)
4. [Memory Management](#memory-management)
5. [Process Management](#process-management)
6. [File System](#file-system)
7. [Device Drivers](#device-drivers)
8. [Networking](#networking)
9. [Security](#security)
10. [Boot Process](#boot-process)

---

## Vision and Goals

### Mission Statement

Aero OS aims to be a modern, secure, and developer-friendly operating system written in Fruti, demonstrating that systems programming can be both safe and ergonomic.

### Core Principles

1. **Security by Default**
   - Memory safety through Fruti's ownership system
   - Capability-based security model
   - Minimal trusted computing base
   - Defense in depth

2. **Performance**
   - Zero-cost abstractions
   - Efficient system calls
   - Optimized memory management
   - Low-latency scheduling

3. **Developer Experience**
   - Clear and consistent APIs
   - Comprehensive documentation
   - Excellent error messages
   - Modern tooling

4. **Compatibility**
   - POSIX-like interface where practical
   - Support for modern hardware
   - Standard protocols (TCP/IP, etc.)
   - Easy porting of applications

### Non-Goals

- **Not** attempting to replace Linux/Windows for general use
- **Not** focusing on embedded systems (initially)
- **Not** prioritizing backward compatibility with legacy systems
- **Not** trying to support every hardware configuration

---

## Architecture Overview

### Microkernel Design

```
┌─────────────────────────────────────────────┐
│           User Space Applications           │
├─────────────────────────────────────────────┤
│         System Services (User Mode)         │
│  ┌──────────┐ ┌──────────┐ ┌─────────────┐│
│  │    VFS   │ │ Net Stack│ │   Drivers   ││
│  └──────────┘ └──────────┘ └─────────────┘│
├─────────────────────────────────────────────┤
│         Microkernel (Kernel Mode)           │
│  ┌──────────┐ ┌──────────┐ ┌─────────────┐│
│  │  Memory  │ │   IPC    │ │  Scheduler  ││
│  └──────────┘ └──────────┘ └─────────────┘│
├─────────────────────────────────────────────┤
│              Hardware Abstraction           │
└─────────────────────────────────────────────┘
```

### Design Rationale

**Microkernel Benefits:**
- Smaller trusted computing base
- Better fault isolation
- Easier to debug and maintain
- Flexible architecture

**Tradeoffs:**
- More context switches (mitigated with optimizations)
- More complex IPC (simplified with zero-copy where possible)

### System Components

**Kernel Mode (Minimal):**
- Memory management
- Process scheduling
- IPC primitives
- Interrupt handling
- Low-level hardware access

**User Mode (Most Functionality):**
- File systems
- Device drivers (most)
- Network stack
- System services

---

## Kernel Design

### Kernel Structure

```
aero-kernel/
├── src/
│   ├── boot/           # Boot process
│   ├── mm/             # Memory management
│   ├── sched/          # Scheduler
│   ├── ipc/            # Inter-process communication
│   ├── sync/           # Synchronization primitives
│   ├── arch/           # Architecture-specific code
│   │   ├── x86_64/
│   │   └── aarch64/
│   └── drivers/        # Kernel-mode drivers only
```

### Core Kernel Services

**1. Memory Management**
- Physical memory allocation
- Virtual memory (paging)
- Kernel heap
- Page fault handling

**2. Scheduler**
- Preemptive multitasking
- Priority-based scheduling
- CPU affinity
- Load balancing

**3. IPC**
- Message passing
- Shared memory
- Synchronization objects

**4. System Calls**
- Minimal set of kernel syscalls
- Fast path for common operations
- Security validation

### Kernel API

```fruti
// Core kernel types
struct Process { ... }
struct Thread { ... }
struct Memory { ... }

// Syscall interface (minimal)
fn sys_send(target: ProcessId, msg: &Message) -> Result<(), Error>
fn sys_recv(source: ProcessId, msg: &mut Message) -> Result<(), Error>
fn sys_map(addr: VirtAddr, size: usize, flags: MapFlags) -> Result<(), Error>
fn sys_unmap(addr: VirtAddr, size: usize) -> Result<(), Error>
fn sys_spawn(path: &Path, args: &[str]) -> Result<ProcessId, Error>
```

---

## Memory Management

### Address Space Layout

```
64-bit Address Space (x86_64)

0x0000_0000_0000_0000 - 0x0000_7FFF_FFFF_FFFF  User space (128 TB)
├── 0x0000_0000_0040_0000                      Program code
├── 0x0000_0000_0080_0000                      Heap
├── 0x0000_7FFF_FF00_0000                      Stack (grows down)
└── 0x0000_7FFF_FFFF_FFFF

0xFFFF_8000_0000_0000 - 0xFFFF_FFFF_FFFF_FFFF  Kernel space (128 TB)
├── 0xFFFF_8000_0000_0000                      Physical memory map
├── 0xFFFF_A000_0000_0000                      Kernel heap
├── 0xFFFF_C000_0000_0000                      Kernel modules
└── 0xFFFF_FFFF_FFFF_F000                      Kernel code
```

### Page Management

**Page Size:** 4 KB standard, support for 2 MB and 1 GB huge pages

**Page Allocator:**
- Buddy allocator for physical pages
- Slab allocator for small objects
- Zero-page optimization
- Copy-on-write support

**Virtual Memory:**
- 4-level page tables (x86_64)
- Lazy allocation
- Demand paging
- Memory-mapped files

### Heap Allocator

```fruti
// Kernel heap allocator
fn kmalloc(size: usize) -> *mut u8
fn kfree(ptr: *mut u8)
fn krealloc(ptr: *mut u8, new_size: usize) -> *mut u8

// User space allocator (via syscalls)
fn mmap(addr: Option<VirtAddr>, len: usize, prot: Protection, 
        flags: MapFlags) -> Result<VirtAddr, Error>
fn munmap(addr: VirtAddr, len: usize) -> Result<(), Error>
```

---

## Process Management

### Process Model

**Process:**
- Independent address space
- One or more threads
- Resource ownership
- Security context (capabilities)

**Thread:**
- Execution context
- Shares address space with sibling threads
- Independent stack
- Schedulable entity

### Process Structure

```fruti
struct Process {
    pid: ProcessId,
    parent: Option<ProcessId>,
    children: Vec<ProcessId>,
    
    address_space: AddressSpace,
    threads: Vec<Thread>,
    
    capabilities: CapabilitySet,
    open_files: FileTable,
    
    state: ProcessState,
}

enum ProcessState {
    Running,
    Ready,
    Blocked(WaitReason),
    Zombie,
}
```

### Scheduler

**Algorithm:** CFS-inspired (Completely Fair Scheduler)

**Features:**
- O(log n) complexity
- Fair CPU time distribution
- Priority support
- Real-time task support
- CPU affinity

**Time Slices:**
- Dynamic based on load
- Minimum: 1 ms
- Maximum: 100 ms
- Higher priority = more time

### Context Switching

```fruti
// Fast context switch
fn switch_to(from: &mut Thread, to: &Thread) {
    // Save current state
    save_registers(&from.context);
    
    // Switch page tables
    switch_page_table(&to.process.address_space);
    
    // Load new state
    restore_registers(&to.context);
}
```

---

## File System

### Virtual File System (VFS)

**Design:** Everything is a file (Unix philosophy)

**VFS Interface:**
```fruti
trait FileSystem {
    fn open(&self, path: &Path, flags: OpenFlags) -> Result<FileHandle, Error>;
    fn read(&self, handle: FileHandle, buf: &mut [u8]) -> Result<usize, Error>;
    fn write(&self, handle: FileHandle, buf: &[u8]) -> Result<usize, Error>;
    fn close(&self, handle: FileHandle) -> Result<(), Error>;
    fn stat(&self, path: &Path) -> Result<Metadata, Error>;
}
```

### Native File System: AeroFS

**Features:**
- Journaling for crash recovery
- Copy-on-write for snapshots
- Built-in compression
- Encryption support
- Extended attributes

**On-Disk Structure:**
```
Superblock (4 KB)
├── Magic number
├── Version
├── Block size
├── Total blocks
└── Root inode

Inode Table
├── Inode 0 (reserved)
├── Inode 1 (root directory)
└── ...

Data Blocks
└── File data, directory entries
```

### File Operations

```fruti
// User space API
fn open(path: &Path, flags: OpenFlags) -> Result<File, Error>
fn read(file: &File, buf: &mut [u8]) -> Result<usize, Error>
fn write(file: &File, buf: &[u8]) -> Result<usize, Error>
fn close(file: File) -> Result<(), Error>

// Directory operations
fn create_dir(path: &Path) -> Result<(), Error>
fn read_dir(path: &Path) -> Result<Vec<DirEntry>, Error>
fn remove_file(path: &Path) -> Result<(), Error>
fn rename(from: &Path, to: &Path) -> Result<(), Error>
```

---

## Device Drivers

### Driver Model

**User Space Drivers (Most):**
- Storage drivers (NVMe, SATA)
- Network drivers (Ethernet, WiFi)
- USB drivers
- Graphics drivers

**Kernel Mode Drivers (Essential Only):**
- Interrupt controller
- Timer
- Basic console
- Memory-mapped I/O

### Driver Interface

```fruti
trait Driver {
    fn probe(&self, device: &Device) -> Result<(), Error>;
    fn init(&mut self) -> Result<(), Error>;
    fn shutdown(&mut self) -> Result<(), Error>;
}

trait BlockDevice {
    fn read_block(&self, block: u64, buf: &mut [u8]) -> Result<(), Error>;
    fn write_block(&self, block: u64, buf: &[u8]) -> Result<(), Error>;
    fn block_size(&self) -> usize;
}

trait NetworkDevice {
    fn send_packet(&self, packet: &[u8]) -> Result<(), Error>;
    fn recv_packet(&self, buf: &mut [u8]) -> Result<usize, Error>;
    fn mac_address(&self) -> MacAddress;
}
```

### Hardware Abstraction

```fruti
// Architecture-specific code hidden behind traits
trait ArchSpecific {
    fn enable_interrupts();
    fn disable_interrupts();
    fn read_port(port: u16) -> u8;
    fn write_port(port: u16, value: u8);
}
```

---

## Networking

### Network Stack (User Space)

**TCP/IP Stack:**
- IPv4 and IPv6 support
- TCP, UDP, ICMP
- Raw sockets
- Packet filtering

**Socket API:**
```fruti
struct Socket { ... }

impl Socket {
    fn new(domain: Domain, type: SocketType) -> Result<Socket, Error>
    fn bind(&self, addr: SocketAddr) -> Result<(), Error>
    fn connect(&self, addr: SocketAddr) -> Result<(), Error>
    fn listen(&self, backlog: i32) -> Result<(), Error>
    fn accept(&self) -> Result<(Socket, SocketAddr), Error>
    fn send(&self, buf: &[u8]) -> Result<usize, Error>
    fn recv(&self, buf: &mut [u8]) -> Result<usize, Error>
}
```

---

## Security

### Capability-Based Security

**Capabilities:**
- Fine-grained permissions
- Non-transferable by default
- Revocable
- Hierarchical

**Example Capabilities:**
```fruti
enum Capability {
    FileRead(Path),
    FileWrite(Path),
    NetworkListen(Port),
    ProcessSpawn,
    DeviceAccess(DeviceId),
}
```

### Memory Safety

**Enforced by Fruti:**
- No buffer overflows
- No use-after-free
- No data races (in safe code)
- Type safety

### Sandboxing

**Process Isolation:**
- Separate address spaces
- Capability restrictions
- Resource limits
- System call filtering

---

## Boot Process

### Boot Sequence

```
1. UEFI Firmware
   └─> Load bootloader

2. Bootloader
   ├─> Initialize graphics
   ├─> Load kernel
   ├─> Set up page tables
   └─> Jump to kernel

3. Kernel Initialization
   ├─> Initialize CPU
   ├─> Set up GDT, IDT
   ├─> Initialize memory management
   ├─> Initialize interrupts
   ├─> Start scheduler
   └─> Launch init process

4. Init Process
   ├─> Mount root filesystem
   ├─> Start system services
   └─> Launch user shell
```

### Bootloader

**AeroBoot:**
- UEFI-based
- Multiboot2 compliant
- Graphics mode setup
- Kernel loading and relocation

---

## Development Roadmap

### Timeline Summary

| Phase | Focus | Status |
|-------|-------|--------|
| Phase 0 | Design | Complete (2025) |
| Phase 1 | Kernel Foundation | Not Started |
| Phase 2 | Core Services | Planned |
| Phase 3 | User Space | Planned |
| Phase 4 | Production | Planned |

### Success Criteria

**Phase 1:**
- Boots on real hardware
- Basic memory management works
- Can run simple programs
- No kernel panics under normal load

**Phase 2:**
- File system operational
- Multiple processes can run
- Device driver framework functional
- Network stack basics working

**Phase 3:**
- Developer tools available
- Can compile Fruti programs on Aero
- Shell and utilities functional
- Package manager operational

**Phase 4:**
- Performance competitive with modern OSes
- Stable and secure
- Growing application ecosystem
- Active community

---

## Conclusion

Aero OS represents an ambitious long-term goal to create a modern, safe, and efficient operating system. This specification provides the roadmap for implementation once the Fruti compiler reaches maturity.

**Current Reality:**
- Comprehensive design complete
- Minimal scaffolding exists
- Implementation begins after compiler maturity
- Multi-year development effort

**Key Strengths:**
- Memory-safe by design (Fruti)
- Modern microkernel architecture
- Clear security model
- Developer-friendly

**Challenges:**
- Large scope
- Requires mature compiler
- Hardware support complexity
- Community building

---

**Status:** Design Complete - Implementation Pending
**Last Updated:** December 7, 2025
**Next Milestone:** Begin Phase 1 after Fruti Compiler Phase 3

---

**Frutisoft © 2025 - Fresh code, crisp ideas**
