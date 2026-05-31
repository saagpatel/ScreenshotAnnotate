# ScreenshotAnnotate

[![TypeScript](https://img.shields.io/badge/TypeScript-3178c6?style=flat-square&logo=typescript&logoColor=white)](#) [![Rust](https://img.shields.io/badge/Rust-dea584?style=flat-square&logo=rust&logoColor=white)](#) [![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](#)

> Press one key, draw an arrow, export — the entire screenshot-to-annotated-PNG workflow in under 20 seconds

ScreenshotAnnotate collapses the manual annotation workflow — grab screenshot, open editor, add markup, export, find the file — into a single hotkey-triggered flow. Hit `⌘⇧5`, select a region, annotate with arrows, rectangles, text, or freehand, and save. Built with Tauri 2 and React 19 for a fast, native macOS experience with minimal resource usage.

## Features

- **Global hotkey** — `⌘⇧5` triggers native macOS screenshot capture with interactive region selection from anywhere on the desktop
- **5 annotation tools** — Arrow (with arrowhead), Rectangle, Text, Freehand, and Redact; switch with `A`/`R`/`T`/`F` (Redact has no shortcut)
- **50-step undo/redo** — full undo stack with `⌘Z` / `⇧⌘Z`; never lose work to a misplaced annotation
- **Color and thickness** — 4 preset colors plus custom hex input; 1–8px stroke width control
- **PNG export** — save annotated screenshots to disk
- **PII detection** — OCR via Tesseract.js scans screenshots for emails, phone numbers, IPs, and credit card numbers; auto-highlights regions for one-click redaction with blur, pixelate, or black-box styles
- **Upload wizard** — attach annotated screenshots directly to support tickets; service credentials stored securely in the macOS keychain
- **Annotation templates** — 3 built-in templates (Error Highlight, Click Here, Step by Step) apply pre-positioned arrows, rectangles, and text labels scaled to image dimensions
- **Thumbnail gallery** — searchable history of all saved screenshots (search by ticket ID or date) with storage usage tracking and LRU eviction at 500 MB

## Quick Start

### Prerequisites

- macOS 13+ (Ventura or later)
- Node.js 18+
- Rust 1.70+ (via [rustup](https://rustup.rs))
- Xcode Command Line Tools

### Installation

```bash
git clone https://github.com/saagpatel/ScreenshotAnnotate.git
cd ScreenshotAnnotate
npm install
```

### Usage

```bash
# Development mode
npm run tauri dev

# Run tests
npm test

# Production build
npm run tauri build
```

Grant screen recording permission when prompted on first launch — macOS requires this for the screenshot capture API.

## Tech Stack

| Layer | Technology |
|-------|------------|
| Desktop shell | Tauri 2 |
| Frontend | React 19, TypeScript 5.8, Vite 7 |
| Styling | CSS custom properties (App.css) |
| State | React hooks (useState) |
| Canvas | SVG overlay |
| OCR | Tesseract.js 7 |
| Clipboard | tauri-plugin-clipboard-manager |
| Tests | Vitest 3 |

## Architecture

The annotation canvas is an `<img>` element (the screenshot) with a transparent SVG overlay. Completed and in-progress annotations are SVG primitives — `<line>`/`<polygon>` for arrows, `<rect>` for rectangles and redaction boxes, `<path>` for freehand strokes, `<text>` for labels. The undo stack stores snapshots of the annotation array, not pixel data — memory overhead stays flat regardless of image size. The Rust backend handles macOS screenshot capture and file I/O; Tesseract.js OCR runs in a Web Worker for PII detection.

## License

MIT
