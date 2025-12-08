# Fruti Standard Library Reference

**Last Updated:** December 7, 2025

**Status:** Design Specification

---

## WARNING - Implementation Status

**This document describes the planned standard library for Fruti. The current compiler (Phase 1 MVP) does NOT include most of these features.**

**Implementation Status:**
- Phase 1 (Current): Basic primitives only
- Phase 2 (Planned): Core collections and I/O
- Phase 3 (Later): Full standard library

This is a design specification that will guide future implementation.

---

## Table of Contents

1. [Prelude](#prelude)
2. [Core Module](#core)
3. [Collections](#collections)
4. [I/O](#io)
5. [Strings](#strings)
6. [File System](#filesystem)
7. [Networking](#networking)
8. [Concurrency](#concurrency)
9. [Time](#time)
10. [Math](#math)

---

## Prelude

The prelude is automatically imported into every Fruti program.

### Types

```fruti
// Primitive types (always available)
bool, i8, i16, i32, i64, u8, u16, u32, u64, f32, f64
str, char

// Common types
Option<T>, Result<T, E>
Vec<T>, String
```

### Functions

```fruti
// Console I/O
fn print(s: str)
fn println(s: str)
fn eprint(s: str)  // print to stderr
fn eprintln(s: str)

// Input
fn read_line() -> str
fn read_input(prompt: str) -> str

// Panic
fn panic(message: str) -> never
fn assert(condition: bool, message: str)
```

### Traits (Planned)

```fruti
trait Debug {
    fn fmt(f: &mut Formatter) -> Result<(), Error>;
}

trait Clone {
    fn clone(&self) -> Self;
}

trait Copy {}  // Marker trait

trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

---

## Core

The `core` module contains fundamental types and operations.

### Option<T>

```fruti
enum Option<T> {
    Some(T),
    None,
}

impl<T> Option<T> {
    fn is_some(&self) -> bool
    fn is_none(&self) -> bool
    fn unwrap(self) -> T  // Panics if None
    fn unwrap_or(self, default: T) -> T
    fn map<U>(self, f: fn(T) -> U) -> Option<U>
    fn and_then<U>(self, f: fn(T) -> Option<U>) -> Option<U>
}
```

**Example:**
```fruti
fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return Option::None;
    }
    return Option::Some(a / b);
}

let result: Option<i32> = divide(10, 2);
match result {
    Option::Some(value) => println("Result: {value}"),
    Option::None => println("Cannot divide by zero"),
}
```

### Result<T, E>

```fruti
enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    fn is_ok(&self) -> bool
    fn is_err(&self) -> bool
    fn unwrap(self) -> T  // Panics if Err
    fn unwrap_or(self, default: T) -> T
    fn map<U>(self, f: fn(T) -> U) -> Result<U, E>
    fn map_err<F>(self, f: fn(E) -> F) -> Result<T, F>
}
```

**Example:**
```fruti
fn parse_number(s: str) -> Result<i32, str> {
    // Parsing logic
    if valid {
        return Result::Ok(number);
    } else {
        return Result::Err("Invalid number format");
    }
}
```

### Pointer Types

```fruti
// Box - Heap allocation
struct Box<T> {
    // Owns T on the heap
}

impl<T> Box<T> {
    fn new(value: T) -> Box<T>
    fn as_ref(&self) -> &T
    fn as_mut(&mut self) -> &mut T
}

// Rc - Reference counted
struct Rc<T> {
    // Shared ownership
}

impl<T> Rc<T> {
    fn new(value: T) -> Rc<T>
    fn clone(&self) -> Rc<T>  // Increment count
    fn strong_count(this: &Rc<T>) -> usize
}

// Arc - Atomic reference counted (thread-safe)
struct Arc<T> {
    // Shared ownership across threads
}

impl<T> Arc<T> {
    fn new(value: T) -> Arc<T>
    fn clone(&self) -> Arc<T>
}
```

---

## Collections

### Vec<T>

Dynamic array with contiguous storage.

```fruti
struct Vec<T> {
    // Dynamic array implementation
}

impl<T> Vec<T> {
    // Construction
    fn new() -> Vec<T>
    fn with_capacity(capacity: usize) -> Vec<T>
    
    // Capacity
    fn len(&self) -> usize
    fn is_empty(&self) -> bool
    fn capacity(&self) -> usize
    fn reserve(&mut self, additional: usize)
    
    // Modification
    fn push(&mut self, value: T)
    fn pop(&mut self) -> Option<T>
    fn insert(&mut self, index: usize, element: T)
    fn remove(&mut self, index: usize) -> T
    fn clear(&mut self)
    
    // Access
    fn get(&self, index: usize) -> Option<&T>
    fn get_mut(&mut self, index: usize) -> Option<&mut T>
    fn first(&self) -> Option<&T>
    fn last(&self) -> Option<&T>
    
    // Iteration
    fn iter(&self) -> Iter<T>
    fn iter_mut(&mut self) -> IterMut<T>
}
```

**Example:**
```fruti
let mut numbers: Vec<i32> = Vec::new();
numbers.push(1);
numbers.push(2);
numbers.push(3);

for n in numbers.iter() {
    println("{n}");
}
```

### HashMap<K, V>

Hash table for key-value storage.

```fruti
struct HashMap<K, V> {
    // Hash table implementation
}

impl<K, V> HashMap<K, V> {
    fn new() -> HashMap<K, V>
    fn with_capacity(capacity: usize) -> HashMap<K, V>
    
    fn insert(&mut self, key: K, value: V) -> Option<V>
    fn get(&self, key: &K) -> Option<&V>
    fn get_mut(&mut self, key: &K) -> Option<&mut V>
    fn remove(&mut self, key: &K) -> Option<V>
    fn contains_key(&self, key: &K) -> bool
    
    fn len(&self) -> usize
    fn is_empty(&self) -> bool
    fn clear(&mut self)
    
    fn keys(&self) -> Keys<K, V>
    fn values(&self) -> Values<K, V>
    fn iter(&self) -> Iter<K, V>
}
```

**Example:**
```fruti
let mut scores: HashMap<str, i32> = HashMap::new();
scores.insert("Alice", 100);
scores.insert("Bob", 85);

let alice_score: Option<&i32> = scores.get("Alice");
```

### HashSet<T>

Hash-based set (unique values).

```fruti
struct HashSet<T> {
    // Hash set implementation
}

impl<T> HashSet<T> {
    fn new() -> HashSet<T>
    fn insert(&mut self, value: T) -> bool
    fn remove(&mut self, value: &T) -> bool
    fn contains(&self, value: &T) -> bool
    fn len(&self) -> usize
    fn is_empty(&self) -> bool
    fn clear(&mut self)
}
```

---

## I/O

### Console I/O

```fruti
mod io {
    fn print(s: str)
    fn println(s: str)
    fn eprint(s: str)
    fn eprintln(s: str)
    
    fn read_line() -> Result<String, Error>
    fn read_to_string() -> Result<String, Error>
}
```

### File I/O

```fruti
struct File {
    // File handle
}

impl File {
    fn open(path: str) -> Result<File, Error>
    fn create(path: str) -> Result<File, Error>
    fn read_to_string(&mut self) -> Result<String, Error>
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize, Error>
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>
    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>
}
```

**Example:**
```fruti
let mut file: File = File::open("data.txt")?;
let contents: String = file.read_to_string()?;
println(contents);
```

### Buffered I/O

```fruti
struct BufReader<R> {
    // Buffered reader
}

impl<R> BufReader<R> {
    fn new(inner: R) -> BufReader<R>
    fn read_line(&mut self, buf: &mut String) -> Result<usize, Error>
    fn lines(&mut self) -> Lines<R>
}

struct BufWriter<W> {
    // Buffered writer
}

impl<W> BufWriter<W> {
    fn new(inner: W) -> BufWriter<W>
    fn flush(&mut self) -> Result<(), Error>
}
```

---

## Strings

### String

Owned, growable UTF-8 string.

```fruti
struct String {
    // Owned string data
}

impl String {
    fn new() -> String
    fn from(s: str) -> String
    fn with_capacity(capacity: usize) -> String
    
    fn push(&mut self, ch: char)
    fn push_str(&mut self, string: str)
    fn pop(&mut self) -> Option<char>
    fn clear(&mut self)
    
    fn len(&self) -> usize
    fn is_empty(&self) -> bool
    fn capacity(&self) -> usize
    
    fn as_str(&self) -> &str
    fn chars(&self) -> Chars
    fn bytes(&self) -> Bytes
    
    fn split(&self, delimiter: char) -> Split
    fn lines(&self) -> Lines
    fn trim(&self) -> &str
    fn to_uppercase(&self) -> String
    fn to_lowercase(&self) -> String
}
```

### str (String Slice)

Immutable view into string data.

```fruti
impl str {
    fn len(&self) -> usize
    fn is_empty(&self) -> bool
    fn chars(&self) -> Chars
    fn bytes(&self) -> Bytes
    
    fn contains(&self, pattern: str) -> bool
    fn starts_with(&self, prefix: str) -> bool
    fn ends_with(&self, suffix: str) -> bool
    
    fn split(&self, delimiter: char) -> Split
    fn lines(&self) -> Lines
    fn trim(&self) -> &str
    fn to_string(&self) -> String
}
```

---

## Filesystem

```fruti
mod fs {
    fn read(path: str) -> Result<Vec<u8>, Error>
    fn read_to_string(path: str) -> Result<String, Error>
    fn write(path: str, contents: &[u8]) -> Result<(), Error>
    fn remove_file(path: str) -> Result<(), Error>
    
    fn create_dir(path: str) -> Result<(), Error>
    fn create_dir_all(path: str) -> Result<(), Error>
    fn remove_dir(path: str) -> Result<(), Error>
    fn remove_dir_all(path: str) -> Result<(), Error>
    
    fn copy(from: str, to: str) -> Result<u64, Error>
    fn rename(from: str, to: str) -> Result<(), Error>
    
    fn metadata(path: str) -> Result<Metadata, Error>
    fn exists(path: str) -> bool
}

struct Metadata {
    fn is_file(&self) -> bool
    fn is_dir(&self) -> bool
    fn len(&self) -> u64
    fn modified(&self) -> Result<SystemTime, Error>
}
```

---

## Networking

### TCP

```fruti
mod net {
    struct TcpListener {
        // TCP listener
    }
    
    impl TcpListener {
        fn bind(addr: str) -> Result<TcpListener, Error>
        fn accept(&self) -> Result<(TcpStream, SocketAddr), Error>
        fn local_addr(&self) -> Result<SocketAddr, Error>
    }
    
    struct TcpStream {
        // TCP connection
    }
    
    impl TcpStream {
        fn connect(addr: str) -> Result<TcpStream, Error>
        fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error>
        fn write(&mut self, buf: &[u8]) -> Result<usize, Error>
        fn shutdown(&self) -> Result<(), Error>
    }
}
```

### UDP

```fruti
mod net {
    struct UdpSocket {
        // UDP socket
    }
    
    impl UdpSocket {
        fn bind(addr: str) -> Result<UdpSocket, Error>
        fn send_to(&self, buf: &[u8], addr: str) -> Result<usize, Error>
        fn recv_from(&self, buf: &mut [u8]) -> Result<(usize, SocketAddr), Error>
    }
}
```

---

## Concurrency

### Threads

```fruti
mod thread {
    fn spawn<F>(f: F) -> JoinHandle
        where F: fn() + Send
    
    fn sleep(duration: Duration)
    fn yield_now()
    
    struct JoinHandle {
        fn join(self) -> Result<(), Error>
    }
}
```

**Example:**
```fruti
let handle: JoinHandle = thread::spawn(|| {
    println("Hello from thread!");
});

handle.join()?;
```

### Channels

```fruti
mod sync {
    fn channel<T>() -> (Sender<T>, Receiver<T>)
    
    struct Sender<T> {
        fn send(&self, value: T) -> Result<(), Error>
    }
    
    struct Receiver<T> {
        fn recv(&self) -> Result<T, Error>
        fn try_recv(&self) -> Result<T, Error>
    }
}
```

### Synchronization

```fruti
mod sync {
    struct Mutex<T> {
        fn new(value: T) -> Mutex<T>
        fn lock(&self) -> MutexGuard<T>
    }
    
    struct RwLock<T> {
        fn new(value: T) -> RwLock<T>
        fn read(&self) -> ReadGuard<T>
        fn write(&self) -> WriteGuard<T>
    }
}
```

---

## Time

```fruti
mod time {
    struct Duration {
        fn from_secs(secs: u64) -> Duration
        fn from_millis(millis: u64) -> Duration
        fn from_micros(micros: u64) -> Duration
        fn as_secs(&self) -> u64
        fn as_millis(&self) -> u128
    }
    
    struct Instant {
        fn now() -> Instant
        fn elapsed(&self) -> Duration
    }
    
    struct SystemTime {
        fn now() -> SystemTime
        fn duration_since(&self, earlier: SystemTime) -> Result<Duration, Error>
    }
}
```

---

## Math

```fruti
mod math {
    // Constants
    const PI: f64 = 3.14159265358979323846;
    const E: f64 = 2.71828182845904523536;
    
    // Basic functions
    fn abs<T>(x: T) -> T
    fn min<T>(a: T, b: T) -> T
    fn max<T>(a: T, b: T) -> T
    fn clamp<T>(value: T, min: T, max: T) -> T
    
    // Power and roots
    fn sqrt(x: f64) -> f64
    fn cbrt(x: f64) -> f64
    fn pow(base: f64, exp: f64) -> f64
    
    // Trigonometry
    fn sin(x: f64) -> f64
    fn cos(x: f64) -> f64
    fn tan(x: f64) -> f64
    fn asin(x: f64) -> f64
    fn acos(x: f64) -> f64
    fn atan(x: f64) -> f64
    fn atan2(y: f64, x: f64) -> f64
    
    // Logarithms
    fn ln(x: f64) -> f64
    fn log2(x: f64) -> f64
    fn log10(x: f64) -> f64
    fn exp(x: f64) -> f64
    
    // Rounding
    fn floor(x: f64) -> f64
    fn ceil(x: f64) -> f64
    fn round(x: f64) -> f64
    fn trunc(x: f64) -> f64
}
```

---

## Summary

The Fruti standard library aims to provide:

1. **Core Utilities** - Essential types and operations
2. **Collections** - Efficient data structures
3. **I/O** - File and console operations
4. **Networking** - TCP/UDP sockets
5. **Concurrency** - Threads and synchronization
6. **System** - Time, filesystem, environment

**Implementation Roadmap:**
- Phase 1 (Complete): Basic primitives
- Phase 2 (Near Term): Core collections, basic I/O
- Phase 3 (Mid Term): Full networking and concurrency
- Phase 4 (Later): Advanced features and optimization

---

**Status:** Design specification - Implementation in progress
**Last Updated:** December 7, 2025

---

**Frutisoft © 2025 - Fresh code, crisp ideas**
