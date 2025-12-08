# Frutisoft Website

This directory contains a minimal placeholder website for Frutisoft, hosted via GitHub Pages.

## Current Status

**Placeholder Site**: The website currently displays a simple landing page directing visitors to the GitHub repository and documentation. Full website development is deferred while focusing on:
- Fruti compiler implementation
- Aero kernel design and documentation
- GitHub repository organization

## Structure

```
website/
├── index.html # Minimal landing page
└── README.md # This file
```

## GitHub Pages Setup

The site is automatically deployed from this directory:
- Go to Repository Settings - Pages
- Source: Deploy from a branch
- Branch: `main`
- Folder: `/website`
- Save

The site will be available at: `https://frutisoft.github.io/`

## Future Development

Once the Fruti compiler reaches MVP status, a proper website will be developed featuring:
- Interactive language playground
- Complete documentation
- Tutorial system
- Download page
- Blog for development updates

## Local Testing

To test the site locally:
```bash
cd website
python -m http.server 8000
```

Visit `http://localhost:8000`

## Deployment

Changes to this directory are automatically deployed when pushed to the main branch:
```bash
git add website/
git commit -m "Update website"
git push
```

GitHub Pages will automatically rebuild and deploy within a few minutes.

---

**Note:** This is intentionally minimal during early development. Focus remains on core compiler and OS development.

---

**Frutisoft © 2025 - Fresh code, crisp ideas**
