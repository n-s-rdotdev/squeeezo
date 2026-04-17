# Squeeezo Research

## Goal

Build a desktop-first PDF compression product with a simple user flow:

1. A user selects a PDF.
2. The app compresses it locally.
3. The output is saved beside the original file with a suffix.

The product should feel lightweight and native on macOS while still preserving the option to support other desktop platforms later.

## Final Product Direction

The current chosen direction is:

- `React + TypeScript + Vite`
- `Tauri v2`
- `shadcn/ui`
- local PDF compression using a bundled sidecar binary
- macOS Finder integration through a `Quick Action / Action Extension`

This direction was chosen over:

- a Next.js app hosted on Vercel with an AWS worker
- a Cloudflare-hosted Next.js app
- a fully native SwiftUI/AppKit macOS app

## Why Desktop-First Won

The product is fundamentally local-file oriented. A desktop application avoids unnecessary infrastructure and fits the core workflow better:

- no forced file upload to a remote backend
- better privacy posture because PDFs can stay on-device
- faster interaction for single-file utilities
- simpler first release than operating a web frontend, queue, storage layer, and worker fleet

## Why Tauri Won

`React + Tauri` is the best compromise between development speed and native capability.

Pros:

- strong fit for a web-heavy team
- reuse of React/TypeScript skills
- smaller and lighter than Electron
- access to native integrations through Tauri plugins and Rust commands
- supports bundling sidecar binaries for native compression tooling
- leaves open a path to Windows and Linux later

Tradeoff:

- for macOS-specific shell integration, Tauri is not enough by itself; native Apple extension targets are still needed

## Why shadcn/ui Fits

`shadcn/ui` works well with `Vite`, `Tailwind CSS`, and React and is a good match for a desktop utility app:

- fast UI assembly without locking into a black-box component library
- easy to customize for a more intentional desktop aesthetic
- pairs naturally with `Tailwind CSS v4`

Recommended frontend set:

- `React`
- `TypeScript`
- `Vite`
- `Tailwind CSS v4`
- `shadcn/ui`
- `lucide-react`
- `React Hook Form`
- `Zod`
- `Zustand`

## Rejected Web Architecture

An earlier serious option was:

- `Next.js` on Vercel
- PDF worker on AWS
- S3 for storage
- SQS for jobs
- Fargate or Lambda for compression

This was technically viable, especially with:

- `S3 + SQS + ECS Fargate`
- presigned upload/download URLs
- async job orchestration

However, it introduced complexity that does not match a local-file utility:

- backend infrastructure
- storage lifecycle management
- queueing and retries
- auth and job-state persistence
- operational cost and monitoring overhead

This architecture is still useful later if the product evolves into a team or SaaS workflow.

## Native Compression Strategy

Compression should not live in the React layer.

Preferred execution path:

`React UI -> Tauri command / shell plugin -> bundled sidecar binary -> output file`

This keeps the UI thin and allows the compression engine to use mature native tooling.

Good sidecar candidates:

- `Ghostscript`
- `qpdf`
- a custom Rust binary
- another packaged CLI if needed

Design preference:

- keep compression logic in one engine
- call that same engine from both the Tauri app and macOS Finder extension

## macOS Integration

### Desired User Flow

1. User selects a PDF in Finder.
2. User chooses `Compress PDF`.
3. The file is compressed.
4. A new file is written to the same directory with a suffix.

### Recommended macOS Integration

Use a macOS `Quick Action / Action Extension`.

Reason:

- this is the right Apple-native pattern for acting on selected files in Finder
- it matches the user’s mental model of processing a chosen file
- it is better aligned with the product than a Finder toolbar integration

### What Not to Use First

Avoid making a Finder Sync toolbar button the primary integration.

Reason:

- Finder Sync is better suited to apps that monitor and annotate synchronized folders
- the product here is not a sync client; it is a one-shot file processor

## Output File Naming

Initial output naming rule:

- `report.pdf` -> `report.compressed.pdf`

Collision handling:

- `report.compressed.pdf`
- `report.compressed-2.pdf`
- `report.compressed-3.pdf`

Principles:

- never overwrite the original by default
- write output to the same directory as the source
- preserve the original file unless the product later adds an explicit replace option

## Suggested v1 Project Structure

```text
squeeezo/
  docs/
    research.md
  apps/
    desktop/                 # React + Tauri app
  packages/
    compression-core/        # shared compression binary or engine wrapper
    ui/                      # optional shared UI code if needed later
  macos/
    CompressPDFExtension/    # Finder Quick Action / Action Extension
```

Notes:

- `apps/desktop` contains the Tauri product shell and settings/history UI
- `packages/compression-core` should be the single source of truth for compression behavior
- `macos/CompressPDFExtension` is intentionally separate because this is Apple-native integration, not a Tauri concern

## Recommended Tauri Plugins

Likely Tauri plugin set for v1:

- `dialog`
- `fs`
- `shell`
- `log`
- `store` or `sql`

Use cases:

- `dialog`: open/save file pickers
- `fs`: file existence and local reads/writes where needed
- `shell`: run the bundled sidecar
- `log`: app and command logging
- `store`: settings and lightweight persistence
- `sql`: only if job history/search needs to become richer

## Persistence Strategy

For v1:

- use Tauri `store` for app settings and recent values
- skip a full database unless there is a clear need for searchable history

Use SQLite only if the product needs:

- compression history
- saved presets
- error records
- analytics-friendly job metadata

## UX Direction

The app should not feel like a web dashboard inside a shell.

Visual direction:

- compact desktop utility
- clear hierarchy
- minimal navigation
- single-purpose primary actions
- support both drag-and-drop and Finder-triggered flows

Likely screens:

- main compression screen
- recent files/history
- settings
- about / update status

## Naming Ideas

Working repo name: `squeeezo`

Strong name candidates from brainstorming:

- `PDF Press`
- `SlimPDF`
- `Press PDF`
- `CompactPDF`
- `LeanPDF`
- `Squeeezo`

`Squeeezo` is more brandable and less generic than the purely descriptive options, but it should still be checked for domain, trademark, and App Store conflicts.

## Risks and Open Questions

Open product questions:

- Is the app strictly single-file, or should v1 support batch compression?
- Does compression need presets such as `smallest`, `balanced`, and `best quality`?
- Should the Finder Quick Action show progress or notifications on completion?
- Is the first release macOS-only, or should the Tauri app still be architected for Windows/Linux shortly after?

Open technical questions:

- Which compression engine gives the best quality/size tradeoff?
- Should the sidecar be a bundled third-party CLI or a small custom Rust wrapper?
- What signing/notarization work is needed once the Quick Action extension is added?
- How should logs and failures be surfaced to users when the action is invoked directly from Finder?

## Current Recommendation

Build v1 as:

- `React + TypeScript + Vite + Tauri v2`
- `shadcn/ui`
- local sidecar-based compression engine
- macOS Finder Quick Action for `Compress PDF`

This gives:

- low infrastructure overhead
- strong local-file ergonomics
- a practical macOS-first workflow
- future room for cross-platform desktop support
