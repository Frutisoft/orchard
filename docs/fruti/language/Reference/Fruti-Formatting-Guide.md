# Fruti Formatting Guide

**Last Updated:** December 8, 2025  
**Status:** Canonical Reference - Official Style Guide

---

## Philosophy: Formatting Should Be Obvious, Not Contentious

Fruti's formatting rules are designed to **eliminate the formatting wars** that plague other programming languages. By analyzing decades of debates across Python, Rust, Go, JavaScript, C++, and Java, we've created a system that:

1. **Leverages language features** - Fruti's optional semicolons, dual logical operators, and clean syntax eliminate common pain points
2. **Provides clear defaults** - No ambiguity, no debates, no configuration paralysis
3. **Optimizes for readability** - Code is read 10x more than it's written
4. **Enables automation** - Future `fruti fmt` will enforce these rules automatically
5. **Proves superiority** - Every decision addresses specific pain points in existing languages

---

## The Pain Points We Solve

### Comparison: How Fruti Improves on Every Language

| Language | Major Formatting Pain Point | How Fruti Solves It |
|----------|---------------------------|-------------------|
| **Python** | Indentation syntax errors (tabs vs spaces) | Spaces-only, not syntax-significant (braces used) |
| **Python** | PEP 8 line length wars (79 vs 88 vs 120) | Pragmatic 100 char default, no dogma |
| **Python** | Import ordering confusion | Clear stdlib to external to local ordering |
| **Python** | Black vs autopep8 vs yapf wars | One formatter: `fruti fmt` (no alternatives) |
| **Rust** | Macro formatting inconsistency | No macros - functions only (consistent) |
| **Rust** | Lifetime annotation visual noise | Automatic inference - no annotations |
| **Rust** | rustfmt customization requires nightly | Clear defaults, minimal config needed |
| **Go** | gofmt rigid tabs (teams want spaces) | Spaces only, modern standard |
| **Go** | Zero customization allowed | Sensible defaults + minimal config |
| **JavaScript** | Semicolon wars (ASI ambiguity) | Optional semicolons with CLEAR rules |
| **JavaScript** | Prettier vs ESLint vs Standard fragmentation | One tool: `fruti fmt` |
| **JavaScript** | Trailing comma debates | Always use trailing commas (diffs) |
| **C++** | Hundreds of clang-format options | ~10 options max, rest is standardized |
| **C++** | Brace placement wars (K&R vs Allman) | Always same-line opening (K&R style) |
| **C++** | Pointer syntax confusion (`*p` vs `p*`) | No raw pointers in safe code |
| **Java** | Verbose ceremony | No forced `public class` per file |
| **Java** | Google vs Oracle style wars | One standard, no alternatives |

**Key Insight:** Most formatting wars stem from **language design flaws** or **lack of clear defaults**. Fruti's design eliminates the root causes.

---

## Core Formatting Rules

### 1. Indentation: 4 Spaces (Always)

**Rule:** Use exactly 4 spaces per indentation level. Never tabs.

**Rationale:**
- Python proved tabs cause syntax errors
- Go's forced tabs alienate many developers
- 4 spaces is optimal readability (studies show 2 too cramped, 8 too wide)
- Universal editor support

```fruti
// CORRECT
fn calculate_total(items: Vec<Item>) -> f64 {
    let mut sum = 0.0
    for item in items {
        sum += item.price
    }
    sum
}

// WRONG - tabs
fn calculate_total(items: Vec<Item>) -> f64 {
	let mut sum = 0.0  // Tab character
	sum
}

// WRONG - 2 spaces
fn calculate_total(items: Vec<Item>) -> f64 {
  let mut sum = 0.0
  sum
}
```

### 2. Line Length: 100 Characters (Pragmatic)

**Rule:** Aim for 100 characters maximum per line. Soft limit - prioritize readability.

**Rationale:**
- PEP 8's 79 chars is archaic (1980s terminal width)
- Modern screens support more (GitHub shows 120+)
- 100 chars balances readability with modern display widths
- Allows side-by-side diffs without horizontal scrolling

```fruti
// CORRECT - under 100 chars
fn send_notification(user: User, message: String, priority: Priority) -> Result<()> {
    ...
}

// ACCEPTABLE - break long function signatures
fn send_notification_with_retry_and_logging(
    user: User,
    message: String, 
    priority: Priority,
    retry_count: i32
) -> Result<()> {
    ...
}

// WRONG - unnecessarily long
fn send_notification(user: User, message: String, priority: Priority, retry_count: i32, timeout_ms: i32, use_ssl: bool) -> Result<()> {
```

### 3. Semicolons: Optional but Consistent

**Rule:** Semicolons are **optional** in Fruti. Choose one style per project.

**Styles:**
- **Style A (Recommended):** Omit semicolons where possible (Rust-like)
- **Style B:** Always use semicolons (JavaScript/C-like)

**Configure in `.fruti.toml`:**
```toml
[style]
semicolons = "omit"  # or "always"
```

**Rationale:**
- JavaScript's ASI causes bugs - Fruti's grammar eliminates ambiguity
- Rust proves semicolons can be optional without confusion
- Let teams choose based on background (flexibility)
- `fruti fmt` enforces consistency automatically

```fruti
// Style A (recommended): Omit semicolons
fn process_data(input: String) -> Result<Data> {
    let parsed = parse_input(input)?
    let validated = validate(parsed)?
    Ok(transform(validated))
}

// Style B: Always use semicolons
fn process_data(input: String) -> Result<Data> {
    let parsed = parse_input(input)?;
    let validated = validate(parsed)?;
    Ok(transform(validated));
}

// WRONG: Inconsistent mixing
fn process_data(input: String) -> Result<Data> {
    let parsed = parse_input(input)?;
    let validated = validate(parsed)?  // Mixed styles
    Ok(transform(validated))
}
```

### 4. Braces: Always Same-Line Opening

**Rule:** Opening braces `{` always on the same line. Closing braces `}` on their own line.

**Rationale:**
- C++ Allman style (new-line braces) wastes vertical space
- K&R style (same-line) proven optimal by Rust, Go, JavaScript
- Reduces file length by 20-30%
- More code visible per screen

```fruti
// CORRECT - same-line opening
fn calculate(x: i32) -> i32 {
    if x > 0 {
        x * 2
    } else {
        0
    }
}

struct Point {
    x: f64,
    y: f64,
}

// WRONG - Allman style wastes space
fn calculate(x: i32) -> i32
{
    if x > 0
    {
        x * 2
    }
    else
    {
        0
    }
}
```

### 5. Imports: Grouped and Ordered

**Rule:** Group imports in this order, with blank lines between groups:
1. Standard library (`std.*`)
2. External dependencies
3. Local modules (relative imports)

Within each group, sort alphabetically.

**Rationale:**
- Python's import chaos (no standard ordering)
- Clear separation shows external dependencies
- Reduces merge conflicts

```fruti
// CORRECT - grouped and ordered
import std::collections::{HashMap, Vec}
import std::fs
import std::http

import serde::json
import tokio::runtime

import app::config
import app::models::User

// WRONG - mixed grouping
import serde::json
import std::fs
import app::config
import std::http
```

**Multiple imports from same module:**
```fruti
// CORRECT - use braces for multiple
import std::collections::{HashMap, HashSet, Vec}

// ALSO ACCEPTABLE - separate lines if many imports
import std::collections::HashMap
import std::collections::HashSet
import std::collections::Vec
import std::collections::BTreeMap
import std::collections::BTreeSet

// WRONG - repetitive
import std::collections::HashMap
import std::collections::Vec
import std::collections::HashMap  // Duplicate
```

### 6. Struct and Enum Formatting

**Rule:** 
- Fields on separate lines
- **Always trailing comma** on multi-line
- No trailing comma on single-line

**Rationale:**
- JavaScript/Python debates: trailing commas improve diffs
- Single-line for small structs (concise), multi-line for large (readable)

```fruti
// CORRECT - multi-line with trailing comma
struct User {
    id: i32,
    name: String,
    email: String,
    age: i32,
}

// CORRECT - single-line, no trailing comma
struct Point { x: f64, y: f64 }

// CORRECT - enum variants
enum Result<T, E> {
    Ok(T),
    Error(E),
}

// WRONG - multi-line without trailing comma
struct User {
    id: i32,
    name: String,
    email: String  // Missing trailing comma
}

// WRONG - single-line with trailing comma
struct Point { x: f64, y: f64, }
```

**Struct Literals:**
```fruti
// CORRECT - multi-line with trailing comma
let user = User {
    id: 1,
    name: "Alice".to_string(),
    email: "alice@example.com".to_string(),
    age: 30,
}

// CORRECT - single-line for small literals
let point = Point { x: 1.0, y: 2.0 }

// CORRECT - field init shorthand
let name = "Bob".to_string()
let email = "bob@example.com".to_string()
let user = User { id: 2, name, email, age: 25 }
```

### 7. Function Signatures

**Rule:**
- Short signatures (< 100 chars): single line
- Long signatures: break after opening paren, one parameter per line, closing paren with return type on last line

**Rationale:**
- Java's verbose method signatures are unreadable
- Python's long signatures often poorly formatted
- Clear parameter separation improves readability

```fruti
// CORRECT - short signature
fn add(x: i32, y: i32) -> i32 {
    x + y
}

// CORRECT - long signature broken
fn process_transaction(
    user_id: i32,
    amount: f64,
    currency: Currency,
    description: String,
    metadata: HashMap<String, String>
) -> Result<Transaction, TransactionError> {
    ...
}

// CORRECT - method receivers
fn calculate_total(self) -> f64 {
    ...
}

fn update_user(mut self, name: String, email: String) {
    self.name = name
    self.email = email
}

// WRONG - awkward breaking
fn process_transaction(user_id: i32, amount: f64, 
    currency: Currency, description: String) -> Result<Transaction> {
    ...
}
```

### 8. Method Chaining

**Rule:** Break chains across lines when:
- Chain is > 80 characters
- Chain has > 2 methods

Indent continuation by 4 spaces. Operator (`.`) leads the line.

**Rationale:**
- Rust's iterator chains become unreadable on one line
- JavaScript Promise chains hard to debug without line breaks
- Leading `.` makes continuation obvious

```fruti
// CORRECT - short chain on one line
let result = items.filter(is_valid).collect()

// CORRECT - long chain broken
let result = items
    .filter(|x| x.is_active)
    .map(|x| x.value)
    .filter(|x| x > threshold)
    .sum()

// CORRECT - complex transformations
let users = database
    .query("SELECT * FROM users")
    .await?
    .filter(|u| u.age >= 18)
    .map(|u| UserDto::from(u))
    .collect::<Vec<_>>()

// WRONG - long chain on one line
let result = items.filter(|x| x.is_active).map(|x| x.value).filter(|x| x > threshold).sum()

// WRONG - trailing dot (hard to see continuation)
let result = items.
    filter(|x| x.is_active).
    map(|x| x.value)
```

### 9. Match Expressions

**Rule:**
- Align `=>` arrows
- Short arms on same line
- Long arms on new line, indented
- Always trailing comma on multi-line matches

**Rationale:**
- Rust's match is powerful but formatting varies
- Aligned arrows improve scannability
- Trailing commas prevent diff noise

```fruti
// CORRECT - short arms
match status {
    Status::Active => "active",
    Status::Pending => "pending",
    Status::Inactive => "inactive",
}

// CORRECT - long arms on new line
match result {
    Ok(value) => {
        log.info("Success: {value}")
        process(value)
    },
    Error(e) => {
        log.error("Failed: {e}")
        handle_error(e)
    },
}

// CORRECT - pattern guards
match user {
    User { age, .. } if age < 18 => "minor",
    User { age, .. } if age < 65 => "adult",
    _ => "senior",
}

// WRONG - misaligned arrows
match status {
    Status::Active => "active",
    Status::Pending   => "pending",
    Status::Inactive => "inactive",
}
```

### 10. Comments

**Rule:**
- Use `//` for single-line comments
- Use `//` on multiple consecutive lines for multi-line comments (no `/* */`)
- Doc comments use `///` for items, `//!` for modules
- Comments on their own line, not trailing (except rare cases)

**Rationale:**
- C++ `/* */` comments don't nest (bug source)
- Rust proves `//` for multi-line works great
- Trailing comments misalign easily, reduce readability

```fruti
// CORRECT - single-line comment
let x = 5  // Rare acceptable trailing comment

// CORRECT - multi-line comment using //
// This function processes user input by first validating
// the format, then parsing the data, and finally applying
// business logic transformations.
fn process_input(data: String) -> Result<Output> {
    ...
}

// CORRECT - doc comment
/// Calculates the total price including tax.
///
/// # Arguments
/// * `price` - The base price before tax
/// * `tax_rate` - The tax rate as a percentage (e.g., 0.08 for 8%)
///
/// # Returns
/// The total price including tax
fn calculate_with_tax(price: f64, tax_rate: f64) -> f64 {
    price * (1.0 + tax_rate)
}

// WRONG - /* */ style
/* This is a multi-line comment
   that uses C-style syntax */

// WRONG - excessive trailing comments
let x = 5  // X coordinate
let y = 10  // Y coordinate
let z = 15  // Z coordinate
```

### 11. Blank Lines

**Rule:**
- One blank line between function definitions
- Two blank lines between major sections (imports, types, impls)
- No blank lines at start/end of blocks

**Rationale:**
- Python's PEP 8 spacing rules improve readability
- Clear section separation aids navigation

```fruti
// CORRECT
import std::collections::Vec

struct User {
    name: String,
}

impl User {
    fn new(name: String) -> Self {
        User { name }
    }
    
    fn greet(self) {
        println("Hello, {self.name}")
    }
}

fn main() {
    let user = User::new("Alice")
    user.greet()
}

// WRONG - no spacing
import std::collections::Vec
struct User {
    name: String,
}
impl User {
    fn new(name: String) -> Self {
        User { name }
    }
    fn greet(self) {
        println("Hello, {self.name}")
    }
}
```

### 12. Logical Operators: Choose One Style

**Rule:** Fruti supports both keywords (`and`, `or`, `not`) and symbols (`&&`, `||`, `!`). Choose one style per project.

**Configure in `.fruti.toml`:**
```toml
[style]
logical_operators = "keywords"  # or "symbols"
```

**Rationale:**
- Python developers prefer keywords
- C/Rust/Go developers prefer symbols
- Both map to identical code - truly equivalent
- `fruti fmt` enforces consistency

```fruti
// Style: keywords (Python-like)
if user.is_active and user.has_permission and not user.is_banned {
    grant_access()
}

// Style: symbols (C/Rust-like)
if user.is_active && user.has_permission && !user.is_banned {
    grant_access()
}

// WRONG - mixing styles
if user.is_active && user.has_permission and !user.is_banned {
    grant_access()
}
```

### 13. Whitespace in Expressions

**Rule:**
- Space after commas: `foo(a, b, c)`
- Space around binary operators: `x + y`, `a == b`
- No space after unary operators: `!condition`, `-value`
- No space inside parentheses/brackets: `foo(x)`, `arr[i]`
- No space before colons in types: `x: i32`

```fruti
// CORRECT
let sum = a + b + c
let is_valid = x > 0 && y < 100
let negated = -value
let inverted = !flag
foo(a, b, c)
arr[index]
let x: i32 = 5

// WRONG - inconsistent spacing
let sum = a+b+c
let is_valid = x>0&&y<100
let negated = - value
foo( a , b , c )
let x : i32 = 5
```

---

## Configuration: Minimal by Design

Fruti formatting has **very few** configurable options (unlike clang-format's 100+). This prevents bikeshedding.

**`.fruti.toml` configuration:**

```toml
[style]
# Semicolon style: "omit" or "always"
semicolons = "omit"

# Logical operator style: "keywords" or "symbols"  
logical_operators = "keywords"

# Line length (soft limit)
max_line_length = 100

# Indentation (always 4 spaces, but configurable for edge cases)
indent_width = 4
```

**That's it.** Everything else is standardized.

---

## The `fruti fmt` Tool (Future)

When `fruti fmt` is implemented, it will:

1. **Enforce all rules automatically** - No debates
2. **Format on save** - Editor integration
3. **Check in CI** - `fruti fmt --check` fails if formatting is wrong
4. **One command** - No eslint + prettier + multiple configs
5. **Fast** - Formats entire large codebases in < 1 second

**Usage:**
```bash
# Format all .fruti files in current directory
fruti fmt

# Check formatting without modifying (for CI)
fruti fmt --check

# Format specific files
fruti fmt src/main.fruti src/lib.fruti
```

**Editor Integration:**
- VS Code: Fruti extension (auto-format on save)
- Vim/Neovim: LSP integration
- Emacs: LSP integration
- All editors: Works via LSP protocol

---

## Why This is Better: Summary Table

| Pain Point | Languages Affected | Fruti Solution |
|-----------|-------------------|----------------|
| **Tabs vs spaces syntax errors** | Python | Spaces only, not syntax-significant |
| **Line length dogma** | Python (79), Go (no limit) | Pragmatic 100 chars |
| **Import ordering confusion** | Python, JavaScript | Clear stdlib to external to local |
| **Formatter fragmentation** | Python (black/autopep8/yapf), JS (prettier/eslint/standard) | One tool: `fruti fmt` |
| **Macro formatting** | Rust | No macros - functions only |
| **Lifetime annotation noise** | Rust | Automatic inference |
| **Tabs forced** | Go | Spaces standard |
| **Zero customization** | Go | Minimal sensible config |
| **Semicolon wars** | JavaScript | Optional with clear rules + config |
| **Trailing comma debates** | JavaScript, Python | Always on multi-line (diffs) |
| **100+ config options** | C++ (clang-format) | ~5 config options total |
| **Brace placement wars** | C++, C# | K&R style (same-line) standardized |
| **Pointer syntax confusion** | C++ | No raw pointers in safe code |
| **Verbose ceremony** | Java | No forced file structure |
| **Style guide wars** | Java, C++, Python | One standard, period |

---

## Examples: Before and After

### Example 1: Web Server Handler (Before)

```fruti
// Poorly formatted (mixed styles)
async fn handle_request(req: http.Request,db: &Database)->http.Response{
match(req.method(),req.path()){
("GET","/users")=>{
let users=db.list_users();http.Response::json(&users)
},
("POST", "/users") => {
match req.json::<CreateUserRequest>().await{Ok(create_req)=>{let user=db.create_user(create_req.name,create_req.email);http.Response::json(&user).with_status(201)},Error(e)=>{http.Response::bad_request("Invalid JSON: {e}")
}}
},_=>http.Response::not_found()}}
```

### Example 1: Web Server Handler (After)

```fruti
// Well-formatted (Fruti style)
async fn handle_request(req: http.Request, db: &Database) -> http.Response {
    match (req.method(), req.path()) {
        ("GET", "/users") => {
            let users = db.list_users()
            http.Response::json(&users)
        },
        ("POST", "/users") => {
            match req.json::<CreateUserRequest>().await {
                Ok(create_req) => {
                    let user = db.create_user(create_req.name, create_req.email)
                    http.Response::json(&user).with_status(201)
                },
                Error(e) => {
                    http.Response::bad_request("Invalid JSON: {e}")
                },
            }
        },
        _ => http.Response::not_found(),
    }
}
```

### Example 2: Data Processing Pipeline

```fruti
// Well-formatted data pipeline
fn process_records(files: Vec<String>) -> Result<Summary> {
    let records = files
        .par_iter()
        .map(|file| {
            let content = fs.read_text(file)?
            parse_csv(content)
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .filter(|r| r.amount > 0.0)
        .collect::<Vec<_>>()
    
    let by_category = group_by_category(records)
    let summaries = by_category
        .into_par_iter()
        .map(|(category, recs)| {
            (category, analyze_category(&recs))
        })
        .collect()
    
    Ok(Summary { data: summaries })
}
```

---

## Migration Guide

### From Other Languages

#### Python Developers
```python
# Python style
def calculate_total(items):
    sum = 0
    for item in items:
        if item['active']:
            sum += item['price']
    return sum
```

```fruti
// Fruti equivalent
fn calculate_total(items: Vec<Item>) -> f64 {
    let mut sum = 0.0
    for item in items {
        if item.active {
            sum += item.price
        }
    }
    sum
}

// Fruti idiomatic (functional)
fn calculate_total(items: Vec<Item>) -> f64 {
    items
        .iter()
        .filter(|item| item.active)
        .map(|item| item.price)
        .sum()
}
```

#### JavaScript Developers
```javascript
// JavaScript style
async function fetchUsers(ids) {
  const users = await Promise.all(
    ids.map(id => fetchUser(id))
  );
  return users.filter(u => u != null);
}
```

```fruti
// Fruti equivalent
async fn fetch_users(ids: Vec<i32>) -> Vec<User> {
    let users = ids
        .iter()
        .map(|id| fetch_user(*id).await)
        .collect::<Vec<_>>()
    
    users
        .into_iter()
        .filter_map(|u| u)
        .collect()
}
```

---

## Philosophy: Why Formatting Matters

> "Programs must be written for people to read, and only incidentally for machines to execute."  
> — Harold Abelson

**Key Principles:**

1. **Consistency reduces cognitive load** - Same formatting everywhere means your brain can focus on logic, not style
2. **Automation prevents bikeshedding** - `fruti fmt` makes formatting a solved problem
3. **Good defaults > infinite configuration** - clang-format's 100+ options cause analysis paralysis
4. **Learn from history** - Python's tabs/spaces war, JavaScript's semicolon drama, C++'s brace wars were all preventable
5. **Language design enables great formatting** - Fruti's syntax eliminates many pain points at the source

**The Frutisoft Approach:**

We believe great formatting rules emerge from:
- **Analyzing decades of pain** - What caused fights in other languages?
- **Proving superiority** - Show how each rule solves real problems
- **Minimizing configuration** - Defaults work for 95% of teams
- **Enabling automation** - Computers format, humans focus on logic
- **Respecting ergonomics** - Balance consistency with readability

---

## Implementation Status

**Current Status:** Phase 1 - Specification Complete

**Roadmap:**
- **Phase 1 (Complete):** Formatting rules designed and documented
- **Phase 2 (Q1 2026):** `fruti fmt` tool implementation
- **Phase 3 (Q2 2026):** Editor integration (VS Code, Vim, Emacs)
- **Phase 4 (Q2 2026):** CI/CD integration templates

**Contributing:**

When `fruti fmt` development begins, we'll welcome:
- Performance optimizations
- Edge case testing
- Editor plugin development
- Documentation improvements

---

## Summary

Fruti's formatting rules are designed to:

1. **Eliminate formatting wars** - Clear, proven defaults
2. **Learn from every language's mistakes** - Tabs vs spaces, semicolons, braces, imports
3. **Optimize for readability** - Code is read 10x more than written
4. **Enable automation** - `fruti fmt` makes consistency effortless
5. **Respect developer time** - No bikeshedding, no config paralysis

**One Formatter. One Standard. Zero Debates.**

That's the Fruti way.

---

**Last Updated:** December 8, 2025  
**Status:** Canonical - All Fruti code must follow these rules

---

**Frutisoft © 2025 - Fresh code, crisp ideas**
