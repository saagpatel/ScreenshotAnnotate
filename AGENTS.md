<!-- portfolio-context:start -->
# Portfolio Context

## What This Project Is

ScreenshotAnnotate is a Tauri desktop screenshot annotation tool for macOS. It turns global hotkey capture, region selection, canvas annotations, undo/redo, OCR, searchable history, and PNG export into a fast local workflow.

## Current State

The repo is active local desktop product work. Existing untracked `.firecrawl` and performance-result folders are local artifacts, so this recovery pass should only add the context file.

## Stack

| Layer | Technology |
|-------|------------|
| Desktop shell | Tauri 2 |
| Frontend | React 19, TypeScript 5.8, Vite 7 |
| Styling | Tailwind CSS 4 |
| State | Zustand 5 |
| Canvas | HTML5 Canvas 2D API |
| OCR | Tesseract.js 7 |
| Clipboard | tauri-plugin-clipboard-manager |
| Tests | Vitest 3, Testing Library |

## How To Run

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
- Canvas undo/redo stores annotation commands rather than pixel snapshots; preserve that memory profile.
- OCR runs in a Web Worker; avoid blocking annotation interactions.
- Generated `.firecrawl` and `.perf-results` folders should not be swept into source commits.

## Next Recommended Move

Add only the context file for this recovery pass, then verify capture permission, canvas annotation, undo/redo, OCR, history, and export paths before shipping changes.

<!-- portfolio-context:end -->
