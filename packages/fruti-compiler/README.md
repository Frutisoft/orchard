# Fruti Compiler

**The reference implementation of the Fruti programming language compiler**

---

## Overview

The Fruti compiler is a **passion project** by Jameson Jones (Illinois Mathematics and Science Academy) translating Fruti source code to LLVM IR. Phase 1 MVP is complete with a functional compilation pipeline. This is a **personal project** demonstrating compiler construction principles.

### Current Status: Phase 1 MVP Complete ✅

**Phase 1 Implemented:**
- ✅ Lexer - Complete tokenization (~553 lines)
- ✅ Parser - Full AST generation (~1,096 lines)
- ✅ Semantic Analyzer - Type checking (~653 lines)
- ✅ Code Generator - LLVM IR output (~308 lines)
- ✅ CLI - Compile, check, analyze commands
- ✅ Tests - 7 passing unit tests

**Total:** ~2,600 lines of compiler code + ~1,200 lines supporting code

**Phase 2 Planned:**
- [ ] Pattern matching
- [ ] Advanced type features (generics, traits)
- [ ] Expanded standard library
- [ ] Improved error messages

**Future Goals** (Post-MVP):
- Additional targets (WASM, ARM)
- Optimization passes
- Incremental compilation
- Full standard library

---

## Building

### Prerequisites

**Minimum Requirements:**
- Rust 1.75+ (for bootstrap compiler)
- Git

**Future Requirements** (for LLVM integration):
- LLVM 17+
- CMake 3.20+

### Build Steps

```bash
# Clone repository
git clone https://github.com/Frutisoft/frutisoft.git
cd frutisoft/packages/fruti-compiler

# Build release version
cargo build --release

# Binary will be at: target/release/fruti
```

### Development Build

```bash
# Fast debug build
cargo build

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run -- compile example.fruti
```

---

## Usage

### Compile a Program

```bash
# Compile to native binary
fruti build main.fruti

# Compile with optimizations
fruti build --release main.fruti

# Specify output file
fruti build main.fruti -o myprogram
```

### Run Directly

```bash
# Compile and run in one step
fruti run main.fruti

# With arguments
fruti run main.fruti -- arg1 arg2
```

### Future Compilation Targets

**Note:** These are planned for future phases. MVP focuses on native x86_64/ARM64.

```bash
# Compile to WebAssembly (Phase 3)
fruti build --target wasm32 main.fruti

# Compile to JavaScript (Phase 3+)
fruti build --target js main.fruti

# Compile to JVM bytecode (Phase 3+)
fruti build --target jvm main.fruti
```

**Current Focus:** Native compilation to x86_64 and ARM64 via LLVM.

### Check Without Building

```bash
# Type check only
fruti check main.fruti

# Format code
fruti fmt main.fruti

# Lint code
fruti lint main.fruti
```

---

## Architecture

```
Source Code
|
Lexer (tokenization)
|
Parser (AST generation)
|
Semantic Analysis (type checking, borrow checking)
|
MIR (Mid-level IR - optimization)
|
LLVM IR Generation
|
LLVM Optimization Passes
|
Native Code / WASM / JS / JVM
```

### Directory Structure

```
fruti-compiler/
├── src/
│   ├── lexer/ # Tokenization
│   ├── parser/ # Syntax analysis
│   ├── ast/ # Abstract syntax tree
│   ├── semantic/ # Type & borrow checking
│   ├── mir/ # Mid-level IR
│   ├── codegen/ # LLVM code generation
│   ├── driver/ # Compiler driver
│   └── main.rs # Entry point
├── tests/ # Integration tests
└── Cargo.toml # Dependencies
```

---

## Testing

```bash
# Run all tests
cargo test

# Run specific test suite
cargo test --test lexer_tests

# Run with output
cargo test -- --nocapture

# Run benchmarks
cargo bench
```

---

## Development Roadmap

### Phase 1: MVP (Current Focus)
- [ ] Complete lexer with comprehensive tests
- [ ] Implement parser for basic language constructs
- [ ] Basic type checking
- [ ] Simple code generation (output to C or LLVM IR)
- [ ] Compile "Hello, World!" program

### Phase 2: Core Features
- [ ] Full language syntax support
- [ ] Ownership and borrow checking
- [ ] Complete type inference
- [ ] Standard library (minimal subset)

### Phase 3: Maturity
- [ ] Optimization passes
- [ ] Improved error messages
- [ ] Language server protocol (LSP)
- [ ] Package manager integration

---

## Contributing

**Note:** This is currently a personal passion project. Contributions may be accepted in the future once the core compiler reaches a stable state. For now, feel free to watch the repository and provide feedback through issues.

### Quick Start

```bash
# Make changes
git checkout -b feature/my-feature

# Test changes
cargo test

# Format and lint
cargo fmt
cargo clippy

# Submit PR
```

---

## Performance Goals

**Note:** Compiler is in early development. Performance metrics are aspirational targets.

**Compilation Speed Goals:**
- Fast iteration for development
- Incremental compilation when mature
- Comparable to other LLVM-based languages

**Binary Performance Goals:**
- Leverage LLVM optimization capabilities
- Zero-cost abstractions (design principle)
- Native performance comparable to C/Rust

---

## Documentation

- [Language Reference](../../docs/fruti/language/Reference/)
- [Language Design Decisions](../../docs/fruti/Language-Design-Decisions.md)
- [Code Examples](../../docs/fruti/examples/)

---

## License

MIT License - see [LICENSE](../../LICENSE)

---

**Frutisoft © 2025 - Fresh code, crisp ideas**
