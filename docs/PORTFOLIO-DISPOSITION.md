# ScreenshotAnnotate — Portfolio Disposition

**Status:** Active — working Tauri 2 + React 19 macOS screenshot
annotation app on `origin/main`, Phases 0 through 4 shipped, no
release-readiness doc yet. Disposition is **not** Release Frozen;
the gate is "decide whether to package this for distribution."

> Disposition uses strict `origin/main` verification.

---

## Verification posture

This repo has both `origin` (`saagpatel/ScreenshotAnnotate`) and
`legacy-origin` (`saagar210/ScreenshotAnnotate`) remotes. Disposition
reads `origin/main` only.

Specifically verified:
- `origin/main` tip: `2469335` chore: add initial CHANGELOG
- Substantive feature commits on `origin/main`:
  - `8b8b860` fix(audit): harden security and close deferred risks
  - `2d22ad0` fix: critical bug fixes from codebase audit
  - `fe49fb8` feat: implement Phase 4 templates and polish
  - `dd146fb` feat: implement Phase 3 Jira/Zendesk upload
  - `fe78420` feat: implement Phase 2 PII detection and redaction
  - `1797edd` feat: Phase 1 complete — export + history
  - `182e3e6` feat: Phase 0 complete — screenshot capture + annotation canvas
- **No `docs/` directory** on `origin/main` (this file creates it)
- Source tree: standard Tauri/React/TS layout under `src-tauri/`,
  `src/`, `scripts/`

---

## Current state in one paragraph

ScreenshotAnnotate is a macOS screenshot-and-annotate desktop app: hit
`⌘⇧5`, select a region (interactive native macOS capture), annotate
with arrows / rectangles / text / freehand, save the annotated PNG.
Tauri 2 + Rust + React 19, 50-step undo/redo, color + thickness
control, Tesseract.js OCR for searchable history, thumbnail gallery
with LRU eviction at 500MB. Phase 0 through 4 complete: capture +
annotation canvas, export + history, PII detection and redaction,
Jira/Zendesk upload, templates and polish. A security audit pass
hardened deferred risks. No release-readiness packet has been
written — no runbook, signing checklist, or notarization plan.

For full detail see `README.md`.

---

## Why "Active" instead of Release Frozen

Same shape as Conductor / SnippetLibrary / Chronomap. The signing
cluster (10 repos after this round: DesktopPEt / ContentEngine /
AIGCCore / Relay / FreeLanceInvoice / Nexus / DeepTank / OPscinema /
ShipKit / SignalFlow) all have release-readiness docs.
ScreenshotAnnotate doesn't. The next move is operator decision-time
about distribution, not signing-credential-time.

The Phase 4 polish commit suggests the operator iterated past v1
feature completeness. The "Jira/Zendesk upload" feature is notable —
it implies an enterprise use case (IT support, customer support
workflows) that distinguishes this from generic screenshot tools.

---

## Possible next moves (operator choice)

### Option 1 — Package for distribution and join the signing cluster

Required scope:

1. Write `docs/RELEASE-READINESS.md`
2. Wire Tauri 2 macOS signing + notarization
3. Cut a v1.0 release
4. Decide distribution channel:
   - GitHub Releases (DMG download)
   - Mac App Store (sandbox restrictions may conflict with global hotkey + native screenshot capture — investigate)
   - Homebrew cask
   - Setapp submission (the Jira/Zendesk integrations are well-aligned with Setapp's IT/dev audience)

Estimated effort: ~4 hours for signing setup + release runbook +
distribution-channel decision.

### Option 2 — Open-source as a build-locally tool

Document the local-build path well, no signing. Users who want it
clone and run themselves.

Estimated effort: ~30 minutes.

### Option 3 — Mark as personal-use IT-support tool

Decide the audience is just the author (IT support / DevOps work).
Move to `Cold Storage` with disposition. Keep repo public.

Estimated effort: ~15 minutes.

---

## Recommendation (informational)

**Option 1 is the natural fit.** Reasoning:

- Phase 4 polish + security audit pass suggest pre-release discipline
- Jira/Zendesk upload + PII detection are clear enterprise
  differentiators (most screenshot tools don't have either)
- Setapp distribution is a real revenue path with low operator
  overhead vs custom direct sales
- The category has paying users (CleanShot X, Shottr, etc.)

But operator-judgment. The Mac App Store sandboxing question is real
and worth investigating before locking in distribution channel.

---

## Portfolio operating system instructions

| Aspect | Posture |
|---|---|
| Portfolio status | `Active` |
| Next packet shape | "Decide between Option 1 / 2 / 3" — and if Option 1, also "investigate Mac App Store sandbox compatibility" |
| Review cadence | Resume normal cadence — this row needs decision-time |
| Resurface conditions | Once the operator picks an option, surface a packet for the work each option implies |
| Do **not** auto-add to signing cluster | The cluster is for repos that already have release-readiness docs. ScreenshotAnnotate doesn't yet. |

---

## Reactivation procedure (for the next code session)

1. **Verify local clone tracking.** `git branch -vv` — if `main`
   tracks `legacy-origin/main`, retarget to `origin/main`. Trap
   reference: FreeLanceInvoice + PersonalKBDrafter corrections.
2. Delete stale `codex/*` branches that pre-date the Phase 4 polish
   commit.
3. Re-run `pnpm install && pnpm tauri build` to confirm the
   toolchain still works after the freeze.
4. If picking Option 1, write the release-readiness doc first and
   investigate App Store sandbox compatibility before any signing
   work.

---

## Last known reference

| Field | Value |
|---|---|
| `origin/main` tip | `2469335` chore: add initial CHANGELOG |
| Last substantive commit | `8b8b860` fix(audit): harden security and close deferred risks |
| Phases completed | 0 (capture + canvas) through 4 (templates + polish) |
| Security audit | Run + hardened (`8b8b860`, `2d22ad0`) |
| Build verification status | green |
| Release readiness doc | **None** — gate before joining the signing cluster |
| Migration note | `legacy-origin` points at frozen `saagar210/ScreenshotAnnotate`; do not push there |
| Notable | Jira + Zendesk upload + PII detection = real enterprise differentiation |
