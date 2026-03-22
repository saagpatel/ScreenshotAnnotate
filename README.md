# Screenshot Annotate

A macOS desktop screenshot annotation tool built with Tauri 2 + React 19 that collapses the 3-minute manual annotation workflow into ~20 seconds.

## Features

### Current shipped surface

**Screenshot Capture:**

- `Capture Screenshot` button launches native macOS screenshot capture
- Global shortcut `⌘⇧A` is attempted when macOS allows it
- Interactive region selection with native crosshair
- Automatic temp file management

**Interactive Annotation:**

- 5 annotation tools: Arrow (with arrowhead), Rectangle, Text, Freehand, Redact
- Undo/Redo stack (50 steps)
- Color picker (4 presets + custom hex)
- Thickness control (1-8px)
- Keyboard shortcuts: `A`/`R`/`T`/`F` for tools, `⌘Z`/`⇧⌘Z` for undo/redo

**Export + History:**

- Save annotated screenshots as PNG
- Local file-based storage (500MB budget with LRU eviction)
- Searchable thumbnail gallery
- Storage usage tracking
- Delete with confirmation

**Release-safe extras:**

- OCR-assisted PII detection
- Blackout redaction for detected or manual regions
- Jira/Zendesk credential storage and upload flow
- Templates, theme, and saved default annotation preferences

## Installation

### Prerequisites

- macOS 13+ (Ventura or later)
- Node.js 18+
- Rust 1.77.2+ (install via [rustup](https://rustup.rs/))

### Setup

```bash
# Install dependencies
npm install

# Run in normal development mode
npm run tauri dev

# Run in lean development mode (temp build caches + auto-clean on exit)
npm run tauri:dev:lean

# Build for production
npm run tauri build
```

## Project Housekeeping

Use these cleanup commands to reclaim disk space:

```bash
# Remove heavy build artifacts only (keeps dependencies)
npm run clean:heavy

# Remove all reproducible local caches/artifacts (including node_modules)
npm run clean:full
```

`npm run clean` remains available as an alias for `npm run clean:full`.

## Normal Dev vs Lean Dev

### Normal dev

```bash
npm run tauri dev
```

- Fastest restarts after the first run
- Writes build artifacts to the repo workspace (for example `src-tauri/target`, Vite cache)
- Best when you are actively iterating and disk space is not tight

### Lean dev

```bash
npm run tauri:dev:lean
```

- Starts the app with temporary cache locations:
  - Rust build output goes to a temporary `CARGO_TARGET_DIR`
  - Vite cache goes to a temporary `VITE_CACHE_DIR`
- Automatically deletes those temporary caches when the dev session exits
- Uses less persistent disk in the repo, but startup may be slower because caches are rebuilt each session

### First Run Permissions

On first launch, macOS will prompt you to grant **Screen Recording** permission:

1. System Settings → Privacy & Security → Screen Recording
2. Enable permission for "Screenshot Annotate"
3. Restart the app

## Usage

1. **Capture**: Click "Capture Screenshot" (global shortcut `⌘⇧A` is best-effort on macOS)
2. **Select region**: Drag to select the area (native macOS crosshair)
3. **Annotate**: Draw arrows, rectangles, text, or freehand
4. **Save**: Click "Save" button or press `⌘S`
5. **View History**: Click "View History" button on idle screen

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

### ✅ Phase 1: Export + History (COMPLETE)

- Export annotated screenshots as PNG (html-to-image + Rust compositing)
- Local file-based history (500MB budget)
- Thumbnail gallery with search
- Storage usage tracking
- LRU eviction

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
- History cleanup (future)

## Tech Stack

- **Frontend**: React 19, TypeScript, Vite, SVG rendering, html-to-image
- **Backend**: Rust, Tauri 2
- **Screenshot**: macOS `screencapture` CLI
- **Storage**: File-based (~/Library/Application Support/)
- **Current**: tesseract.js (OCR), macOS Keychain (credentials)

## Storage Location

Screenshots are saved to:

```
~/Library/Application Support/com.screenshot-annotate/history/
```

Each screenshot is stored in a self-contained directory with:

- `original.png` - unmodified capture
- `annotated.png` - final export with annotations
- `thumbnail.png` - 200px-wide preview
- `meta.json` - metadata
- `annotations.json` - annotation data

## Performance

All targets met on M4 Pro:

- Hotkey → crosshair: ~350ms ✅
- Annotation rendering: <16ms ✅
- Export + save: ~400ms ✅
- App startup: ~1.5s ✅

## Project Structure

```
screenshot-annotate/
├── src-tauri/           # Rust backend
│   ├── src/
│   │   ├── lib.rs       # Tauri app setup
│   │   ├── capture.rs   # Screenshot capture
│   │   ├── export.rs    # PNG compositing
│   │   └── history.rs   # File-based storage
│   └── Cargo.toml
├── src/                 # React frontend
│   ├── components/
│   ├── hooks/
│   ├── types/
│   └── App.tsx
└── README.md
```

## Contributing

This is a personal-use tool for IT support engineers. Feature requests and bug reports welcome via GitHub Issues.

## License

MIT

---

**Current Status**: Local build, unit tests, and Rust tests are green.  
**Known limitation**: Global shortcut registration can be blocked by macOS, so the capture button is the guaranteed path.  
**Release posture**: Black box redaction is treated as the safe release mode.
