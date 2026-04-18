import { useCallback, useEffect, useState } from "react"
import {
  clearRecentJobs,
  getRecentJobs,
  getSettings,
  updateSettings as persistSettings,
} from "@/features/compress/api"
import type { PartialAppSettings } from "@/features/compress/types"
import { useAppStore } from "@/lib/store"
import { isTauri } from "@/lib/tauri"

export function useAppBootstrap() {
  const setRecentJobs = useAppStore((state) => state.setRecentJobs)
  const setSettings = useAppStore((state) => state.setSettings)
  const [isLoading, setIsLoading] = useState(true)
  const [isSavingSettings, setIsSavingSettings] = useState(false)

  const refresh = useCallback(async () => {
    setIsLoading(true)

    try {
      const [settings, recentJobs] = await Promise.all([
        getSettings(),
        getRecentJobs(),
      ])

      setSettings(settings)
      setRecentJobs(recentJobs)
    } finally {
      setIsLoading(false)
    }
  }, [setRecentJobs, setSettings])

  async function refreshRecentJobs() {
    setRecentJobs(await getRecentJobs())
  }

  async function clearJobs() {
    await clearRecentJobs()
    setRecentJobs([])
  }

  async function updateSettings(settings: PartialAppSettings) {
    setIsSavingSettings(true)

    try {
      const nextSettings = await persistSettings(settings)
      setSettings(nextSettings)
    } finally {
      setIsSavingSettings(false)
    }
  }

  useEffect(() => {
    const handle = window.setTimeout(() => {
      void refresh()
    }, 0)

    return () => {
      window.clearTimeout(handle)
    }
  }, [refresh])

  return {
    clearRecentJobs: clearJobs,
    isLoading,
    isSavingSettings,
    refresh,
    refreshRecentJobs,
    runtimeReady: isTauri(),
    updateSettings,
  }
}
