import {
  AlertTriangle,
  CheckCircle2,
  FolderOpen,
  LoaderCircle,
  Settings2,
} from "lucide-react"
import { useEffect, useState } from "react"
import { Button } from "@/components/ui/button"
import { Card } from "@/components/ui/card"
import {
  analyzePdf,
  compressPdf,
  openFile,
  pickPdfPath,
  revealInFolder,
} from "@/features/compress/api"
import type {
  AnalyzePdfResult,
  AppSettings,
  CompressionResult,
} from "@/features/compress/types"

type CompressWorkspaceProps = {
  isSavingSettings: boolean
  onCompressionFinished: () => void | Promise<void>
  onSaveSettings: (settings: Partial<AppSettings>) => void | Promise<void>
  runtimeReady: boolean
  settings: AppSettings | null
}

const defaultSettings: AppSettings = {
  outputSuffix: ".compressed",
  keepRecentJobs: 20,
  revealOutputOnSuccess: false,
  openOutputOnSuccess: false,
}

function formatBytes(bytes: number | null) {
  if (bytes === null) {
    return "N/A"
  }

  return new Intl.NumberFormat("en", {
    maximumFractionDigits: 1,
    notation: "compact",
  }).format(bytes)
}

export function CompressWorkspace({
  isSavingSettings,
  onCompressionFinished,
  onSaveSettings,
  runtimeReady,
  settings,
}: CompressWorkspaceProps) {
  const [selectedPath, setSelectedPath] = useState<string | null>(null)
  const [analysis, setAnalysis] = useState<AnalyzePdfResult | null>(null)
  const [result, setResult] = useState<CompressionResult | null>(null)
  const [isAnalyzing, setIsAnalyzing] = useState(false)
  const [isCompressing, setIsCompressing] = useState(false)
  const [runtimeNote, setRuntimeNote] = useState<string | null>(null)

  const activeSettings = settings ?? defaultSettings
  const [outputSuffix, setOutputSuffix] = useState(activeSettings.outputSuffix)
  const [revealOutputOnSuccess, setRevealOutputOnSuccess] = useState(
    activeSettings.revealOutputOnSuccess,
  )
  const [openOutputOnSuccess, setOpenOutputOnSuccess] = useState(
    activeSettings.openOutputOnSuccess,
  )

  useEffect(() => {
    setOutputSuffix(activeSettings.outputSuffix)
    setRevealOutputOnSuccess(activeSettings.revealOutputOnSuccess)
    setOpenOutputOnSuccess(activeSettings.openOutputOnSuccess)
  }, [
    activeSettings.openOutputOnSuccess,
    activeSettings.outputSuffix,
    activeSettings.revealOutputOnSuccess,
  ])

  const suffixError =
    outputSuffix.length === 0
      ? "Suffix is required."
      : /^\.[a-z0-9._-]+$/i.test(outputSuffix)
        ? null
        : "Use a dot-prefixed filename suffix."
  const settingsChanged =
    outputSuffix !== activeSettings.outputSuffix ||
    revealOutputOnSuccess !== activeSettings.revealOutputOnSuccess ||
    openOutputOnSuccess !== activeSettings.openOutputOnSuccess

  async function saveSettings() {
    if (suffixError) {
      return
    }

    await onSaveSettings({
      openOutputOnSuccess,
      outputSuffix,
      revealOutputOnSuccess,
    })
  }

  async function chooseFile() {
    setRuntimeNote(null)
    setResult(null)
    setAnalysis(null)

    const path = await pickPdfPath()
    if (!path) {
      if (!runtimeReady) {
        setRuntimeNote(
          "Tauri runtime features are unavailable in the browser preview. Use `pnpm tauri:dev` for native file access.",
        )
      }
      return
    }

    setSelectedPath(path)
    setIsAnalyzing(true)

    try {
      const nextAnalysis = await analyzePdf(path)
      setAnalysis(nextAnalysis)
    } finally {
      setIsAnalyzing(false)
    }
  }

  async function runCompression() {
    if (!selectedPath) {
      return
    }

    setIsCompressing(true)

    try {
      const nextResult = await compressPdf({
        inputPath: selectedPath,
        source: "desktop",
        suffix: outputSuffix,
      })

      setResult(nextResult)
      await onCompressionFinished()

      if (nextResult.status === "success" && nextResult.outputPath) {
        if (revealOutputOnSuccess) {
          await revealInFolder(nextResult.outputPath)
        }
        if (openOutputOnSuccess) {
          await openFile(nextResult.outputPath)
        }
      }
    } finally {
      setIsCompressing(false)
    }
  }

  return (
    <Card className="flex h-full flex-col gap-5 overflow-hidden">
      <div className="flex flex-wrap items-start justify-between gap-3">
        <div className="space-y-1.5">
          <p className="text-xs uppercase tracking-[0.22em] text-zinc-500">
            Single-file workflow
          </p>
          <h2 className="font-['Avenir_Next','Space_Grotesk','Segoe_UI',sans-serif] text-2xl font-semibold tracking-[-0.04em] text-white">
            Choose a PDF, then compress it.
          </h2>
        </div>
        <div className="rounded-[0.95rem] border border-emerald-500/20 bg-emerald-500/10 px-3 py-1.5 text-xs font-medium text-emerald-300">
          Suffix {outputSuffix}
        </div>
      </div>

      <div className="grid min-h-0 flex-1 content-start gap-4">
        <div className="flex flex-wrap items-center gap-3">
          <Button onClick={() => void chooseFile()} type="button">
            Choose PDF
          </Button>
          <Button
            disabled={!selectedPath || isCompressing || !runtimeReady}
            onClick={() => void runCompression()}
            type="button"
            variant="secondary"
          >
            {isCompressing ? (
              <>
                <LoaderCircle className="h-4 w-4 animate-spin" />
                Compressing
              </>
            ) : (
              "Compress"
            )}
          </Button>
        </div>

        <div className="rounded-[1rem] border border-white/10 bg-black/20 p-4">
          <p className="text-xs uppercase tracking-[0.18em] text-zinc-500">
            Selected input
          </p>
          <p className="mt-2 break-all text-sm leading-6 text-zinc-200">
            {selectedPath ?? "No file selected yet."}
          </p>
          {runtimeNote ? (
            <p className="mt-3 text-sm text-amber-300">{runtimeNote}</p>
          ) : null}
        </div>

        <div className="grid gap-3 sm:grid-cols-3">
          <div className="rounded-[1rem] border border-white/10 bg-black/20 px-4 py-3">
            <p className="text-xs uppercase tracking-[0.18em] text-zinc-500">
              Bytes
            </p>
            <p className="mt-2 text-lg font-semibold text-white">
              {isAnalyzing ? "Inspecting..." : formatBytes(analysis?.bytes ?? null)}
            </p>
          </div>
          <div className="rounded-[1rem] border border-white/10 bg-black/20 px-4 py-3">
            <p className="text-xs uppercase tracking-[0.18em] text-zinc-500">
              PDF
            </p>
            <p className="mt-2 text-lg font-semibold text-white">
              {isAnalyzing ? "Inspecting..." : analysis?.isPdf ? "Yes" : "Unknown"}
            </p>
          </div>
          <div className="rounded-[1rem] border border-white/10 bg-black/20 px-4 py-3">
            <p className="text-xs uppercase tracking-[0.18em] text-zinc-500">
              Pages
            </p>
            <p className="mt-2 text-lg font-semibold text-white">
              {isAnalyzing ? "Inspecting..." : analysis?.pageCount ?? "Unknown"}
            </p>
          </div>
        </div>

        {analysis?.error ? (
          <p className="rounded-[1rem] border border-rose-500/20 bg-rose-500/10 px-4 py-3 text-sm text-rose-200">
            {analysis.error.code}: {analysis.error.message}
          </p>
        ) : null}

        <div className="grid gap-4 rounded-[1rem] border border-white/10 bg-white/[0.03] p-4">
          <div className="flex items-center gap-2 text-zinc-100">
            <Settings2 className="h-4 w-4 text-zinc-400" />
            <h3 className="font-semibold">Output</h3>
          </div>

          <div className="grid gap-3 lg:grid-cols-[minmax(0,1.1fr)_minmax(220px,0.8fr)_minmax(220px,0.8fr)_auto] lg:items-end">
            <label className="grid gap-2 text-sm text-zinc-300">
              <span>Suffix</span>
              <input
                className="rounded-[0.95rem] border border-white/10 bg-black/20 px-4 py-3 text-white outline-none transition focus:border-white/25"
                onChange={(event) => setOutputSuffix(event.target.value)}
                value={outputSuffix}
              />
            </label>

            <label className="flex items-center justify-between gap-4 rounded-[0.95rem] border border-white/10 bg-black/20 px-4 py-3 text-sm text-zinc-300">
              <span>Reveal on success</span>
              <input
                checked={revealOutputOnSuccess}
                className="h-4 w-4 accent-zinc-100"
                onChange={(event) => setRevealOutputOnSuccess(event.target.checked)}
                type="checkbox"
              />
            </label>

            <label className="flex items-center justify-between gap-4 rounded-[0.95rem] border border-white/10 bg-black/20 px-4 py-3 text-sm text-zinc-300">
              <span>Open on success</span>
              <input
                checked={openOutputOnSuccess}
                className="h-4 w-4 accent-zinc-100"
                onChange={(event) => setOpenOutputOnSuccess(event.target.checked)}
                type="checkbox"
              />
            </label>

            <Button
              className="w-full lg:w-auto lg:px-6"
              disabled={Boolean(suffixError) || !settingsChanged || isSavingSettings}
              onClick={() => void saveSettings()}
              type="button"
            >
              {isSavingSettings ? "Saving..." : "Save output settings"}
            </Button>
          </div>

          {suffixError ? (
            <p className="text-xs text-rose-300">{suffixError}</p>
          ) : null}
        </div>

        {result ? (
          <div className="rounded-[1rem] border border-white/10 bg-black/20 p-4">
            <div className="flex flex-wrap items-center justify-between gap-3">
              <div className="flex items-center gap-2 text-zinc-100">
                {result.status === "success" ? (
                  <CheckCircle2 className="h-4 w-4 text-emerald-400" />
                ) : (
                  <AlertTriangle className="h-4 w-4 text-amber-300" />
                )}
                <h3 className="font-semibold capitalize">
                  {result.status.replace("_", " ")}
                </h3>
              </div>

              <div className="text-sm text-zinc-400">
                Saved{" "}
                <span className="font-medium text-white">
                  {result.reductionPercent !== null
                    ? `${result.reductionPercent.toFixed(1)}%`
                    : "N/A"}
                </span>
              </div>
            </div>

            <div className="mt-4 grid gap-3 sm:grid-cols-3">
              <div>
                <p className="text-xs uppercase tracking-[0.18em] text-zinc-500">
                  Original
                </p>
                <p className="mt-2 text-sm font-medium text-white">
                  {formatBytes(result.originalBytes)}
                </p>
              </div>
              <div>
                <p className="text-xs uppercase tracking-[0.18em] text-zinc-500">
                  Output
                </p>
                <p className="mt-2 text-sm font-medium text-white">
                  {formatBytes(result.outputBytes)}
                </p>
              </div>
              <div>
                <p className="text-xs uppercase tracking-[0.18em] text-zinc-500">
                  Duration
                </p>
                <p className="mt-2 text-sm font-medium text-white">
                  {(result.durationMs / 1000).toFixed(1)}s
                </p>
              </div>
            </div>

            {result.error ? (
              <p className="mt-4 rounded-[1rem] border border-rose-500/20 bg-rose-500/10 px-3 py-2 text-sm text-rose-200">
                {result.error.code}: {result.error.message}
              </p>
            ) : null}

            <div className="mt-4 flex flex-wrap gap-2">
              <Button
                disabled={!result.outputPath}
                onClick={() => {
                  if (result.outputPath) {
                    void revealInFolder(result.outputPath)
                  }
                }}
                type="button"
                variant="ghost"
              >
                <FolderOpen className="h-4 w-4" />
                Reveal output
              </Button>
              <Button
                disabled={!result.outputPath}
                onClick={() => {
                  if (result.outputPath) {
                    void openFile(result.outputPath)
                  }
                }}
                type="button"
                variant="ghost"
              >
                Open output
              </Button>
            </div>
          </div>
        ) : null}
      </div>
    </Card>
  )
}
