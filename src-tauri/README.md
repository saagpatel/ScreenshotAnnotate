# Screenshot Annotate

A macOS desktop screenshot annotation tool built with Tauri 2 + React 19 that collapses the 3-minute manual annotation workflow into ~20 seconds.

## Features (Phase 0 - MVP)

✅ **Global Hotkey Capture** - Press `⌘⇧5` (Cmd+Shift+5) anywhere to trigger native macOS screenshot capture  
✅ **Interactive Annotation Canvas** - Draw arrows, rectangles, text, and freehand annotations  
✅ **Undo/Redo Stack** - Full undo/redo support with keyboard shortcuts  
✅ **Color Picker** - 4 preset colors (red, yellow, green, blue) + custom hex input  
✅ **Keyboard Shortcuts** - `A` = arrow, `R` = rectangle, `T` = text, `F` = freehand, `⌘Z` = undo, `⌘⇧Z` = redo

## Installation

See the repository-root [setup](../README.md#quick-start) and [verification guide](../README.md#verification). Run npm and Cargo commands from the repository root; native launch needs macOS and controlled test data.

### First Run Permissions

On first launch, macOS will prompt you to grant **Screen Recording** permission:

1. System Settings → Privacy & Security → Screen Recording
2. Enable permission for "Screenshot Annotate"
3. Restart the app

## Usage

1. **Capture**: Press `⌘⇧5` (or click "Capture Screenshot" button)
2. **Select region**: Drag to select the area you want to capture (native macOS crosshair)
3. **Annotate**:
   - Use toolbar buttons or keyboard shortcuts to select tools
   - Draw annotations on your screenshot
   - Change colors and thickness as needed
4. **Save**: Click "Save" button or press `⌘S` (Phase 2 will add export functionality)

## Keyboard Shortcuts

| Action | Shortcut |
|--------|----------|
| Capture screenshot | `⌘⇧5` |
| Switch to Arrow tool | `A` |
| Switch to Rectangle tool | `R` |
| Switch to Text tool | `T` |
| Switch to Freehand tool | `F` |
| Undo | `⌘Z` |
| Redo | `⌘⇧Z` |
| Save | `⌘S` |
| Cancel | `Esc` |

## Development Roadmap

### ✅ Phase 0: Screenshot Capture + Annotation (COMPLETE)
- macOS screenshot capture via `screencapture` CLI
- Interactive SVG annotation canvas
- 4 annotation tools (arrow, rectangle, text, freehand)
- Undo/redo stack management
- Keyboard shortcuts

### 🚧 Phase 1: Export + History (Next)
- Export annotated screenshots as PNG
- Local file-based history (500MB budget)
- Thumbnail gallery
- Search by ticket ID

### 📋 Phase 2: PII Detection + Redaction
- Tesseract.js OCR (WASM-based)
- Auto-detect email, phone, IP, credit card
- Manual redaction tool
- Blur/pixelate/blackbox styles

### 📋 Phase 3: Jira/Zendesk Upload
- API client for Jira Cloud + Zendesk
- OAuth token management (macOS Keychain)
- Ticket auto-detection from clipboard
- Upload confirmation + URL copy

### 📋 Phase 4: Templates + Polish
- 3 built-in templates (Error Highlight, Click Here, Step by Step)
- Dark/light mode
- Settings panel (hotkey config, retention, API credentials)
- History cleanup (auto-delete after N days)

## Tech Stack

- **Frontend**: React 19, TypeScript, Vite
- **Backend**: Rust, Tauri 2
- **Screenshot**: macOS `screencapture` CLI
- **Annotations**: SVG (interactive), `html-to-image` (export)
- **Storage**: File-based (Phase 1+)
- **OCR**: tesseract.js v5 (WASM) (Phase 2+)

---

**Current Status**: Phase 0 complete ✅  
**Next**: Phase 1 (Export + History) - ETA 3 days
