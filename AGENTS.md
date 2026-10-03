<!-- portfolio-context:start -->
# Portfolio Context

## What This Project Is

ScreenshotAnnotate is a Tauri desktop screenshot annotation tool for macOS. It turns global hotkey capture, region selection, canvas annotations, undo/redo, OCR, searchable history, and PNG export into a fast local workflow.

## Current State

The repo is active local desktop product work. Generated `.firecrawl` and performance-result folders are local artifacts and should not be included in source commits.

## Stack

| Layer | Technology |
|-------|------------|
| Desktop shell | Tauri 2 |
| Frontend | React 19, TypeScript 7.0, Vite 8 |
| Styling | CSS custom properties (App.css) |
| State | React hooks (useState) |
| Canvas | SVG overlay |
| OCR | Tesseract.js 7 |
| Clipboard | tauri-plugin-clipboard-manager |
| Tests | Vitest 5 |

## How To Run

Use npm with the committed `package-lock.json`; run `npm ci` from the repository root first. See [README prerequisites and verification](README.md#quick-start) for supported Node versions and native build requirements.

```bash
# Development mode
npm run tauri dev

# Run tests
npm test

# Production build
npm run tauri build
```

Grant screen recording permission when prompted on first launch — macOS requires this for the screenshot capture API.

## Known Risks

- macOS screen recording permission is required for capture; test first-launch permission behavior after capture changes.
- Canvas undo/redo stores snapshots of annotation arrays rather than pixel snapshots; preserve that memory profile.
- OCR runs in a Web Worker; avoid blocking annotation interactions.
- Generated `.firecrawl` and `.perf-results` folders should not be swept into source commits.

## Next Recommended Move

For behavior changes, verify the affected capture permission, annotation, undo/redo, OCR, history, or export paths before shipping. See [verification](README.md#verification) for focused checks and native smoke-test precautions.

<!-- portfolio-context:end -->
