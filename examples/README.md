# Fruti Examples - Phase 1 MVP

**Runnable example programs for the current Fruti compiler implementation**

---

## Current Status: Phase 1 MVP

These examples **actually work** with the current compiler (Phase 1 MVP completed December 7, 2025). They demonstrate the implemented features:

- Variables and basic types (i32, f64, str, bool)
- Functions and control flow
- Basic operators
- Comments

**Looking for more advanced examples?** See `/docs/fruti/examples/` for design reference examples showing planned features (web servers, CLI tools, concurrency, etc.).

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
