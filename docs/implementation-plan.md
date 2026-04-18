# Squeeezo v1 Implementation Plan

## Summary

Build `Squeeezo` as a cross-platform desktop PDF compressor with a `React + TypeScript + Vite + Tauri v2` app, `shadcn/ui` frontend, and a shared Rust-based compression layer that uses a bundled `Ghostscript` CLI as the default engine. Ship the core app on `macOS`, `Windows`, and `Linux`; ship the Finder Quick Action only on `macOS`. Keep v1 focused on single-file compression, a single default compression mode, direct-download installers, and lightweight recent-job persistence.

## Locked Decisions

- Scope: `single-file only` in v1.
- Platforms: `cross-platform v1` for the main app; `macOS-only` shell integration.
- Finder behavior: `one-click` Quick Action using the app’s default compression behavior.
- Compression controls: `single default mode` only in v1; no preset picker or advanced tuning UI.
- Persistence: keep `recent jobs only`; do not add full searchable history yet.
- Distribution: `direct download`; `macOS` notarized, `Windows/Linux` packaged as direct-download artifacts.
- Engine licensing assumption: `open-source compatible`; using `Ghostscript` under `AGPL` is acceptable for v1.
- Default safety rule: never overwrite the original file; write a sibling file with a suffix.

## Repo Structure To Implement

```text
squeeezo/
  apps/
    desktop/
      package.json
      vite.config.ts
      tailwind.config.ts
      components.json
      src/
        app/
        components/
        features/
          compress/
          recent-jobs/
          settings/
        hooks/
        lib/
        styles/
      src-tauri/
        Cargo.toml
        tauri.conf.json
        src/
          main.rs
          commands/
          state/
  crates/
    compression-core/
      Cargo.toml
      src/
        lib.rs
        engine.rs
        naming.rs
        analyze.rs
        compress.rs
        errors.rs
        models.rs
    compression-cli/
      Cargo.toml
      src/main.rs
  macos/
    SqueeezoActionExtension/
      SqueeezoActionExtension.xcodeproj
      ActionExtension/
  scripts/
    fetch-sidecars/
    package-macos-extension/
  docs/
    research.md
    implementation-plan.md
    release.md
```

## Public Interfaces And Stable Types

- Tauri command `analyze_pdf(inputPath: string) -> AnalyzePdfResult`
- Tauri command `compress_pdf(request: CompressionRequest) -> CompressionResult`
- Tauri command `get_recent_jobs() -> RecentJobRecord[]`
- Tauri command `clear_recent_jobs() -> void`
- Tauri command `reveal_in_folder(path: string) -> void`
- Tauri command `open_file(path: string) -> void`
- Tauri command `get_settings() -> AppSettings`
- Tauri command `update_settings(settings: Partial<AppSettings>) -> AppSettings`

- CLI contract for shared engine:

```bash
squeeezo-compress --input /path/file.pdf --suffix .compressed --json
```

- `CompressionRequest`

```ts
type CompressionRequest = {
  inputPath: string
  source: "desktop" | "finder_action" | "cli"
  suffix?: string
}
```

- `CompressionResult`

```ts
type CompressionResult = {
  status: "success" | "no_gain" | "failed"
  inputPath: string
  outputPath: string | null
  originalBytes: number
  outputBytes: number | null
  reductionBytes: number | null
  reductionPercent: number | null
  durationMs: number
  warnings: CompressionWarning[]
  error: CompressionError | null
}
```

- `CompressionError.code`

```ts
"NOT_FOUND" | "NOT_PDF" | "PASSWORD_PROTECTED" | "READ_DENIED" |
"WRITE_DENIED" | "OUT_OF_SPACE" | "ENGINE_MISSING" | "ENGINE_FAILED" |
"CORRUPT_PDF" | "FILE_IN_USE" | "UNSUPPORTED_PLATFORM" | "UNKNOWN"
```

- `AppSettings`

```ts
type AppSettings = {
  outputSuffix: ".compressed"
  keepRecentJobs: 20
  revealOutputOnSuccess: false
  openOutputOnSuccess: false
}
```

- `RecentJobRecord`

```ts
type RecentJobRecord = {
  id: string
  createdAt: string
  source: "desktop" | "finder_action"
  inputPath: string
  outputPath: string | null
  originalBytes: number
  outputBytes: number | null
  status: "success" | "no_gain" | "failed"
  errorCode: string | null
  durationMs: number
}
```

## Architecture

- Frontend: `React`, `Vite`, `TypeScript`, `Tailwind CSS v4`, `shadcn/ui`, `React Hook Form`, `Zod`, `Zustand`, `lucide-react`.
- Backend bridge: Tauri Rust commands; keep all file-system writes and engine invocation in Rust, not in the React layer.
- Compression core: `crates/compression-core` as the single source of truth for validation, output naming, temp file handling, engine execution, error mapping, and result calculation.
- CLI wrapper: `crates/compression-cli` for native invocation by the macOS extension and future automation.
- Engine: bundle `Ghostscript` binaries per platform and invoke them from Rust with platform-specific adapters.
- Persistence: store `AppSettings` and `RecentJobRecord[]` locally; cap recent jobs at `20`; prune oldest entries.
- Logging: structured logs in debug builds; redact absolute file paths in release logs unless the user opts into diagnostics later.

## UI And UX

- Main desktop screen:
  - drag-and-drop zone
  - `Choose PDF` button
  - selected-file summary
  - `Compress` primary action
  - result card with bytes saved, percentage saved, `Reveal in Finder/Explorer`, and `Open Output`
- Secondary screens:
  - `Recent Jobs`
  - `Settings`
  - `About`
- Finder Quick Action:
  - visible only for a single selected PDF
  - runs immediately with no option sheet
  - returns a notification on success and failure
- Output naming:
  - `report.pdf -> report.compressed.pdf`
  - collisions: `report.compressed-2.pdf`, `report.compressed-3.pdf`, etc.
- No replace-original workflow in v1.

## Implementation Phases

### 1. Workspace and tooling

- Set up `pnpm` workspace for the frontend and root Rust workspace for native crates.
- Scaffold `apps/desktop` with `Vite + React + TypeScript`.
- Add `shadcn/ui`, `Tailwind v4`, linting, formatting, and shared TS config.
- Add Tauri v2 shell and wire desktop packaging for macOS, Windows, and Linux.

### 2. Compression core

- Implement `compression-core` models, validation, naming, temporary-file lifecycle, and error taxonomy.
- Implement engine adapter abstraction and a `GhostscriptAdapter`.
- Add no-gain handling: if output is not at least `1%` smaller or `32 KB` smaller, discard output and return `status: "no_gain"`.
- Add temp-output strategy: write to a temp directory first, then atomically move to the final path.

### 3. Tauri backend

- Implement commands for analyze, compress, settings, recent jobs, open file, and reveal in folder.
- Keep path validation and write permissions in Rust.
- Normalize all command errors to the stable `CompressionError.code` set.

### 4. React app

- Build compression screen, progress states, success/failure states, and recent jobs list.
- Use `Zod` for any user-editable settings validation.
- Keep a single default compression flow; do not expose engine arguments in v1.

### 5. macOS Quick Action

- Create a native Action Extension target in `macos/SqueeezoActionExtension`.
- Pass the selected file URL to `compression-cli`.
- Accept exactly one PDF; reject directories, multiple files, and unsupported UTIs.
- On completion, surface a notification and return control to Finder.
- Share settings with the host app via app group storage so the suffix and recent-job logging are consistent.

### 6. Packaging and release

- Bundle platform-specific Ghostscript binaries with the app.
- Add scripts for fetching, checksumming, and placing sidecars per platform.
- Build macOS `.dmg`, Windows `.msi`, Linux `.AppImage` and `.deb`.
- Sign and notarize macOS builds; package the Action Extension into the macOS app bundle during release.

### 7. CI and release automation

- CI matrix: `macos-latest`, `windows-latest`, `ubuntu-latest`.
- Run `pnpm test`, `cargo test`, frontend build, Tauri build, and packaging smoke checks.
- On tagged releases, build artifacts for all platforms; notarize macOS if secrets are present.

## Edge Cases And Required Behavior

- Non-PDF selected: reject with `NOT_PDF`.
- Uppercase or mixed-case extension: accept if file content/UTI is PDF.
- Password-protected or encrypted PDFs: fail with `PASSWORD_PROTECTED`.
- Corrupt or zero-byte PDFs: fail with `CORRUPT_PDF`.
- Already-optimized PDFs: return `no_gain`; do not keep larger output.
- Output path already exists: append incrementing numeric suffix.
- Read-only source directory: fail with `WRITE_DENIED`.
- File locked by another app: fail with `FILE_IN_USE`.
- Source file deleted between selection and compression: fail with `NOT_FOUND`.
- Symlinked input: resolve real path for processing, but write output adjacent to the user-visible selected file path.
- Network/cloud-synced folders: treat as supported, but if move/write fails, return `WRITE_DENIED` or `FILE_IN_USE` with a retry message.
- Insufficient disk space: fail with `OUT_OF_SPACE`; clean temp files.
- Engine binary missing or incompatible: fail with `ENGINE_MISSING` or `UNSUPPORTED_PLATFORM`.
- Engine exits non-zero: capture stderr, map to `ENGINE_FAILED`, and persist sanitized diagnostics.
- App closed during compression: process must terminate cleanly and temp files must be removed on next launch.
- Quick Action receives multiple files despite UI filtering: reject with a single clear error.
- Paths with spaces, Unicode, long names, or hidden files: must work.
- Original filename already includes `.compressed`: append again only if needed for collision avoidance; do not try to “smart-rename” existing user files.
- Very large files: stream/process through temp files; do not load full PDFs into memory in JS.
- Recent jobs store corruption: fall back to an empty list and recreate the store.
- Release logs and crash reports: never include PDF content; avoid absolute file paths in release telemetry/logging.

## Testing And Acceptance Criteria

- Frontend unit tests with `Vitest + React Testing Library` for:
  - file selection state
  - success/failure rendering
  - no-gain behavior
  - recent jobs rendering
  - settings validation
- Rust unit and integration tests for:
  - naming collisions
  - path normalization
  - temp file cleanup
  - no-gain threshold
  - error mapping
  - malformed input handling
- CLI integration tests for:
  - success JSON output
  - failure JSON output
  - stderr logging on engine failure
- Manual platform smoke tests:
  - macOS: app launch, compress from app, compress from Finder Quick Action, notarized install
  - Windows: app launch, compress from app, reveal in Explorer
  - Linux: app launch, compress from app, output file generation
- Acceptance criteria:
  - user can compress a single PDF from the app on all three platforms
  - macOS Finder Quick Action works without opening the app
  - originals are never overwritten
  - output lands beside the original with deterministic suffixing
  - no-gain outputs are discarded
  - recent jobs persist across restarts
  - all temporary files are cleaned up after success and on next launch after interrupted runs

## Assumptions And Defaults

- Intended plan file in repo: `docs/implementation-plan.md`.
- `Ghostscript` is the default engine because the project is assumed AGPL-compatible.
- `qpdf` is not included in v1 because structural optimization alone is not sufficient for the product goal.
- `Zustand` is sufficient; do not add heavier client state libraries in v1.
- `SQLite` is out of scope for v1; use lightweight local persistence only.
- Windows/Linux do not get Explorer/Nautilus context-menu integration in v1.
- Finder Quick Action uses the same default behavior as the desktop app and exposes no per-run options.
- If macOS extension sandboxing blocks same-folder writes in some cases, the fallback is: write to a coordinated temporary replacement directory, then finalize adjacent to the source using coordinated move semantics; if that still fails, return a clear permission error rather than silently redirecting the output elsewhere.
