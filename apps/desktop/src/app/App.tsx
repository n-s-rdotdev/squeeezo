import { RefreshCcw } from "lucide-react"
import { startTransition } from "react"
import { CompressWorkspace } from "@/features/compress/components/compress-workspace"
import { useAppBootstrap } from "@/hooks/use-app-bootstrap"
import { useAppStore } from "@/lib/store"

export function App() {
  const bootstrap = useAppBootstrap()
  const settings = useAppStore((state) => state.settings)

  return (
    <main className="h-screen bg-[radial-gradient(circle_at_top,rgba(96,165,250,0.12),transparent_30%),linear-gradient(180deg,#09090b_0%,#0f1117_48%,#09090b_100%)] text-zinc-100">
      <div className="mx-auto flex h-screen max-w-[1040px] flex-col px-5 py-5 sm:px-6">
        <header className="flex items-center justify-between gap-4 pb-5">
          <div className="space-y-1.5">
            <p className="font-['Avenir_Next','Space_Grotesk','Segoe_UI',sans-serif] text-3xl font-semibold uppercase tracking-[0.3em] text-white">
              Squeeezo
            </p>
            <h1 className="text-sm font-medium tracking-[0.02em] text-zinc-400">
              Compress one PDF.
            </h1>
          </div>

          <button
            className="inline-flex items-center justify-center gap-2 rounded-[0.95rem] border border-white/10 bg-white/[0.05] px-4 py-2 text-sm font-medium text-zinc-200 transition hover:border-white/20 hover:bg-white/[0.1]"
            onClick={() => {
              startTransition(() => {
                void bootstrap.refresh()
              })
            }}
            type="button"
          >
            <RefreshCcw className="h-4 w-4" />
            Refresh
          </button>
        </header>

        <section className="min-h-0 flex-1">
          <CompressWorkspace
            isSavingSettings={bootstrap.isSavingSettings}
            onCompressionFinished={() => bootstrap.refreshRecentJobs()}
            onSaveSettings={bootstrap.updateSettings}
            runtimeReady={bootstrap.runtimeReady}
            settings={settings}
          />
        </section>
      </div>
    </main>
  )
}
