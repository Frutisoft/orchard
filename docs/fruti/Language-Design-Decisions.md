# Fruti Language: Definitive Design Decisions

**Last Updated:** December 7, 2025

**Status:** Canonical Reference - Design Specification

**Implementation Status:** This is a complete design specification for the Fruti language. The compiler implementation is in early MVP development. All features described here are planned, but not yet implemented unless otherwise noted.

---

## Language Vision

Fruti aspires to be a **comprehensive general-purpose programming language** that learns from decades of language design. Rather than being narrowly focused on one domain, Fruti aims to address pain points across systems programming, application development, scripting, and more.

**Design Philosophy:**
- **Simple but not simplistic** - Easy to learn, powerful when needed
- **General-purpose by nature** - From CLI tools to web servers to systems programming
- **Comprehensive standard library** - Batteries included, no framework fatigue
- **Learn from the best** - Take inspiration from Rust's safety, Python's ergonomics, Go's simplicity

While this is an ambitious goal, it's approached with humility as a passion project. The aim is not to replace existing languages, but to explore what's possible when combining the best ideas from modern language design.

This document provides definitive answers to all language design questions, based on what developers value most: predictability, simplicity, and consistency.

---

## Implementation Roadmap

### Phase 1: MVP (Early Phase)

**Status:** COMPLETE (Recently)

**Features:**
- Basic syntax and semantics
- Core type system
- Functions and control flow
- Simple ownership rules
- LLVM code generation

**Deliverable:** Working compiler for basic programs

### Phase 2: Core Features (Mid Phase)

**Target:** Near Term

**Features:**
- Structs and enums
- Pattern matching
- Traits/interfaces
- Generic types
- Standard library basics

**Deliverable:** Usable for real projects

### Phase 3: Full Language (Later Phase)

**Target:** Mid Term

**Features:**
- Async/await
- Macros
- Advanced type features
- Full standard library
- Optimization passes

**Deliverable:** Production-ready compiler

---

## Table of Contents

1. [Syntax Design](#syntax-design)
2. [Type System](#type-system)
3. [Memory Management](#memory-management)
4. [Error Handling](#error-handling)
5. [Concurrency](#concurrency)
6. [Generics](#generics)
7. [Standard Library](#standard-library)
8. [Tooling](#tooling)

---

## Syntax Design

### General Principles

**1. Explicit Over Implicit**
```fruti
// Type annotations required where ambiguous
let x: i32 = 5;        // Clear
let message: str = "hello";  // Obvious

// But inference where unambiguous
let numbers = vec![1, 2, 3];  // Vec<i32> inferred
```

**2. Keywords Over Symbols**
```fruti
fn main() {
    // Use words for clarity
    if condition and other_condition {
        // ...
    }
    
    // Not symbols
    // if condition && other_condition { ... }  // Symbols also supported but not encouraged
}
```

**3. Consistent Style**
```fruti
// snake_case for variables and functions
let user_name: str = "Alice";
fn calculate_total() -> i32 { ... }

// PascalCase for types
struct UserAccount { ... }
enum ResponseType { ... }

// SCREAMING_SNAKE_CASE for constants
const MAX_CONNECTIONS: i32 = 100;
```

### Variable Declarations

**Immutable by Default:**
```fruti
let x: i32 = 5;     // Immutable
// x = 10;          // ERROR: cannot assign to immutable variable

let mut y: i32 = 5;  // Mutable
y = 10;              // OK
```

**Type Inference:**
```fruti
let x = 5;           // i32 inferred
let y = 3.14;        // f64 inferred
let s = "hello";     // str inferred
let v = vec![1, 2];  // Vec<i32> inferred
```

**Multiple Declarations:**
```fruti
let x: i32 = 5;
let y: i32 = 10;
let z: i32 = x + y;

// Or with type inference
let (a, b, c) = (1, 2, 3);
```

### Functions

**Function Syntax:**
```fruti
// Basic function
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

// No return value
fn print_message(msg: str) {
    print(msg);
}

// Implicit return (last expression)
fn multiply(a: i32, b: i32) -> i32 {
    a * b  // No semicolon = return value
}

// Early return
fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return Option::None;
    }
    return Option::Some(a / b);
}
```

**Parameters:**
```fruti
// By value (moves)
fn take_ownership(s: String) { ... }

// By reference (borrows)
fn borrow_value(s: &String) { ... }

// Mutable reference
fn modify_value(s: &mut String) { ... }

// Default parameters (planned Phase 2)
fn greet(name: str, greeting: str = "Hello") {
    println("{greeting}, {name}!");
}
```

### Control Flow

**If Expressions:**
```fruti
// If statement
if x > 0 {
    println("positive");
} else if x < 0 {
    println("negative");
} else {
    println("zero");
}

// If as expression
let result: str = if x > 0 {
    "positive"
} else {
    "not positive"
};
```

**Loops:**
```fruti
// While loop
while condition {
    // ...
}

// Infinite loop
loop {
    // ...
    if done {
        break;
    }
}

// For loop
for i in 0..10 {
    println("{i}");
}

// Iterate over collection
for item in collection {
    println("{item}");
}
```

**Match Expressions (Phase 2):**
```fruti
match value {
    0 => println("zero"),
    1..=5 => println("small"),
    _ => println("large"),
}

// With binding
match option {
    Option::Some(x) => println("value: {x}"),
    Option::None => println("no value"),
}
```

### Comments

```fruti
// Single-line comment

/*
   Multi-line comment
   spanning multiple lines
*/

/// Documentation comment for functions
/// Supports markdown formatting
fn documented_function() { ... }

//! Module-level documentation
//! Describes the entire module
```

---

## Type System

### Primitive Types

**Integers:**
```fruti
i8, i16, i32, i64    // Signed integers
u8, u16, u32, u64    // Unsigned integers

let small: i8 = 127;
let big: i64 = 1_000_000;
let byte: u8 = 255;
```

**Floating Point:**
```fruti
f32, f64             // 32-bit and 64-bit floats

let pi: f32 = 3.14;
let precise: f64 = 3.141592653589793;
```

**Boolean:**
```fruti
bool                 // true or false

let is_ready: bool = true;
let is_done: bool = false;
```

**Character and Strings:**
```fruti
char                 // Unicode scalar value
str                  // String slice (immutable)
String               // Owned string (mutable)

let c: char = 'A';
let s: str = "hello";
let mut owned: String = String::from("hello");
```

### Compound Types

**Arrays (Phase 2):**
```fruti
// Fixed-size array
let numbers: [i32; 5] = [1, 2, 3, 4, 5];
let first: i32 = numbers[0];

// Array with same value
let zeros: [i32; 100] = [0; 100];
```

**Tuples (Phase 2):**
```fruti
// Tuple with mixed types
let tuple: (i32, str, bool) = (42, "hello", true);

// Destructuring
let (x, y, z) = tuple;

// Access by index
let first = tuple.0;
```

**Structs (Phase 2):**
```fruti
// Define struct
struct Point {
    x: i32,
    y: i32,
}

// Create instance
let p: Point = Point { x: 10, y: 20 };

// Access fields
println("x: {p.x}, y: {p.y}");

// Tuple struct
struct Color(u8, u8, u8);
let red: Color = Color(255, 0, 0);
```

**Enums (Phase 2):**
```fruti
// Simple enum
enum Direction {
    North,
    South,
    East,
    West,
}

// Enum with data
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Usage
let result: Result<i32, str> = Result::Ok(42);
match result {
    Result::Ok(value) => println("Success: {value}"),
    Result::Err(error) => println("Error: {error}"),
}
```

### Type Inference

**Inference Rules:**
```fruti
// Infer from literal
let x = 5;           // i32
let y = 3.14;        // f64
let s = "hello";     // str

// Infer from function return
fn get_number() -> i32 { 42 }
let n = get_number();  // i32

// Infer from usage
let mut v = Vec::new();
v.push(1);           // Vec<i32> inferred

// Explicit when ambiguous
let x: i64 = 5;      // Force i64 instead of default i32
```

### Type Casting

```fruti
// Explicit casts with 'as'
let x: i32 = 5;
let y: f64 = x as f64;

// Fallible conversions (Phase 2)
let s: str = "42";
let n: i32 = s.parse()?;  // Returns Result
```

---

## Memory Management

### Ownership Rules

**The Three Rules:**
1. Each value has exactly one owner
2. When the owner goes out of scope, the value is dropped
3. Ownership can be moved or borrowed

**Move Semantics:**
```fruti
let s1: String = String::from("hello");
let s2: String = s1;  // s1 moved to s2
// println(s1);       // ERROR: s1 no longer valid
println(s2);          // OK
```

**Copy Types:**
```fruti
// Simple types are copied
let x: i32 = 5;
let y: i32 = x;  // x is copied, not moved
println(x);       // OK: x still valid
println(y);       // OK: y has its own copy
```

### Borrowing

**Immutable Borrows:**
```fruti
fn calculate_length(s: &String) -> usize {
    return s.len();  // Borrow, don't take ownership
}

let s: String = String::from("hello");
let len: usize = calculate_length(&s);
println(s);  // s still valid
```

**Mutable Borrows:**
```fruti
fn append_world(s: &mut String) {
    s.push_str(" world");
}

let mut s: String = String::from("hello");
append_world(&mut s);
println(s);  // "hello world"
```

**Borrow Rules:**
- Multiple immutable borrows: OK
- One mutable borrow: OK
- Immutable + mutable borrow: ERROR

```fruti
let mut s: String = String::from("hello");

// Multiple immutable borrows: OK
let r1: &String = &s;
let r2: &String = &s;
println("{r1} and {r2}");

// Mutable borrow: OK (after immutable borrows end)
let r3: &mut String = &mut s;
r3.push_str(" world");
```

### Lifetimes (Phase 2)

**Explicit Lifetimes:**
```fruti
// Function with lifetime annotations
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}
```

**Lifetime Elision:**
```fruti
// Compiler infers lifetimes
fn first_word(s: &str) -> &str {
    // Lifetime automatically inferred
}
```

### Smart Pointers (Phase 2)

**Box - Heap Allocation:**
```fruti
let b: Box<i32> = Box::new(5);
println("boxed value: {b}");
```

**Rc - Reference Counting:**
```fruti
let shared: Rc<String> = Rc::new(String::from("shared"));
let ref1: Rc<String> = shared.clone();
let ref2: Rc<String> = shared.clone();
// All three point to same data
```

**Arc - Atomic Reference Counting (Thread-Safe):**
```fruti
let shared: Arc<Vec<i32>> = Arc::new(vec![1, 2, 3]);
let shared_clone: Arc<Vec<i32>> = shared.clone();
// Can be shared across threads
```

---

## Error Handling

### Design Philosophy

**No Exceptions:**
- Errors are values, not control flow
- Explicit error handling required
- No hidden control flow

### Option<T>

**For Optional Values:**
```fruti
enum Option<T> {
    Some(T),
    None,
}

// Usage
fn find_user(id: i32) -> Option<User> {
    if user_exists(id) {
        return Option::Some(get_user(id));
    } else {
        return Option::None;
    }
}

// Handling
match find_user(42) {
    Option::Some(user) => println("Found: {user.name}"),
    Option::None => println("User not found"),
}
```

### Result<T, E>

**For Operations That Can Fail:**
```fruti
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Usage
fn divide(a: i32, b: i32) -> Result<i32, str> {
    if b == 0 {
        return Result::Err("Division by zero");
    }
    return Result::Ok(a / b);
}

// Handling
match divide(10, 2) {
    Result::Ok(value) => println("Result: {value}"),
    Result::Err(error) => println("Error: {error}"),
}
```

### Error Propagation

**The ? Operator (Phase 2):**
```fruti
fn read_username_from_file() -> Result<String, Error> {
    let mut file = File::open("username.txt")?;  // Propagate error
    let mut username = String::new();
    file.read_to_string(&mut username)?;         // Propagate error
    return Result::Ok(username);
}

// Equivalent to:
fn read_username_from_file() -> Result<String, Error> {
    let mut file = match File::open("username.txt") {
        Result::Ok(f) => f,
        Result::Err(e) => return Result::Err(e),
    };
    
    let mut username = String::new();
    match file.read_to_string(&mut username) {
        Result::Ok(_) => {},
        Result::Err(e) => return Result::Err(e),
    }
    
    return Result::Ok(username);
}
```

### Panic

**For Unrecoverable Errors:**
```fruti
// Explicit panic
panic("something went terribly wrong");

// Assert
assert(x > 0, "x must be positive");

// Unwrap (panics if None/Err)
let value: i32 = some_option.unwrap();
```

---

## Concurrency

### Design Goals

**Safe Concurrency:**
- No data races at compile time
- Ownership prevents sharing mutable state
- Clear synchronization primitives

### Threads (Phase 3)

**Spawn Threads:**
```fruti
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println("Hello from thread!");
    });
    
    handle.join().unwrap();
}
```

**Message Passing:**
```fruti
use std::sync::mpsc;

let (sender, receiver) = mpsc::channel();

thread::spawn(move || {
    sender.send("Hello").unwrap();
});

let message = receiver.recv().unwrap();
println("Received: {message}");
```

**Shared State:**
```fruti
use std::sync::{Arc, Mutex};

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter_clone = counter.clone();
    let handle = thread::spawn(move || {
        let mut num = counter_clone.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

println("Result: {}", *counter.lock().unwrap());
```

### Async/Await (Phase 3)

**Async Functions:**
```fruti
async fn fetch_data(url: str) -> Result<String, Error> {
    let response = http::get(url).await?;
    let body = response.text().await?;
    return Result::Ok(body);
}

// Usage
async fn main() {
    let data = fetch_data("https://example.com").await.unwrap();
    println(data);
}
```

**Concurrent Tasks:**
```fruti
use std::future::join;

async fn process_data() {
    let (result1, result2) = join!(
        fetch_data("url1"),
        fetch_data("url2")
    ).await;
    
    println("Results: {result1}, {result2}");
}
```

---

## Generics

### Generic Functions (Phase 2)

```fruti
fn largest<T>(list: &[T]) -> &T 
    where T: Ord
{
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    return largest;
}

// Usage
let numbers = vec![1, 5, 3, 9, 2];
let largest_number = largest(&numbers);
```

### Generic Structs (Phase 2)

```fruti
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        return Point { x: x, y: y };
    }
}

// Usage
let int_point: Point<i32> = Point::new(5, 10);
let float_point: Point<f64> = Point::new(1.0, 4.0);
```

### Traits (Phase 2)

**Define Trait:**
```fruti
trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        return format!("{}: {}", self.title, self.content);
    }
}
```

**Trait Bounds:**
```fruti
fn print_summary<T>(item: &T) 
    where T: Summary
{
    println(item.summarize());
}
```

---

## Standard Library

### Core Modules

**Prelude (Auto-imported):**
- Basic types: `Option`, `Result`, `Vec`, `String`
- Common traits: `Clone`, `Copy`, `Debug`
- I/O: `print`, `println`, `read_line`

**Collections:**
```fruti
use std::collections::{Vec, HashMap, HashSet};
```

**I/O:**
```fruti
use std::io::{File, stdin, stdout};
use std::fs;
```

**Networking:**
```fruti
use std::net::{TcpListener, TcpStream, UdpSocket};
```

**Concurrency:**
```fruti
use std::thread;
use std::sync::{Arc, Mutex, RwLock};
```

### Philosophy

**Batteries Included:**
- Rich standard library
- Common tasks covered
- Minimal external dependencies needed

**Quality Over Quantity:**
- Well-documented
- Consistent APIs
- Performance-optimized

---

## Tooling

### Compiler

**Command-Line Interface:**
```bash
# Compile
fruti build main.fruti

# Run directly
fruti run main.fruti

# Check syntax
fruti check main.fruti

# Generate IR
fruti build main.fruti --emit=llvm-ir
```

### Package Manager (Phase 2)

**Cargo-Inspired:**
```toml
[package]
name = "myproject"
version = "0.1.0"

[dependencies]
http = "1.0"
json = "0.5"
```

```bash
# Create new project
fruti new myproject

# Build project
fruti build

# Run tests
fruti test

# Publish package
fruti publish
```

### IDE Support (Phase 2)

**Language Server Protocol:**
- Autocomplete
- Go to definition
- Find references
- Inline documentation
- Error highlighting

**VS Code Extension:**
- Syntax highlighting
- Debugging support
- Integrated terminal

---

## Summary

Fruti is designed to be:

1. **Safe** - Memory safety through ownership
2. **Fast** - Zero-cost abstractions
3. **Productive** - Great tooling and libraries
4. **Simple** - Consistent and predictable
5. **General-Purpose** - Suitable for many domains

**Current Status:**
- Phase 1 Complete: Basic compiler working
- Phase 2 Planned: Core features
- Phase 3 Planned: Full language

**Learn More:**
- Language Reference: Complete syntax guide
- Standard Library: API documentation
- Examples: Real-world code samples
- Contributing: Join the development

---

**Last Updated:** December 7, 2025
**Status:** Canonical design specification

---

**Frutisoft © 2025 - Fresh code, crisp ideas**
