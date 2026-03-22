# Screenshot Annotate

A macOS desktop screenshot annotation tool built with Tauri 2 + React 19 that collapses the 3-minute manual annotation workflow into ~20 seconds.

## Features

✅ **Button-first Capture** - The `Capture Screenshot` button always launches native macOS screenshot capture  
✅ **Best-effort Global Shortcut** - `⌘⇧A` is attempted when macOS allows registration  
✅ **Interactive Annotation Canvas** - Draw arrows, rectangles, text, freehand, and blackout redactions  
✅ **Undo/Redo Stack** - Full undo/redo support with keyboard shortcuts  
✅ **Export + History** - Save annotated screenshots and browse local history  
✅ **OCR + Upload Surface** - OCR detection, credential storage, and upload flow are wired into the app

## Installation

### Prerequisites

- macOS 13+ (Ventura or later)
- Node.js 18+
- Rust 1.77.2+ (install via [rustup](https://rustup.rs/))

### Setup

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

### First Run Permissions

On first launch, macOS will prompt you to grant **Screen Recording** permission:

1. System Settings → Privacy & Security → Screen Recording
2. Enable permission for "Screenshot Annotate"
3. Restart the app

## Usage

1. **Capture**: Click "Capture Screenshot" (`⌘⇧A` is best-effort on macOS)
2. **Select region**: Drag to select the area you want to capture (native macOS crosshair)
3. **Annotate**:
   - Use toolbar buttons or keyboard shortcuts to select tools
   - Draw annotations on your screenshot
   - Change colors and thickness as needed
4. **Save**: Click "Save" button or press `⌘S`

## Keyboard Shortcuts

| Action                   | Shortcut               |
| ------------------------ | ---------------------- |
| Capture screenshot       | `⌘⇧A` (when available) |
| Switch to Arrow tool     | `A`                    |
| Switch to Rectangle tool | `R`                    |
| Switch to Text tool      | `T`                    |
| Switch to Freehand tool  | `F`                    |
| Undo                     | `⌘Z`                   |
| Redo                     | `⌘⇧Z`                  |
| Save                     | `⌘S`                   |
| Cancel                   | `Esc`                  |

## Development Roadmap

### ✅ Phase 0: Screenshot Capture + Annotation (COMPLETE)

- macOS screenshot capture via `screencapture` CLI
- Interactive SVG annotation canvas
- 4 annotation tools (arrow, rectangle, text, freehand)
- Undo/redo stack management
- Keyboard shortcuts

### ✅ Phase 1: Export + History

- Export annotated screenshots as PNG
- Local file-based history (500MB budget)
- Thumbnail gallery
- Search by ticket ID

### 🚧 Phase 2: PII Detection + Redaction

- Tesseract.js OCR (WASM-based)
- Auto-detect email, phone, IP, credit card
- Manual blackout redaction tool
- Release-safe black box style enabled

### 🚧 Phase 3: Jira/Zendesk Upload

- API client for Jira Cloud + Zendesk
- OAuth token management (macOS Keychain)
- Ticket auto-detection from clipboard
- Upload confirmation + URL copy

### 🚧 Phase 4: Templates + Polish

- 3 built-in templates (Error Highlight, Click Here, Step by Step)
- Dark/light mode
- Settings panel (defaults, theme, API credentials)
- History cleanup (auto-delete after N days)

## Tech Stack

- **Frontend**: React 19, TypeScript, Vite
- **Backend**: Rust, Tauri 2
- **Screenshot**: macOS `screencapture` CLI
- **Annotations**: SVG (interactive), `html-to-image` (export)
- **Storage**: File-based
- **OCR**: tesseract.js (WASM)

---

**Current Status**: Launch, tests, and build are in active hardening.  
**Known limitation**: The global shortcut may be unavailable on some macOS setups, so the capture button is the guaranteed path.
