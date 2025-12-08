#!/usr/bin/env pwsh
# Build script for Frutisoft monorepo

param(
    [Parameter(Mandatory=$false)]
    [ValidateSet("all", "fruti", "aero")]
    [string]$Target = "all",
    
    [Parameter(Mandatory=$false)]
    [switch]$Release
)

$ErrorActionPreference = "Stop"

Write-Host "`n=== Frutisoft Build Script ===`n" -ForegroundColor Cyan

function Build-Fruti {
    Write-Host "Building Fruti Compiler..." -ForegroundColor Yellow
    Push-Location packages/fruti-compiler
    
    try {
        if ($Release) {
            cargo build --release
            Write-Host "[OK] Fruti compiler built (release mode)" -ForegroundColor Green
        } else {
            cargo build
            Write-Host "[OK] Fruti compiler built (debug mode)" -ForegroundColor Green
        }
    } finally {
        Pop-Location
    }
}

function Build-Aero {
    Write-Host "Building Aero Kernel..." -ForegroundColor Yellow
    Push-Location packages/aero-kernel
    
    try {
        if ($Release) {
            cargo build --release
            Write-Host "[OK] Aero kernel built (release mode)" -ForegroundColor Green
        } else {
            cargo build
            Write-Host "[OK] Aero kernel built (debug mode)" -ForegroundColor Green
        }
    } finally {
        Pop-Location
    }
}

function Build-All {
    Write-Host "Building entire workspace..." -ForegroundColor Yellow
    
    if ($Release) {
        cargo build --workspace --release
        Write-Host "[OK] All packages built (release mode)" -ForegroundColor Green
    } else {
        cargo build --workspace
        Write-Host "[OK] All packages built (debug mode)" -ForegroundColor Green
    }
}

# Execute builds
switch ($Target) {
    "fruti" { Build-Fruti }
    "aero"  { Build-Aero }
    "all"   { Build-All }
}

Write-Host "`n=== Build Complete ===`n" -ForegroundColor Cyan
