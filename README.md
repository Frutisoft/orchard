# Frutisoft: Fruti Language & Aero OS

**A learning project exploring modern programming language and OS design**

---

## Vision

Frutisoft is a solo student project exploring the design and implementation of:

- **Fruti** - A general-purpose programming language that learns from the pain points of existing languages to create a simple, comprehensive, and developer-friendly alternative

- **Aero OS** - A capability-based microkernel OS design study

**Current Status:** Early development / Design phase

**Developer:** Solo student learning compiler and OS development

**Timeline:** Multi-year learning and development project

---

## Monorepo Structure

```
frutisoft/                    # Root monorepo
├── packages/                 # Main projects
│   ├── fruti-compiler/       # Fruti compiler (Rust bootstrap)
│   │   ├── src/              # Compiler source code
│   │   │   ├── lexer.rs      # Tokenization (553 lines)
│   │   │   ├── parser.rs     # AST generation (1,096 lines)
│   │   │   ├── semantic.rs   # Type checking (653 lines)
│   │   │   └── codegen.rs    # LLVM IR generation (308 lines)
│   │   ├── tests/            # Compiler tests
│   │   └── Cargo.toml        # Package manifest
│   │
│   └── aero-kernel/          # Aero OS kernel
│       ├── src/              # Kernel source code
│       ├── drivers/          # Device drivers (planned)
│       └── Cargo.toml        # Package manifest
│
├── docs/                     # Complete documentation
│   ├── fruti/                # Fruti language docs
│   │   ├── language/         # Guides and reference
│   │   ├── examples/         # Code examples
│   │   └── Language-Design-Decisions.md
│   ├── aero/                 # Aero OS docs
│   │   ├── os/               # Architecture and guides
│   │   └── Aero-OS-Technical-Spec.md
│   ├── project/              # Project management
│   ├── resources/            # Production readiness guides
│   ├── assets/               # Logos, images, media
│   └── archive/              # Historical documents
│
├── examples/                 # Working code samples
│   ├── hello-world.fruti     # Basic Fruti program
│   ├── basic-features.fruti  # Language features demo
│   └── README.md
│
├── website/                  # GitHub Pages site
│   ├── index.html            # Main homepage
│   └── README.md
│
├── tools/                    # Build and utility scripts
│   ├── build.ps1             # Build script
│   ├── test.ps1              # Test runner
│   └── README.md
│
├── .github/                  # CI/CD workflows
├── Cargo.toml                # Workspace manifest
├── CONTRIBUTING.md           # Contribution guidelines
├── LICENSE                   # MIT License
└── README.md                 # This file
```

---

## Getting Started

### Quick Start

**Clone the monorepo:**
```bash
git clone https://github.com/Frutisoft/frutisoft.git
cd frutisoft
```

**Build the Fruti compiler:**
```bash
# Build entire workspace
cargo build --workspace

# Or build just the compiler
cargo build -p fruti-compiler

# Or use the build script
.\tools\build.ps1 -Target fruti
```

**Try the compiler:**
```bash
cd packages/fruti-compiler

# Check an example
cargo run -- check ../../examples/hello-world.fruti

# Build to LLVM IR
cargo run -- build ../../examples/hello-world.fruti -o hello

# View verbose output
$env:FRUTI_VERBOSE="1"
cargo run -- check ../../examples/basic-features.fruti
```

### For Interested Observers

**Explore the Design:**
1. Read the [Language Design Decisions](./docs/fruti/Language-Design-Decisions.md)
2. Explore [Code Examples](./examples/)
3. Review the [OS Architecture Spec](./docs/aero/Aero-OS-Technical-Spec.md)
4. Check [Phase 1 MVP Status](./docs/project/management/PHASE-1-MVP-COMPLETE.md)

**Current State:**
- [x] Phase 1 MVP Complete (Fruti Compiler)
- [x] Lexer (~553 lines, 7 tests passing)
- [x] Parser (~1,096 lines, recursive descent)
- [x] Semantic Analysis (~653 lines, type checking)
- [x] Code Generation (~308 lines, LLVM IR)
- [ ] Aero OS (design phase - scaffolding only)
- [x] Website (GitHub Pages ready)

### For Developers (Future)

**This project is not yet ready for external contributors.** Once the core compiler reaches production readiness, contribution guidelines will be established.

---

## Documentation

All documentation is in the [`docs/`](./docs/) directory.

### Key Documents

**For Everyone:**
- [Project Overview](./docs/README.md)
- [Quick Start Guide](./docs/fruti/language/Guides/Quick-Start.md)

**For Developers:**
- [Language Design Decisions](./docs/fruti/Language-Design-Decisions.md)
- [Aero OS Technical Spec](./docs/aero/Aero-OS-Technical-Spec.md)
- [Standard Library Reference](./docs/fruti/language/Reference/Standard-Library.md)
- [Ownership Deep Dive](./docs/fruti/language/Guides/Ownership-Deep-Dive.md)

**For Contributors:**
- [Contributing Guide](./CONTRIBUTING.md)
- [Compiler Architecture](./packages/fruti-compiler/README.md)
- [Kernel Architecture](./packages/aero-kernel/README.md)

**Project Status:**
- [Phase 1 MVP Complete](./docs/project/management/PHASE-1-MVP-COMPLETE.md)
- [Project Management](./docs/project/management/README.md)

---

## Why Fruti?

### Design Goals

Fruti aims to be a **general-purpose language** that addresses common frustrations developers face across different programming paradigms. While ambitious, the goal is approached humbly: learning from decades of language design to create something that feels natural and complete.

### Pain Points Addressed

**From C/C++:**
- Memory safety without garbage collection
- No undefined behavior
- Modern module system
- Helpful error messages

**From Rust:**
- Faster compilation through incremental builds
- Simpler ownership model (automatic borrowing)
- Automatic lifetime inference
- Gentler learning curve

**From Python:**
- Native compiled performance (comparable to C++)
- Static type checking
- True parallelism
- Built-in package manager

**From Go:**
- Full generic programming
- Advanced type system
- More expressive syntax
- Better error handling

---

## Why Aero OS?

### What Makes Aero Different

**Security:**
- Capability-based (no root user)
- Sandboxing by default
- Verified boot chain
- Privacy-first networking

**Performance:**
- Fast boot time (target: < 2 seconds)
- Improved battery efficiency through better resource management
- Efficient microkernel architecture
- Minimal resource usage

**Compatibility:**
- Runs Linux binaries (target: near-native performance)
- Runs Windows apps (Wine-based compatibility layer)
- Native Fruti applications
- Web applications as first-class citizens

**User Experience:**
- Zero configuration
- Self-healing and auto-updating
- Beautiful, consistent interface
- Works offline-first

---

## Project Status

### Fruti Language

- [x] Language design completed
- [x] Specifications written (all ambiguities resolved)
- [x] Comprehensive documentation created
- [x] Standard library reference documented
- [x] Code examples (5 complete programs)
- [ ] **Compiler implementation (lexer, parser, semantic analysis)** - Current Focus
- [ ] LLVM backend
- [ ] Standard library implementation
- [ ] Package manager
- [ ] IDE support (VS Code extension)

**Current Phase:** MVP Development - Building basic compiler infrastructure

**Realistic Timeline** (Solo Developer):
- **Year 1:** MVP compiler (Hello World compiles)
- **Year 2:** Core language features
- **Year 3:** Standard library, tooling
- **Year 4+:** Community building, ecosystem

### Aero OS

- [x] Architecture designed
- [x] Technical specifications written (120 syscalls defined)
- [x] Documentation created
- [x] Security model (capability-based)
- [x] IPC design specified
- [ ] Microkernel implementation
- [ ] Core system services
- [ ] Device drivers
- [ ] Desktop environment
- [ ] Linux binary compatibility

**Current Phase:** Design Complete - Implementation on hold (focus on compiler first)

**Realistic Timeline** (Solo Developer):
- **Years 1-3:** Focus on Fruti compiler
- **Years 4-6:** Begin kernel implementation
- **Years 7+:** System services and drivers

---

## Roadmap (Solo Developer Reality)

### 2025-2026: Foundation (Compiler MVP)
- Lexer and parser implementation
- Basic type checking
- Simple code generation (LLVM IR)
- Hello World program compiles and runs
- Core language features working

### 2027-2028: Language Maturity
- Full language feature implementation
- Ownership and borrow checking
- Basic standard library
- Error messages and tooling
- Self-hosting experiments

### 2029-2030: Ecosystem Building
- Package manager design
- IDE integration (LSP)
- Documentation generator
- Growing standard library
- Community formation (if interest exists)

### 2031+: OS Development (If Compiler Succeeds)
- Begin microkernel implementation in Fruti
- Core services
- Basic driver support
- Proof-of-concept OS

**Note:** These are aspirational timelines. Progress will be made incrementally as a learning project.

---

## Contributing

**Current Status:** This is a solo learning project. The project is not yet ready for external contributions.

### Future Plans

Once the compiler reaches MVP status (Hello World compiles), contribution guidelines will be established for:

**Documentation:**
- Fix typos and grammar
- Add examples
- Write tutorials

**Code:** (When ready)
- Implement compiler features
- Write tests
- Fix bugs

**Interested in following along?**
- Star the repository for updates
- Read the design documentation
- Share feedback via GitHub Discussions (when enabled)

See [CONTRIBUTING.md](./CONTRIBUTING.md) for more details.

---

## Community

**Current Status:** Community infrastructure will be established once the project has working code to share.

### Planned Community Spaces (Future)

- **GitHub:** Repository and discussions
- **Discord/Forum:** When there's a community to support
- **Blog:** Development updates and learning experiences

**For Now:** 
- Follow the repository for updates
- Read the documentation and design decisions
- Watch this space for progress announcements

---

## License

### Fruti Language
- Compiler: MIT License
- Standard Library: MIT License
- Documentation: CC BY 4.0

### Aero OS
- Kernel: GPL v3
- System Services: GPL v3
- Applications: MIT License
- Documentation: CC BY 4.0

See [LICENSE](./LICENSE) for details.

---

## Acknowledgments

Frutisoft builds on decades of programming language and operating system research.

**Inspired by:**
- Rust (ownership model, type system)
- Python (syntax philosophy, ease of use)
- Go (simplicity, fast compilation)
- C (performance, low-level control)
- Kotlin (modern syntax, pragmatism)
- seL4 (verified microkernel)
- Plan 9 (everything is a file)

---

## Contact

**GitHub Issues:** Best way to provide feedback or report issues (when project has runnable code)

**Note:** This is a learning project by a solo student developer. Response times may vary based on academic schedule.

---

## Support the Project

**Ways to Show Interest:**
- Star the repository to follow progress
- Read the documentation and provide feedback
- Share if you find the design interesting

**Note:** This is a learning project focused on education, not commercial sponsorship.

---

**Frutisoft © 2025 - Fresh code, crisp ideas**