import { invoke } from "@tauri-apps/api/core"

declare global {
  interface Window {
    __TAURI_INTERNALS__?: object
  }
}

export function isTauri() {
  return typeof window !== "undefined" && Boolean(window.__TAURI_INTERNALS__)
}

export async function invokeCommand<T = void>(
  command: string,
  payload?: Record<string, unknown>,
) {
  return invoke<T>(command, payload)
}
