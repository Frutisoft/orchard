# Frutisoft Documentation

**Comprehensive documentation for Fruti programming language and Aero OS**

Last Updated: December 7, 2025

---

## Documentation Structure

This directory contains all documentation, resources, and historical materials for the Frutisoft project.

### [Fruti Language](./fruti/)
A general-purpose programming language designed to learn from existing languages' pain points

- **[Language Design Decisions](./fruti/Language-Design-Decisions.md)** - All syntax and semantic choices
- **[Quick Start Guide](./fruti/language/Guides/Quick-Start.md)** - Get started in 10 minutes
- **[Language Reference](./fruti/language/Reference/)** - Complete language specification
- **[Standard Library](./fruti/language/Reference/Standard-Library.md)** - Built-in modules and APIs
- **[Examples](./fruti/examples/)** - Code examples and patterns
- **[Guides](./fruti/language/Guides/)** - Tutorials and how-tos

### [Aero Operating System](./aero/)
Everything about Aero OS

- **[Technical Specification](./aero/Aero-OS-Technical-Spec.md)** - Kernel architecture and implementation
- **[Architecture](./aero/os/Architecture/)** - System design and internals
- **[Developer Guide](./aero/os/Developer%20Guide/)** - Building for Aero OS
- **[User Guide](./aero/os/User%20Guide/)** - Using Aero OS

### [Project Management](./project/)
Project status and milestones

- **[Phase 1 MVP Complete](./project/management/PHASE-1-MVP-COMPLETE.md)** - Fruti compiler Phase 1 status
- **[Project Status](./project/management/README.md)** - Current status and timeline

### [Resources](./resources/)
Production readiness guides and development resources

- **[Language Development Resources](./resources/lang/)** - Guides for building production-ready compilers
- **[OS Development Resources](./resources/os/)** - System readiness checklists and best practices

### [Assets](./assets/)
Visual assets, logos, and media

- Brand materials and design resources
- Project logos and icons

### [Archive](./archive/)
Historical documents and original design notes

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
- [Quick Start](./Language/Guides/Quick-Start.md) - [Basic Tutorial](./Language/Guides/Basic-Syntax.md)

**Understand ownership and borrowing**
- [Ownership Model](./Language/Reference/Ownership.md) - [Design Decision Doc](./Specifications/Language-Design-Decisions.md#2-ownership-and-memory)

**Write an async program**
- [Async Guide](./Language/Guides/Async-Programming.md) - [Async Examples](./Language/Examples/Async-Examples.md)

**Build a web server**
- [Web Server Example](./Language/Examples/Web-Server.md) - [HTTP Library Docs](./Language/Reference/Standard-Library.md#http)

**Understand Aero OS design**
- [OS Overview](./Operating%20System/Architecture/Overview.md) - [Technical Spec](./Specifications/Aero-OS-Technical-Spec.md)

**Write a device driver**
- [Driver Development Guide](./Operating%20System/Developer%20Guide/Driver-Development.md)

**Port software to Aero**
- [Compatibility Guide](./Operating%20System/Developer%20Guide/Compatibility.md)

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
