# Pre-Push Verification Script
# Run this before pushing to GitHub for the first time

Write-Host "=== Frutisoft Pre-Push Verification ===" -ForegroundColor Cyan
Write-Host ""

$errors = @()
$warnings = @()

# Check 1: Working directory is clean
Write-Host "Checking working directory..." -NoNewline
$status = git status --porcelain
if ($status) {
    $errors += "Working directory is not clean. Commit or stash changes."
    Write-Host " FAIL" -ForegroundColor Red
} else {
    Write-Host " PASS" -ForegroundColor Green
}

# Check 2: No emoji/symbol mentions in commit history
Write-Host "Checking commit history for emoji mentions..." -NoNewline
$emojiCommits = git log --all --oneline | Select-String -Pattern "emoji|symbol" -CaseSensitive:$false
if ($emojiCommits) {
    $errors += "Found commits mentioning emojis/symbols: $emojiCommits"
    Write-Host " FAIL" -ForegroundColor Red
} else {
    Write-Host " PASS" -ForegroundColor Green
}

# Check 3: Required files exist
Write-Host "Checking required files..." -NoNewline
$requiredFiles = @(
    "README.md",
    "LICENSE",
    "CONTRIBUTING.md",
    "CODE_OF_CONDUCT.md",
    ".gitignore",
    ".github/workflows/ci.yml",
    ".github/SECURITY.md",
    ".github/ISSUE_TEMPLATE/bug_report.md",
    ".github/PULL_REQUEST_TEMPLATE.md"
)
$missing = @()
foreach ($file in $requiredFiles) {
    if (-not (Test-Path $file)) {
        $missing += $file
    }
}
if ($missing) {
    $errors += "Missing required files: $($missing -join ', ')"
    Write-Host " FAIL" -ForegroundColor Red
} else {
    Write-Host " PASS" -ForegroundColor Green
}

# Check 4: Compiler tests pass
Write-Host "Running compiler tests..." -NoNewline
Push-Location packages/fruti-compiler
$testResult = cargo test --quiet 2>&1
$testExitCode = $LASTEXITCODE
Pop-Location
if ($testExitCode -ne 0) {
    $errors += "Compiler tests failed"
    Write-Host " FAIL" -ForegroundColor Red
} else {
    Write-Host " PASS" -ForegroundColor Green
}

# Check 5: Formatting check
Write-Host "Checking code formatting..." -NoNewline
Push-Location packages/fruti-compiler
$fmtResult = cargo fmt --check 2>&1
$fmtExitCode = $LASTEXITCODE
Pop-Location
if ($fmtExitCode -ne 0) {
    $errors += "Code formatting issues found. Run 'cargo fmt'"
    Write-Host " FAIL" -ForegroundColor Red
} else {
    Write-Host " PASS" -ForegroundColor Green
}

# Check 6: Clippy warnings
Write-Host "Checking clippy warnings..." -NoNewline
Push-Location packages/fruti-compiler
$clippyResult = cargo clippy --quiet -- -D warnings 2>&1
$clippyExitCode = $LASTEXITCODE
Pop-Location
if ($clippyExitCode -ne 0) {
    $errors += "Clippy warnings found"
    Write-Host " FAIL" -ForegroundColor Red
} else {
    Write-Host " PASS" -ForegroundColor Green
}

# Check 7: Remote configured
Write-Host "Checking git remote..." -NoNewline
$remote = git remote get-url origin 2>&1
if ($LASTEXITCODE -ne 0) {
    $errors += "Git remote not configured"
    Write-Host " FAIL" -ForegroundColor Red
} else {
    Write-Host " PASS" -ForegroundColor Green
    Write-Host "  Remote: $remote" -ForegroundColor Gray
}

# Check 8: Branch is main
Write-Host "Checking current branch..." -NoNewline
$branch = git branch --show-current
if ($branch -ne "main") {
    $warnings += "Current branch is '$branch', not 'main'"
    Write-Host " WARN" -ForegroundColor Yellow
} else {
    Write-Host " PASS" -ForegroundColor Green
}

# Summary
Write-Host ""
Write-Host "=== Verification Summary ===" -ForegroundColor Cyan

if ($errors.Count -eq 0 -and $warnings.Count -eq 0) {
    Write-Host "All checks passed! Repository is ready for first push." -ForegroundColor Green
    Write-Host ""
    Write-Host "To push to GitHub, run:" -ForegroundColor Cyan
    Write-Host "  git push -u origin main" -ForegroundColor White
    Write-Host ""
    Write-Host "After pushing, configure repository settings as described in:" -ForegroundColor Cyan
    Write-Host "  .github/REPOSITORY_SETUP.md" -ForegroundColor White
    exit 0
} else {
    if ($errors.Count -gt 0) {
        Write-Host ""
        Write-Host "ERRORS:" -ForegroundColor Red
        foreach ($error in $errors) {
            Write-Host "  - $error" -ForegroundColor Red
        }
    }
    if ($warnings.Count -gt 0) {
        Write-Host ""
        Write-Host "WARNINGS:" -ForegroundColor Yellow
        foreach ($warning in $warnings) {
            Write-Host "  - $warning" -ForegroundColor Yellow
        }
    }
    Write-Host ""
    Write-Host "Fix the errors above before pushing." -ForegroundColor Red
    exit 1
}
