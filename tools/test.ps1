#!/usr/bin/env pwsh
# Test script for Frutisoft monorepo

param(
    [Parameter(Mandatory=$false)]
    [ValidateSet("all", "fruti", "aero")]
    [string]$Target = "all"
)

$ErrorActionPreference = "Stop"

Write-Host "`n=== Frutisoft Test Suite ===`n" -ForegroundColor Cyan

function Test-Fruti {
    Write-Host "Testing Fruti Compiler..." -ForegroundColor Yellow
    Push-Location packages/fruti-compiler
    
    try {
        cargo test
        Write-Host "[OK] Fruti tests passed" -ForegroundColor Green
    } finally {
        Pop-Location
    }
}

function Test-Aero {
    Write-Host "Testing Aero Kernel..." -ForegroundColor Yellow
    Push-Location packages/aero-kernel
    
    try {
        cargo test
        Write-Host "[OK] Aero tests passed" -ForegroundColor Green
    } finally {
        Pop-Location
    }
}

function Test-All {
    Write-Host "Running all tests..." -ForegroundColor Yellow
    cargo test --workspace
    Write-Host "[OK] All tests passed" -ForegroundColor Green
}

# Execute tests
switch ($Target) {
    "fruti" { Test-Fruti }
    "aero"  { Test-Aero }
    "all"   { Test-All }
}

Write-Host "`n=== Tests Complete ===`n" -ForegroundColor Cyan
