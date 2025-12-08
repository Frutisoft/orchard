# Fruti Quick Start Guide

**Learn about Fruti's design in 10 minutes**

---

## Current Status

**This guide describes the intended design of Fruti.** The compiler is currently in early MVP development and does not yet support these features. Fruti is designed as a **general-purpose programming language** that learns from existing languages' pain points to create a simple, comprehensive solution. This is a learning project by a solo developer - check the [repository](https://github.com/Frutisoft/frutisoft) for current implementation status.

**Use this guide to:**
- Understand Fruti's design philosophy
- See what the language will look like when complete
- Learn about planned features and syntax
- Provide feedback on the design

---

## Installation (Not Yet Available)

**Current Status:** The Fruti compiler is in active development. There is no automated installer yet.

**To experiment with Fruti:**
1. Clone the repository: `git clone https://github.com/Frutisoft/frutisoft.git`
2. Follow build instructions in [packages/fruti-compiler/README.md](../../../../packages/fruti-compiler/README.md)
3. Build from source using Cargo

---

## Your First Program

Create `hello.fruti`:

```fruti
import std.io

fn main() {
    io.println("Hello, Fruti!")
}
```

Run it:

```bash
fruti run hello.fruti
# Output: Hello, Fruti!
```

---

## Project Setup

```bash
# Create new project
fruti new my-project
cd my-project

# Project structure created:
# my-project/
# ├── fruti.toml      # Project configuration
# ├── src/
# │   └── main.fruti  # Main source file
# └── tests/
#     └── tests.fruti # Test file
```

---

## Basic Syntax

### Variables
```fruti
// Immutable (default)
let name = "Alice"
let age = 30

// Mutable
var counter = 0
counter += 1

// Type annotations (optional)
let explicit: Int = 42
```

### Functions
```fruti
// Simple function
fn greet(name: String) -> String {
    "Hello, {name}!"
}

// Call it
let message = greet("World")
io.println(message)
```

### Control Flow
```fruti
// If expression
let status = if age >= 18 {
    "adult"
} else {
    "minor"
}

// Loops
for i in 0..10 {
    io.println(i)
}

while condition {
    // ...
}
```

### Collections
```fruti
// Lists
let numbers = [1, 2, 3, 4, 5]
let first = numbers[0]

// Maps
let ages = {
    "Alice": 30,
    "Bob": 25
}

// List comprehension
let squares = [x * x for x in numbers]
```

---

## Error Handling

```fruti
// Result type for errors
fn divide(a: Int, b: Int) -> Result<Int, String> {
    if b == 0 {
        Error("Division by zero")
    } else {
        Ok(a / b)
    }
}

// Using ? operator
fn complex() -> Result<Int, String> {
    let x = divide(10, 2)?
    let y = divide(20, 4)?
    Ok(x + y)
}

// Match on Result
match divide(10, 0) {
    Ok(result) -> io.println("Result: {result}")
    Error(msg) -> io.println("Error: {msg}")
}
```

---

## Structs and Traits

```fruti
// Define a struct
struct Person {
    name: String
    age: Int
}

// Create instance
let person = Person {
    name: "Alice",
    age: 30
}

// Define a trait
trait Greet {
    fn greet(self) -> String
}

// Implement trait
impl Greet for Person {
    fn greet(self) -> String {
        "Hello, I'm {self.name}"
    }
}

// Use it
io.println(person.greet())
```

---

## Building and Running

```bash
# Run directly
fruti run src/main.fruti

# Build executable
fruti build
./target/my-project

# Build optimized
fruti build --release

# Run tests
fruti test

# Format code
fruti fmt

# Check without building
fruti check
```

---

## Adding Dependencies

Edit `fruti.toml`:

```toml
[package]
name = "my-project"
version = "0.1.0"

[dependencies]
http = "2.0"
json = "1.5"
```

Install dependencies:

```bash
fruti add http@2.0
# or
fruti update
```

Use in code:

```fruti
import http

async fn main() {
    let response = http.get("https://api.example.com").await?
    io.println(response.text())
}
```

---

## Next Steps

- **[Language Reference](../Reference/README.md)** - Complete language documentation
- **[Standard Library](../Reference/Standard-Library.md)** - Built-in modules and functions
- **[Examples](../../examples/)** - More code examples
- **[Language Design Decisions](../../Language-Design-Decisions.md)** - Complete design rationale
- **[GitHub Repository](https://github.com/Frutisoft/frutisoft)** - Source code and development progress

---

**Happy coding with Fruti!**

---

**Frutisoft © 2025 - Fresh code, crisp ideas**
