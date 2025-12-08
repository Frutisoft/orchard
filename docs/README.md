# Frutisoft Documentation

**Comprehensive documentation for Fruti programming language and Aero OS**

Last Updated: December 8, 2025

---

## Quick Links

- **[Fruti Language Design](./fruti/Language-Design-Decisions.md)** - All syntax and semantic choices
- **[Quick Start Guide](./fruti/language/Guides/Quick-Start.md)** - Get started in 10 minutes
- **[Aero OS Specification](./aero/Aero-OS-Technical-Spec.md)** - Kernel architecture
- **[Phase 1 Complete](./PHASE-1-COMPLETE.md)** - Fruti compiler Phase 1 status

---

## Documentation Structure

### [Fruti Language](./fruti/)
General-purpose programming language

- **Language Reference** - Complete specification
- **Standard Library** - Built-in modules and APIs
- **Examples** - Code patterns and use cases
- **Guides** - Tutorials and deep dives

### [Aero Operating System](./aero/)
Microkernel OS design

- **Technical Specification** - Architecture and implementation
- **Developer Guide** - Building for Aero OS
- **User Guide** - Using Aero OS

### [Resources](./resources/)
Development references

- Language evaluation criteria and production readiness guides
- OS development checklists and best practices

### [Archive](./archive/)
Original design documents from early 2025

- Original Fruti language design documents
- Original Aero OS architecture notes
- Project evolution and restructure documentation

---

## Quick Links

### For Developers New to Fruti
1. [Quick Start Guide](./fruti/language/Guides/Quick-Start.md)
2. [Language Design Decisions](./fruti/Language-Design-Decisions.md)
3. [Common Examples](./fruti/examples/)

### For Experienced Developers
1. [Language Design Decisions](./fruti/Language-Design-Decisions.md)
2. [Ownership and Memory Model](./fruti/language/Guides/Ownership-Deep-Dive.md)
3. [Standard Library Reference](./fruti/language/Reference/Standard-Library.md)

### For Systems Programmers
1. [Aero OS Technical Spec](./aero/Aero-OS-Technical-Spec.md)
2. [Architecture Overview](./aero/os/Architecture/)
3. [Developer Guide](./aero/os/Developer%20Guide/)

### For Contributors
1. [Contributing Guide](../CONTRIBUTING.md)
2. [Compiler Documentation](../packages/fruti-compiler/README.md)
3. [Kernel Documentation](../packages/aero-kernel/README.md)

---

## Document Types

### Specifications (Canonical)
Definitive technical documents that define how things work. These are the source of truth.

**Location:** `Specifications/`

### Reference Documentation
Complete API documentation, language features, and system interfaces.

**Location:** `Language/Reference/`, `Operating System/Developer Guide/`

### Guides and Tutorials
Step-by-step instructions and learning materials.

**Location:** `Language/Guides/`, `Operating System/User Guide/`

### Examples
Working code samples demonstrating features and patterns.

**Location:** `Language/Examples/`

---

## Find What You Need

### "I want to..."

**Learn Fruti from scratch**
- [Quick Start](./fruti/language/Guides/Quick-Start.md) - [Ownership Deep Dive](./fruti/language/Guides/Ownership-Deep-Dive.md)

**Understand ownership and borrowing**
- [Ownership Deep Dive](./fruti/language/Guides/Ownership-Deep-Dive.md) - [Design Decision Doc](./fruti/Language-Design-Decisions.md#2-ownership-and-memory)

**Write an async program**
- [Async Examples](./fruti/examples/concurrent-downloader.fruti) - [Standard Library](./fruti/language/Reference/Standard-Library.md)

**Build a web server**
- [Web Server Example](./fruti/examples/web-server.fruti) - [HTTP Library Docs](./fruti/language/Reference/Standard-Library.md#http)

**Understand Aero OS design**
- [OS Overview](./aero/os/Architecture/) - [Technical Spec](./aero/Aero-OS-Technical-Spec.md)

**Write a device driver**
- [Driver Development Guide](./aero/os/Developer%20Guide/) (coming soon)

**Port software to Aero**
- [Compatibility Guide](./aero/os/Developer%20Guide/) (coming soon)

---

## Documentation Standards

All documentation follows these principles:

1. **Clear and Concise** - No unnecessary jargon
2. **Example-Driven** - Show, don't just tell
3. **Complete** - No "TBD" or missing sections
4. **Accurate** - Reflects actual implementation
5. **Up-to-Date** - Reviewed regularly

---

## Documentation Roadmap

### Phase 1: Foundation (Completed)
- [x] Language design decisions
- [x] OS technical specification
- [x] Quick start guide
- [x] Basic syntax reference
- [x] Compiler Phase 1 MVP

### Phase 2: Complete Reference (In Progress)
- [ ] Full language reference
- [ ] Standard library documentation
- [ ] Comprehensive examples
- [ ] System call reference

### Phase 3: Advanced Topics
- [ ] Advanced language features
- [ ] Performance optimization
- [ ] Debugging and profiling
- [ ] Cross-compilation

---

## Contributing to Documentation

Contributions welcome once project reaches MVP status!

### How to Help (Future)
- Fix typos and grammar
- Add missing examples
- Clarify confusing explanations
- Write new tutorials

See [CONTRIBUTING.md](../CONTRIBUTING.md) for details.

---

## License

Documentation licensed under CC BY 4.0
Code examples licensed under MIT

---

**Frutisoft © 2025 - Fresh code, crisp ideas**
