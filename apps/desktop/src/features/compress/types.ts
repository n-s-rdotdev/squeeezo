export type CompressionSource = "desktop" | "finder_action" | "cli"

export type CompressionErrorCode =
  | "NOT_FOUND"
  | "NOT_PDF"
  | "PASSWORD_PROTECTED"
  | "READ_DENIED"
  | "WRITE_DENIED"
  | "OUT_OF_SPACE"
  | "ENGINE_MISSING"
  | "ENGINE_FAILED"
  | "CORRUPT_PDF"
  | "FILE_IN_USE"
  | "UNSUPPORTED_PLATFORM"
  | "UNKNOWN"

export type CompressionWarning = {
  code: string
  message: string
}

export type CompressionError = {
  code: CompressionErrorCode
  message: string
  details: string | null
}

export type AnalyzePdfResult = {
  inputPath: string
  bytes: number
  isPdf: boolean
  pageCount: number | null
  warnings: CompressionWarning[]
  error: CompressionError | null
}

export type CompressionRequest = {
  inputPath: string
  source: CompressionSource
  suffix?: string
}

export type CompressionResult = {
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

export type AppSettings = {
  outputSuffix: string
  keepRecentJobs: number
  revealOutputOnSuccess: boolean
  openOutputOnSuccess: boolean
}

export type PartialAppSettings = Partial<AppSettings>

export type RecentJobRecord = {
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
