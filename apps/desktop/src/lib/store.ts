import { create } from "zustand"
import type { AppSettings, RecentJobRecord } from "@/features/compress/types"

type AppStore = {
  recentJobs: RecentJobRecord[]
  setRecentJobs: (jobs: RecentJobRecord[]) => void
  setSettings: (settings: AppSettings) => void
  settings: AppSettings | null
}

export const useAppStore = create<AppStore>((set) => ({
  recentJobs: [],
  setRecentJobs: (jobs) => set({ recentJobs: jobs }),
  setSettings: (settings) => set({ settings }),
  settings: null,
}))
