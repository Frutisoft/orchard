# Build Tools

This directory contains build and utility scripts for the Frutisoft monorepo.

## Available Scripts

### build.ps1 (PowerShell)
Build individual packages or entire workspace.

**Usage:**
```powershell
# Build everything (debug mode)
.\tools\build.ps1

# Build specific package
.\tools\build.ps1 -Target fruti
.\tools\build.ps1 -Target aero

# Release build
.\tools\build.ps1 -Release
.\tools\build.ps1 -Target fruti -Release
```

### test.ps1 (PowerShell)
Run tests for packages.

**Usage:**
```powershell
# Run all tests
.\tools\test.ps1

# Test specific package
.\tools\test.ps1 -Target fruti
.\tools\test.ps1 -Target aero
```

## CI/CD Scripts (Planned)

- `ci/lint.ps1` - Run linters and formatters
- `ci/check.ps1` - Run all checks before commit
- `ci/release.ps1` - Create release builds

## Direct Cargo Commands

You can also use Cargo directly:

```bash
# Build entire workspace
cargo build --workspace

# Build specific package
cargo build -p fruti-compiler
cargo build -p aero-kernel

# Run tests
cargo test --workspace

# Check without building
cargo check --workspace

# Clean build artifacts
cargo clean
```

## Workspace Benefits

- Shared dependencies (defined in root Cargo.toml)
- Single target directory (faster builds)
- Build multiple packages together
- Easier dependency management between packages
