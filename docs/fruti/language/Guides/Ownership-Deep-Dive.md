# Ownership Deep Dive

**Last Updated:** December 7, 2025

## WARNING - Implementation Status

**This guide describes design specifications for Fruti's ownership system. The actual compiler implementation is in early MVP development. Most features described here are planned but not yet fully implemented.**

**What's Implemented in Phase 1:**
- Basic move semantics
- Simple borrowing
- Basic type checking

**What's Planned (Phase 2+):**
- Full ownership validation
- Lifetime inference
- Comprehensive borrow checking
- Smart pointers

---

## Table of Contents

1. [Ownership Fundamentals](#ownership-fundamentals)
2. [Move Semantics](#move-semantics)
3. [Borrowing](#borrowing)
4. [Lifetimes](#lifetimes)
5. [Common Patterns](#common-patterns)
6. [Comparison with Rust](#comparison-with-rust)
7. [Best Practices](#best-practices)

---

## Ownership Fundamentals

### The Three Rules

Fruti's ownership system is based on three fundamental rules:

1. **Each value has a single owner**
2. **When the owner goes out of scope, the value is dropped**
3. **Ownership can be transferred (moved) or temporarily shared (borrowed)**

### Why Ownership?

**Memory Safety Without Garbage Collection:**
- No use-after-free
- No double-free
- No null pointer dereferences
- Predictable performance

**Clear Ownership Semantics:**
- Explicit about who owns what
- No hidden copies
- Obvious resource management

### Basic Example

```fruti
fn main() {
    let s1: str = "hello";  // s1 owns the string
    let s2: str = s1;       // ownership moves to s2
    // print(s1);           // ERROR: s1 no longer valid
    print(s2);              // OK: s2 is the owner
}  // s2 goes out of scope, string is dropped
```

---

## Move Semantics

### What is a Move?

When you assign a value to another variable, **ownership transfers**:

```fruti
let v1: Vec<i32> = vec![1, 2, 3];
let v2: Vec<i32> = v1;  // v1 moved to v2
// v1 is now invalid
```

**After the move:**
- `v2` owns the vector
- `v1` cannot be used
- No copying occurred

### Types That Move

**Most types move by default:**
- Strings (`str`)
- Vectors (`Vec<T>`)
- Custom structs
- Heap-allocated data

**Copy types don't move:**
- Integers (`i32`, `i64`, etc.)
- Floats (`f32`, `f64`)
- Booleans (`bool`)
- Small fixed-size types

### Explicit Moves

Use the `move` keyword to be explicit:

```fruti
fn take_ownership(v: Vec<i32>) {
    // v is owned here
}

fn main() {
    let data: Vec<i32> = vec![1, 2, 3];
    take_ownership(move data);  // explicitly move
    // data is no longer accessible
}
```

### Move in Functions

**Passing by value moves ownership:**

```fruti
fn process(s: str) -> str {
    // s is owned by this function
    let result: str = s + " processed";
    return result;  // ownership moves to caller
}

fn main() {
    let text: str = "hello";
    let processed: str = process(text);  // text moved
    // text is invalid here
    print(processed);  // OK
}
```

---

## Borrowing

### Immutable Borrows

**Borrow without taking ownership:**

```fruti
fn print_length(s: &str) {
    // s is borrowed, not owned
    print(s.len());
}

fn main() {
    let text: str = "hello";
    print_length(&text);  // borrow text
    print(text);          // text still valid!
}
```

**Rules for Immutable Borrows:**
- Any number of immutable borrows allowed
- Original value cannot be modified
- Borrows must not outlive the owner

### Mutable Borrows

**Exclusive mutable access:**

```fruti
fn append(s: &mut str, suffix: str) {
    s.push_str(suffix);
}

fn main() {
    let mut text: str = "hello";
    append(&mut text, " world");
    print(text);  // "hello world"
}
```

**Rules for Mutable Borrows:**
- Only one mutable borrow at a time
- No immutable borrows while mutably borrowed
- Ensures data race freedom

### Borrow Rules Summary

```fruti
let mut x: i32 = 5;

// Multiple immutable borrows: OK
let r1: &i32 = &x;
let r2: &i32 = &x;
print(r1 + r2);

// Immutable and mutable borrow: ERROR
let r3: &i32 = &x;
let r4: &mut i32 = &mut x;  // ERROR!

// Multiple mutable borrows: ERROR
let r5: &mut i32 = &mut x;
let r6: &mut i32 = &mut x;  // ERROR!

// Single mutable borrow: OK
let r7: &mut i32 = &mut x;
*r7 = 10;
```

### Borrow Scopes

Borrows are valid for their scope:

```fruti
fn main() {
    let mut data: str = "hello";
    
    {
        let r: &str = &data;
        print(r);
    }  // r goes out of scope
    
    // Now we can borrow mutably
    let r2: &mut str = &mut data;
    r2.push_str(" world");
}
```

---

## Lifetimes

### What Are Lifetimes?

**Lifetimes ensure references don't outlive their data:**

```fruti
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}
```

**The `'a` annotation means:**
- Return value lives as long as both inputs
- Compiler ensures no dangling references

### Lifetime Elision

**Simple cases don't need explicit lifetimes:**

```fruti
// Explicit lifetime
fn first_word<'a>(s: &'a str) -> &'a str {
    // ...
}

// Elided (compiler infers)
fn first_word(s: &str) -> &str {
    // ...
}
```

**Elision Rules:**
1. Each input parameter gets its own lifetime
2. If there's one input lifetime, it's assigned to all outputs
3. If there's a `&self` or `&mut self`, its lifetime is assigned to all outputs

### Struct Lifetimes

**Structs can hold references:**

```fruti
struct TextAnalyzer<'a> {
    text: &'a str,
    position: usize,
}

impl<'a> TextAnalyzer<'a> {
    fn new(text: &'a str) -> TextAnalyzer<'a> {
        return TextAnalyzer {
            text: text,
            position: 0,
        };
    }
    
    fn advance(&mut self) {
        self.position += 1;
    }
}
```

### Static Lifetime

**`'static` means the reference lives for the entire program:**

```fruti
let s: &'static str = "I live forever";

static GLOBAL: i32 = 42;  // Has 'static lifetime
```

---

## Common Patterns

### Pattern 1: Returning Ownership

```fruti
fn create_vec() -> Vec<i32> {
    let v: Vec<i32> = vec![1, 2, 3];
    return v;  // ownership transferred to caller
}

fn main() {
    let numbers: Vec<i32> = create_vec();
    // numbers owns the vector
}
```

### Pattern 2: Borrow and Return

```fruti
fn find_max(numbers: &Vec<i32>) -> i32 {
    let mut max: i32 = numbers[0];
    for n in numbers {
        if *n > max {
            max = *n;
        }
    }
    return max;
}

fn main() {
    let nums: Vec<i32> = vec![1, 5, 3, 9, 2];
    let maximum: i32 = find_max(&nums);
    // nums still valid!
    print(maximum);
}
```

### Pattern 3: Builder Pattern

```fruti
struct Config {
    name: str,
    port: i32,
    debug: bool,
}

impl Config {
    fn new(name: str) -> Config {
        return Config {
            name: name,
            port: 8080,
            debug: false,
        };
    }
    
    fn set_port(mut self, port: i32) -> Config {
        self.port = port;
        return self;  // ownership moves back
    }
    
    fn set_debug(mut self, debug: bool) -> Config {
        self.debug = debug;
        return self;
    }
}

fn main() {
    let config: Config = Config::new("server")
        .set_port(3000)
        .set_debug(true);
}
```

### Pattern 4: Smart Pointers

```fruti
// Reference counted pointer for shared ownership
fn share_data() {
    let data: Rc<str> = Rc::new("shared");
    let ref1: Rc<str> = data.clone();  // increment count
    let ref2: Rc<str> = data.clone();  // increment count
    // All three refs point to same data
}  // count drops to zero, data freed

// Box for heap allocation
fn heap_allocate() {
    let large_data: Box<[i32; 1000]> = Box::new([0; 1000]);
    // Stored on heap, not stack
}
```

---

## Comparison with Rust

### Similarities

**Core Concepts:**
- Ownership rules are the same
- Move semantics work identically
- Borrow checker enforces safety
- Lifetimes prevent dangling references

### Differences

**Fruti Simplifications:**

1. **Explicit `move` keyword (optional but encouraged):**
   ```fruti
   // Rust: implicit
   let v2 = v1;
   
   // Fruti: can be explicit
   let v2: Vec<i32> = move v1;
   ```

2. **Simpler lifetime syntax (where possible):**
   ```fruti
   // Fruti aims for more inference
   fn process(s: &str) -> &str {
       // Lifetime inferred
   }
   ```

3. **Error messages focus on pedagogy:**
   - Explain why the borrow checker rejected code
   - Suggest concrete fixes
   - Link to documentation

**Rust Features Not (Yet) in Fruti:**
- Pin/Unpin
- Unsafe blocks (may add later)
- Some advanced lifetime patterns
- Zero-cost abstractions guarantees (still optimizing)

---

## Best Practices

### 1. Prefer Borrowing

**Instead of moving:**
```fruti
fn calculate(data: Vec<i32>) -> i32 {
    // Takes ownership, caller loses access
}
```

**Borrow when possible:**
```fruti
fn calculate(data: &Vec<i32>) -> i32 {
    // Borrows, caller retains access
}
```

### 2. Return Ownership for New Data

```fruti
fn create_user(name: str, age: i32) -> User {
    return User { name: name, age: age };
    // Caller gets ownership of new User
}
```

### 3. Use Mutable Borrows for Modification

```fruti
fn update_score(player: &mut Player, points: i32) {
    player.score += points;
    // Modifies in place, no copy
}
```

### 4. Clone When You Need To

**Sometimes copying is the right choice:**
```fruti
fn backup(data: &Vec<i32>) -> Vec<i32> {
    return data.clone();  // Explicit copy
}
```

### 5. Understand Copy Types

**Copy types don't move:**
```fruti
let x: i32 = 5;
let y: i32 = x;  // x is copied, not moved
print(x);        // x still valid
```

### 6. Use Type Annotations

**Be explicit about ownership:**
```fruti
let owned: str = "hello";
let borrowed: &str = &owned;
let mut_borrowed: &mut str = &mut owned;
```

### 7. Scope Borrowsnarrowly

**End borrows as soon as possible:**
```fruti
fn process(data: &mut Vec<i32>) {
    {
        let r: &Vec<i32> = data;
        print(r);
    }  // r dropped
    
    // Now we can mutably borrow again
    data.push(42);
}
```

---

## Common Errors and Solutions

### Error: Use After Move

```fruti
let s1: str = "hello";
let s2: str = s1;
print(s1);  // ERROR: s1 was moved
```

**Solution:** Clone or borrow:
```fruti
// Option 1: Clone
let s1: str = "hello";
let s2: str = s1.clone();
print(s1);  // OK

// Option 2: Borrow
let s1: str = "hello";
let s2: &str = &s1;
print(s1);  // OK
```

### Error: Multiple Mutable Borrows

```fruti
let mut v: Vec<i32> = vec![1, 2, 3];
let r1: &mut Vec<i32> = &mut v;
let r2: &mut Vec<i32> = &mut v;  // ERROR
```

**Solution:** Use borrows sequentially:
```fruti
let mut v: Vec<i32> = vec![1, 2, 3];
{
    let r1: &mut Vec<i32> = &mut v;
    r1.push(4);
}
let r2: &mut Vec<i32> = &mut v;
r2.push(5);
```

### Error: Borrow Outlives Owner

```fruti
fn get_ref() -> &str {
    let s: str = "hello";
    return &s;  // ERROR: s dropped when function returns
}
```

**Solution:** Return owned data:
```fruti
fn get_string() -> str {
    let s: str = "hello";
    return s;  // OK: ownership transferred
}
```

---

## Summary

**Key Takeaways:**

1. **Ownership** - Every value has exactly one owner
2. **Moves** - Assignment transfers ownership by default
3. **Borrowing** - Temporary access without taking ownership
4. **Lifetimes** - Ensure references don't outlive their data
5. **Safety** - No garbage collector, no use-after-free, no data races

**Mental Model:**
- Think of values as physical objects
- Only one person can own an object
- Others can borrow it temporarily
- When the owner leaves, the object is gone

**When to Use What:**
- **Own** - When you need full control
- **Borrow (&)** - When you need read access
- **Mutably Borrow (&mut)** - When you need write access
- **Clone** - When you genuinely need a copy

---

## Further Reading

- Language Reference: Type System
- Language Reference: Memory Management
- Examples: Ownership Patterns
- Compiler Error Index: Borrow Checker Errors

---

**Status:** Design specification complete, implementation in progress
**Last Updated:** December 7, 2025

---

**Frutisoft © 2025 - Fresh code, crisp ideas**
