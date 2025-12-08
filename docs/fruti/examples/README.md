# Fruti Code Examples

**Learn by example - practical code demonstrating Fruti's design**

---

## Status: Design Reference

**These examples show what Fruti code will look like, not what currently runs.** The compiler is in early MVP development. Use these examples to:

- **Understand the syntax design**
- **See planned language features**
- **Evaluate the language's philosophy**
- **Provide design feedback**

**These are aspirational examples** - they demonstrate the intended final design, not the current implementation state.

---

## Table of Contents

1. [Getting Started](#getting-started)
2. [Web Development](#web-development)
3. [Command-Line Tools](#command-line-tools)
4. [Concurrent Programming](#concurrent-programming)
5. [Systems Programming](#systems-programming)
6. [Data Processing](#data-processing)
7. [More Examples](#more-examples)

---

## Getting Started

### Hello World
```fruti
fn main() {
    println("Hello, World!")
}
```

### Variables and Types
```fruti
fn main() {
    let name = "Alice"           // String (inferred)
    let age = 30                 // Int (inferred)
    let height = 5.7             // Float (inferred)
    let is_active = true         // Bool (inferred)
    
    // Mutable variables
    let mut counter = 0
    counter += 1
    
    println("Name: {name}, Age: {age}, Height: {height}")
}
```

### Functions
```fruti
fn add(a: Int, b: Int) -> Int {
    a + b
}

fn greet(name: String) {
    println("Hello, {name}!")
}

fn main() {
    let sum = add(5, 3)
    greet("Bob")
}
```

---

## Web Development

### [Full Web Server](./web-server.fruti)
Complete REST API with routing, JSON, and in-memory database.

**Features:**
- HTTP server with routing
- JSON serialization/deserialization
- Thread-safe in-memory database
- CRUD operations
- Error handling

**Key concepts:**
```fruti
async fn handle_request(req: http.Request, db: &Database) -> http.Response {
    match (req.method(), req.path()) {
        ("GET", "/users") => {
            let users = db.list_users()
            http.Response::json(&users)
        }
        ("POST", "/users") => {
            let user_data = req.json::<CreateUserRequest>().await?
            let user = db.create_user(user_data.name, user_data.email)
            http.Response::json(&user).with_status(201)
        }
        _ => http.Response::not_found()
    }
}
```

### Simple HTTP Client
```fruti
import std.http

async fn main() {
    // GET request
    let response = http.get("https://api.github.com/users/octocat").await?
    let user: User = response.json().await?
    println("User: {user.login}")
    
    // POST request
    let data = CreatePost {
        title: "Hello",
        body: "World"
    }
    let response = http.post("https://api.example.com/posts")
        .json(&data)
        .send()
        .await?
    
    println("Status: {response.status()}")
}
```

---

## Command-Line Tools

### [Word Counter (wc)](./cli-tool.fruti)
Full-featured CLI tool with argument parsing and piping support.

**Features:**
- Command-line argument parsing
- File and stdin input
- Multiple counting modes (lines, words, chars)
- Output to file or stdout
- Help text

**Usage:**
```bash
wc file.txt
wc -l -w file.txt
cat file.txt | wc -c
wc -l file.txt -o counts.txt
```

### Simple CLI Tool
```fruti
import std.env
import std.fs

fn main() -> Result<()> {
    let args = env.args().collect()
    
    if args.len() < 2 {
        eprintln("Usage: mytool <file>")
        return Err("Missing argument")
    }
    
    let filename = &args[1]
    let content = fs.read_text(filename)?
    
    println("File has {content.lines().count()} lines")
    Ok(())
}
```

---

## Concurrent Programming

### [Concurrent File Downloader](./concurrent-downloader.fruti)
Download multiple files concurrently with progress tracking and retry logic.

**Features:**
- Async/await for I/O operations
- Concurrent downloads with rate limiting
- Automatic retry with exponential backoff
- Real-time progress tracking
- Shared state with Arc and Mutex

**Key concepts:**
```fruti
async fn download_with_retry(url: String, max_retries: Int) -> Result<Vec<u8>> {
    let mut attempts = 0
    
    loop {
        attempts += 1
        
        match http.get(&url).await {
            Ok(response) => return Ok(response.bytes().await?),
            Err(e) if attempts < max_retries => {
                eprintln("Retry {attempts}/{max_retries}: {e}")
                async.sleep(1s * attempts).await  // Exponential backoff
            }
            Err(e) => return Err(e)
        }
    }
}

async fn main() {
    let tasks = urls.iter()
        .map(|url| async.spawn(download_with_retry(url, 3)))
        .collect()
    
    let results = async.join_all(tasks).await
}
```

### Producer-Consumer with Channels
```fruti
import std.sync.channel
import std.thread

fn main() {
    let (tx, rx) = channel::unbounded()
    
    // Producer thread
    thread.spawn(move || {
        for i in 0..10 {
            tx.send(i).unwrap()
        }
    })
    
    // Consumer thread
    thread.spawn(move || {
        while let Some(value) = rx.recv() {
            println("Received: {value}")
        }
    })
}
```

---

## Systems Programming

### [Low-Level Memory Management](./systems-programming.fruti)
Custom allocators, zero-copy buffers, and FFI.

**Features:**
- Arena allocator implementation
- Zero-copy buffer handling
- Memory-mapped files
- Custom Arc implementation
- C FFI examples
- Unsafe code patterns

**Key concepts:**
```fruti
struct Arena {
    buffer: *mut u8
    capacity: usize
    offset: usize
}

impl Arena {
    fn alloc(mut self, size: usize, align: usize) -> Option<*mut u8> {
        let padding = (align - (self.offset % align)) % align
        let aligned_offset = self.offset + padding
        
        if aligned_offset + size > self.capacity {
            return None
        }
        
        let ptr = unsafe {
            self.buffer.offset(aligned_offset as isize)
        }
        
        self.offset = aligned_offset + size
        Some(ptr)
    }
}
```

### Device Driver Template
```fruti
import std.aero.driver

struct MyDriver {
    device: Device
}

impl Driver for MyDriver {
    fn probe(device: Device) -> Result<Self> {
        // Initialize hardware
        Ok(MyDriver { device })
    }
    
    fn read(self, offset: u64, buf: &mut [u8]) -> Result<usize> {
        // Read from device
        Ok(buf.len())
    }
    
    fn write(self, offset: u64, buf: &[u8]) -> Result<usize> {
        // Write to device
        Ok(buf.len())
    }
}
```

---

## Data Processing

### [CSV Processing Pipeline](./data-processing.fruti)
Parallel data processing with functional programming patterns.

**Features:**
- CSV parsing
- Functional transformations (map, filter, reduce)
- Parallel processing
- Statistical analysis
- Group by and aggregation
- Moving averages and percentiles

**Key concepts:**
```fruti
fn process_pipeline(files: Vec<String>) -> Result<Summary> {
    let records = files.par_iter()  // Parallel iterator
        .flat_map(|file| parse_csv(file))
        .filter(|r| r.amount > 0.0)
        .map(|r| normalize(r))
        .collect()
    
    let summary = records.iter()
        .fold(Summary::new(), |acc, r| acc.update(r))
    
    Ok(summary)
}

// Group by and aggregate
fn group_by_category(records: Vec<Record>) -> HashMap<String, Vec<Record>> {
    records.into_iter()
        .fold(HashMap::new(), |mut acc, record| {
            acc.entry(record.category.clone())
                .or_insert(Vec::new())
                .push(record)
            acc
        })
}
```

### JSON Processing
```fruti
import std.json
import std.fs

struct Config {
    host: String
    port: Int
    features: Vec<String>
}

fn main() -> Result<()> {
    // Parse JSON
    let json_str = fs.read_text("config.json")?
    let config: Config = json.parse(&json_str)?
    
    println("Server: {config.host}:{config.port}")
    
    // Serialize JSON
    let new_config = Config {
        host: "localhost",
        port: 3000,
        features: vec!["auth", "cache"]
    }
    
    let json_str = json.stringify_pretty(&new_config)?
    fs.write_text("config.json", &json_str)?
    
    Ok(())
}
```

---

## More Examples

### Error Handling
```fruti
fn divide(a: Int, b: Int) -> Result<Int> {
    if b == 0 {
        Err("Division by zero")
    } else {
        Ok(a / b)
    }
}

fn main() -> Result<()> {
    let result = divide(10, 2)?  // ? propagates errors
    println("Result: {result}")
    
    // Match on Result
    match divide(10, 0) {
        Ok(value) => println("Result: {value}"),
        Err(e) => eprintln("Error: {e}")
    }
    
    // Unwrap with default
    let result = divide(10, 0).unwrap_or(0)
    
    Ok(())
}
```

### Pattern Matching
```fruti
enum Message {
    Text(String)
    Number(Int)
    Quit
}

fn handle_message(msg: Message) {
    match msg {
        Message.Text(s) if s.len() > 10 => {
            println("Long message: {s}")
        }
        Message.Text(s) => {
            println("Short message: {s}")
        }
        Message.Number(n) if n > 100 => {
            println("Large number: {n}")
        }
        Message.Number(n) => {
            println("Number: {n}")
        }
        Message.Quit => {
            println("Goodbye!")
        }
    }
}
```

### Generics and Traits
```fruti
trait Printable {
    fn print(self)
}

impl Printable for Int {
    fn print(self) {
        println("Number: {self}")
    }
}

impl Printable for String {
    fn print(self) {
        println("Text: {self}")
    }
}

fn print_twice<T: Printable>(value: T) {
    value.print()
    value.print()
}

fn main() {
    print_twice(42)
    print_twice("Hello")
}
```

### Closures and Higher-Order Functions
```fruti
fn main() {
    let numbers = vec![1, 2, 3, 4, 5]
    
    // Map
    let doubled = numbers.map(|x| x * 2)
    
    // Filter
    let evens = numbers.filter(|x| x % 2 == 0)
    
    // Reduce
    let sum = numbers.reduce(0, |acc, x| acc + x)
    
    // Chaining
    let result = numbers.iter()
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .sum()
    
    // Custom closure
    let multiplier = 10
    let scale = |x| x * multiplier
    let scaled = numbers.map(scale)
}
```

### Testing
```fruti
fn add(a: Int, b: Int) -> Int {
    a + b
}

#[test]
fn test_add() {
    assert_eq(add(2, 3), 5)
    assert_eq(add(-1, 1), 0)
    assert_eq(add(0, 0), 0)
}

#[test]
fn test_divide() {
    assert_eq(divide(10, 2), Ok(5))
    assert(divide(10, 0).is_err())
}

// Run with: fruti test
```

---

## Running Examples

### Compile and Run
```bash
# Compile single file
fruti build example.fruti

# Run directly
fruti run example.fruti

# With optimizations
fruti build --release example.fruti

# Run tests
fruti test example.fruti
```

### Create Project
```bash
# Create new project
fruti new myproject

# Project structure:
# myproject/
#   src/
#     main.fruti
#   fruti.toml

# Build and run
cd myproject
fruti run
```

---

## Additional Resources

- [Language Reference](../Language/Reference/)
- [Standard Library](../Language/Reference/Standard-Library.md)
- [Quick Start Guide](../Language/Guides/Quick-Start.md)
- [Language Design Decisions](../Language-Design-Decisions.md)

---

**Frutisoft © 2025 - Fresh code, crisp ideas**
