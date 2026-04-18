import { open } from "@tauri-apps/plugin-dialog"
import { invokeCommand, isTauri } from "@/lib/tauri"
import type {
  AnalyzePdfResult,
  AppSettings,
  CompressionRequest,
  CompressionResult,
  PartialAppSettings,
  RecentJobRecord,
} from "./types"

const browserDefaults: AppSettings = {
  outputSuffix: ".compressed",
  keepRecentJobs: 20,
  revealOutputOnSuccess: false,
  openOutputOnSuccess: false,
}

export async function pickPdfPath() {
  if (!isTauri()) {
    return null
  }

  const selected = await open({
    multiple: false,
    filters: [{ name: "PDF", extensions: ["pdf"] }],
  })

  return typeof selected === "string" ? selected : null
}

export async function analyzePdf(inputPath: string) {
  return invokeCommand<AnalyzePdfResult>("analyze_pdf", { inputPath })
}

export async function compressPdf(request: CompressionRequest) {
  return invokeCommand<CompressionResult>("compress_pdf", { request })
}

export async function getRecentJobs() {
  if (!isTauri()) {
    return [] satisfies RecentJobRecord[]
  }

  return invokeCommand<RecentJobRecord[]>("get_recent_jobs")
}

export async function clearRecentJobs() {
  if (!isTauri()) {
    return
  }

  await invokeCommand("clear_recent_jobs")
}

export async function getSettings() {
  if (!isTauri()) {
    return browserDefaults
  }

  return invokeCommand<AppSettings>("get_settings")
}

export async function updateSettings(settings: PartialAppSettings) {
  if (!isTauri()) {
    return { ...browserDefaults, ...settings }
  }

  return invokeCommand<AppSettings>("update_settings", { settings })
}

export async function revealInFolder(path: string) {
  if (!isTauri()) {
    return
  }

  await invokeCommand("reveal_in_folder", { path })
}

export async function openFile(path: string) {
  if (!isTauri()) {
    return
  }

  await invokeCommand("open_file", { path })
}
