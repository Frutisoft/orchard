# Frutisoft Examples

This directory contains example programs demonstrating Fruti language features and Aero OS concepts.

## Fruti Language Examples

### Basic
- `hello-world.fruti` - Simple hello world program
- `basic-features.fruti` - Variables, control flow, functions

### Advanced (Coming Soon)
- Structs and enums
- Pattern matching
- Trait implementations
- Ownership examples
- Concurrent programming

## Aero OS Examples (Planned)

- Kernel module template
- System call examples
- IPC communication
- Device driver skeleton

## Running Examples

```bash
# From repository root
cd packages/fruti-compiler

# Check syntax
cargo run -- check ../../examples/hello-world.fruti

# Build to LLVM IR
cargo run -- build ../../examples/hello-world.fruti -o hello

# With verbose output
FRUTI_VERBOSE=1 cargo run -- check ../../examples/basic-features.fruti
```

## Contributing Examples

Feel free to add more examples! They help demonstrate language features and serve as learning resources.

---

**Frutisoft © 2025 - Fresh code, crisp ideas**
