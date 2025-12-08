# Contributing to Frutisoft

Thank you for your interest in the Fruti programming language and Aero OS!

---

## Current Status: Personal Passion Project

**This project is not yet ready for external contributions.** Frutisoft is currently a personal passion project by Jameson Jones (Illinois Mathematics and Science Academy) focused on compiler and OS development.

### Why Not Open for Contributions Yet?

1. **Core architecture is being established** - The fundamental design and implementation patterns need to be proven first
2. **No runnable code yet** - The compiler is in early MVP development; there's nothing to contribute to yet
3. **Personal exploration** - This is a passion project to deeply explore compiler and OS design
4. **Documentation priority** - Design documentation needs to stabilize before implementation contributions

### Future Plans

Once the compiler reaches **MVP status** (Hello World compiles and runs), contribution guidelines will be established for:

- **Bug fixes and testing**
- **Documentation improvements**
- **Standard library functions**
- **Tool development**
- **Example programs**

---

## For Now: How You Can Help

### 1. **Star the Repository**
Show interest and follow progress on GitHub

### 2. **Read the Documentation**
- [Language Design Decisions](./docs/fruti/Language-Design-Decisions.md)
- [Aero OS Technical Spec](./docs/aero/Aero-OS-Technical-Spec.md)
- [Code Examples](./docs/fruti/examples/)

### 3. **Provide Design Feedback**
- Comment on design decisions
- Suggest improvements to syntax or features
- Share use cases that might inform design

### 4. **Share**
If you find the project interesting, share it with others who might be interested in language or OS design

---

## Development Information (For Future Reference)

The sections below describe the intended development workflow once the project is ready for contributions.

---

## Code of Conduct

### Our Pledge

When the project opens for contributions, we'll maintain a welcoming community for all. Respect and constructive collaboration will be core values.

### Expected Behavior

- Be respectful and inclusive
- Welcome newcomers and help them get started
- Focus on what is best for the project
- Show empathy towards other community members
- Give and receive constructive feedback gracefully

### Unacceptable Behavior

- Harassment, discrimination, or offensive comments
- Personal attacks or trolling
- Publishing others' private information
- Any conduct that could reasonably be considered inappropriate

---

## Future: Getting Started

### Prerequisites (When Ready)

**For Fruti Compiler:**
- Rust 1.75+ (for bootstrapping)
- LLVM 17+
- CMake 3.20+
- Git

**For Aero Kernel:**
- Rust nightly (for low-level features)
- QEMU (for testing)
- GCC cross-compiler (optional, for C interop)

### First-Time Setup (Not Yet Functional)

```bash
# Clone the repository
git clone https://github.com/Frutisoft/frutisoft.git
cd frutisoft

# Build the compiler (when ready)
cd packages/fruti-compiler
cargo build --release

# Run tests (when tests exist)
cargo test
```
- Update documentation as needed

### 4. Test Thoroughly

```bash
# Run all tests
cargo test --all

# Run specific test suite
cargo test --package fruti-compiler

# Format code
cargo fmt --all

# Lint code
cargo clippy --all -- -D warnings
```

### 5. Commit and Push

```bash
# Stage changes
git add .

# Commit with clear message
git commit -m "feat: Add support for async iterators

- Implement AsyncIterator trait
- Add runtime support for async iteration
- Add tests and examples"

# Push to your fork
git push origin feature/your-feature-name
```

### 6. Open Pull Request

1. Go to GitHub and create Pull Request
2. Fill out PR template completely
3. Link related issues
4. Request review from maintainers
5. Respond to feedback promptly

---

## Coding Standards

### Rust Code (Compiler and Kernel)

```rust
// Use descriptive names
fn parse_function_declaration() -> Result<FunctionDecl> {
    // Implementation
}

// Document public APIs
/// Parses a Fruti source file into an AST.
///
/// # Arguments
/// * `source` - Source code as a string
/// * `filename` - Name of the file (for error messages)
///
/// # Returns
/// Parsed AST or compilation error
pub fn parse(source: &str, filename: &str) -> Result<Ast> {
    // Implementation
}

// Use Result for error handling
fn compile_file(path: &Path) -> Result<Vec<u8>, CompileError> {
    let source = fs::read_to_string(path)?;
    // ...
}

// Prefer explicit types for clarity
let count: usize = items.len();

// Use iterator methods
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
```

### Fruti Code (Examples and Tests)

```fruti
// Follow Fruti style guide
fn calculate_total(items: Vec<Item>) -> Float {
    items.iter()
        .map(|item| item.price)
        .sum()
}

// Clear variable names
let total_price = calculate_total(cart_items)
let user_count = database.count_users()

// Document public APIs
/// Sends an HTTP request and returns the response.
///
/// # Arguments
/// * `url` - The URL to request
/// * `method` - HTTP method (GET, POST, etc.)
///
/// # Returns
/// HTTP response or error
fn send_request(url: String, method: HttpMethod) -> Result<Response>
```

### Formatting

- **Rust**: Run `cargo fmt` before committing
- **Fruti**: Follow language style guide
- **Line length**: 100 characters max
- **Indentation**: 4 spaces (no tabs)
- **Imports**: Group by std, external crates, internal modules

---

## Testing Guidelines

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_tokens() {
        let source = "let x = 42";
        let tokens = lex(source).unwrap();
        
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].kind, TokenKind::Let);
        assert_eq!(tokens[1].kind, TokenKind::Identifier);
    }
}
```

### Integration Tests

```rust
// tests/integration/compile_test.rs
#[test]
fn test_hello_world_compilation() {
    let source = r#"
        fn main() {
            println("Hello, World!")
        }
    "#;
    
    let result = compile_to_binary(source);
    assert!(result.is_ok());
}
```

### Test Coverage

- Aim for 80%+ code coverage
- Test edge cases and error conditions
- Include regression tests for fixed bugs
- Document test intent clearly

---

## Documentation

### Code Documentation

- **Public APIs**: Must have doc comments
- **Complex algorithms**: Add inline comments explaining approach
- **Modules**: Include module-level documentation
- **Examples**: Provide usage examples in doc comments

### User Documentation

Located in `docs/`:

- **Guides**: Step-by-step tutorials
- **Reference**: Complete API documentation
- **Examples**: Practical code examples
- **Specifications**: Language and OS specifications

Update relevant docs when:
- Adding new features
- Changing behavior
- Fixing documentation errors

---

## Community

### Getting Help

- **GitHub Discussions**: Ask questions, share ideas
- **Stack Overflow**: Tag questions with `fruti` and `aero-os`

### Staying Updated

- **GitHub**: Watch repository for updates
- **Documentation**: Read the [docs](./docs/)

### Recognition

Contributors are recognized in:
- `CONTRIBUTORS.md` file
- Release notes
- Project website

---

## Project Structure

```
frutisoft/
├── fruti-compiler/          # Fruti compiler implementation
│   ├── src/
│   │   ├── lexer/          # Tokenization
│   │   ├── parser/         # AST generation
│   │   ├── semantic/       # Type checking, borrow checking
│   │   ├── codegen/        # LLVM IR generation
│   │   └── driver/         # Compiler driver
│   └── tests/              # Compiler tests
│
├── aero-kernel/            # Aero OS kernel
│   ├── src/               # Kernel source
│   └── drivers/           # Device drivers
│
├── docs/                   # All documentation
│   ├── fruti/            # Language documentation
│   ├── aero/             # OS documentation
│   ├── project/          # Project management
│   ├── resources/        # Development resources
│   ├── assets/           # Visual assets
│   └── archive/          # Historical documents
│
├── examples/              # Working code samples
├── website/              # GitHub Pages site
├── tools/                # Build scripts
└── .github/              # CI/CD workflows
```

---

## Pull Request Checklist

Before submitting, ensure:

- [ ] Code compiles without warnings
- [ ] All tests pass (`cargo test --all`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation is updated
- [ ] Commit messages are clear and descriptive
- [ ] PR description explains what and why
- [ ] Related issues are linked

---

## Release Process

Maintainers handle releases following semantic versioning:

- **Major** (1.0.0): Breaking changes
- **Minor** (0.1.0): New features, backward compatible
- **Patch** (0.0.1): Bug fixes

---

## Questions?

If you have questions about contributing:
1. Check existing documentation
2. Search GitHub issues
3. Ask in GitHub Discussions

Thank you for contributing to Frutisoft!

---

**Frutisoft © 2025 - Fresh code, crisp ideas**
