# GitHub Repository Setup Guide

This document describes the recommended GitHub repository settings for the Frutisoft monorepo.

## Repository Settings

### General Settings

**Repository Name:** `frutisoft`
**Description:** "Fruti programming language and Aero OS - A passion project exploring modern language and OS design"
**Website:** (Leave blank or add documentation URL once Pages is set up)
**Topics:** `programming-language`, `compiler`, `operating-system`, `rust`, `llvm`, `microkernel`, `systems-programming`, `language-design`

**Features:**
- Issues
- Discussions (Enable for community Q&A)
- Projects (Optional - for roadmap tracking)
- Wiki (Optional - for additional documentation)

**Pull Requests:**
- Allow merge commits
- Allow squash merging
- Allow rebase merging
- Always suggest updating pull request branches
- Automatically delete head branches

### Branch Protection Rules

**Branch:** `main`

**Protect matching branches:**
- Require a pull request before merging
  - Required approvals: 1 (for when you have collaborators)
  - Dismiss stale pull request approvals when new commits are pushed
  - Require review from Code Owners: No (not needed for solo project)
- Require status checks to pass before merging
  - Require branches to be up to date before merging
  - Required status checks:
    - `Test Fruti Compiler (ubuntu-latest, stable)`
    - `Test Fruti Compiler (windows-latest, stable)`
    - `Test Fruti Compiler (macos-latest, stable)`
    - `Lint`
- Require conversation resolution before merging
- Require signed commits: Optional (enable if you use GPG)
- Require linear history
- Require deployments to succeed: No (not needed yet)
- Lock branch (prevents deletion)
- Do not allow bypassing: No (you can bypass as maintainer)

### GitHub Actions

**Actions permissions:**
- Allow all actions and reusable workflows

**Workflow permissions:**
- Read and write permissions
- Allow GitHub Actions to create and approve pull requests

### GitHub Pages

**Source:** Deploy from a branch
**Branch:** `gh-pages` / `(root)` (will be auto-created by deploy workflow)
**Custom domain:** (Optional - configure later)

Alternatively, use the website deployment workflow which deploys from the `website/` folder.

### Security

**Dependabot:**
- Enabled (via `.github/dependabot.yml`)
- Dependabot alerts
- Dependabot security updates

**Code scanning:**
- Enable CodeQL (optional - add `.github/workflows/codeql.yml`)

**Secret scanning:**
- Enabled (automatic for public repos)

### Discussions (Recommended)

Enable GitHub Discussions for:
- **Q&A:** Community questions
- **Ideas:** Feature requests and design discussions
- **Show and Tell:** Share projects built with Fruti
- **Announcements:** Release notes and updates

### Topics/Tags

Add these topics to help with discoverability:
```
programming-language
compiler
operating-system
rust
llvm
microkernel
systems-programming
language-design
bootstrapped-compiler
capability-based
fruti
aero-os
```

## Initial Push Checklist

Before pushing to GitHub:

- [x] All sensitive data removed (no API keys, passwords, etc.)
- [x] `.gitignore` configured for Rust/Cargo
- [x] License file present (MIT)
- [x] README.md with badges and documentation links
- [x] CONTRIBUTING.md with contribution guidelines
- [x] CODE_OF_CONDUCT.md
- [x] SECURITY.md with security policy
- [x] `.github/workflows/` CI/CD pipelines configured
- [x] `.github/ISSUE_TEMPLATE/` issue templates
- [x] `.github/PULL_REQUEST_TEMPLATE.md`
- [x] `.github/dependabot.yml` for dependency updates
- [x] Working directory is clean (no uncommitted changes)
- [x] All tests pass locally
- [x] Documentation is up to date

## Post-Push Configuration

After the first push:

1. **Enable GitHub Pages:**
   - Go to Settings → Pages
   - Select source (workflow or branch)
   - Save

2. **Enable Discussions:**
   - Go to Settings → General → Features
   - Check "Discussions"
   - Configure categories

3. **Set Branch Protection:**
   - Go to Settings → Branches
   - Add rule for `main` branch
   - Configure as described above

4. **Add Repository Topics:**
   - Click the gear icon next to "About"
   - Add the topics listed above

5. **Configure Secrets (if needed):**
   - Go to Settings → Secrets and variables → Actions
   - Add any necessary secrets (not needed for basic setup)

6. **Monitor First CI Run:**
   - Go to Actions tab
   - Verify all workflows pass
   - Fix any issues

## Ongoing Maintenance

- **Weekly:** Review Dependabot PRs
- **Monthly:** Review open issues and discussions
- **Per Release:** Update README badges and version numbers
- **Quarterly:** Review and update documentation

---

**Frutisoft © 2025 - Fresh code, crisp ideas**
