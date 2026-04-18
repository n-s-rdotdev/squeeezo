import { History, Trash2 } from "lucide-react"
import { Button } from "@/components/ui/button"
import { Card } from "@/components/ui/card"
import type { RecentJobRecord } from "@/features/compress/types"

type RecentJobsPanelProps = {
  isLoading: boolean
  jobs: RecentJobRecord[]
  onClear: () => void | Promise<void>
}

export function RecentJobsPanel({
  isLoading,
  jobs,
  onClear,
}: RecentJobsPanelProps) {
  return (
    <Card>
      <div className="flex items-center justify-between gap-3">
        <div>
          <p className="text-xs uppercase tracking-[0.2em] text-stone-500">
            Recent jobs
          </p>
          <h2 className="mt-2 text-xl font-semibold text-stone-950">
            Persisted locally
          </h2>
        </div>
        <Button onClick={() => void onClear()} type="button" variant="ghost">
          <Trash2 className="h-4 w-4" />
          Clear
        </Button>
      </div>

      <div className="mt-5 grid gap-3">
        {isLoading ? (
          <p className="text-sm text-stone-500">Loading recent jobs…</p>
        ) : jobs.length === 0 ? (
          <p className="text-sm leading-6 text-stone-600">
            No recent jobs yet. Successful and failed desktop runs will be
            stored here once the native command executes.
          </p>
        ) : (
          jobs.map((job) => (
            <article
              className="rounded-[1.35rem] border border-stone-200 bg-white px-4 py-3"
              key={job.id}
            >
              <div className="flex items-start justify-between gap-4">
                <div className="min-w-0">
                  <p className="truncate text-sm font-medium text-stone-950">
                    {job.inputPath}
                  </p>
                  <p className="mt-1 text-xs uppercase tracking-[0.16em] text-stone-500">
                    {job.status.replace("_", " ")}
                  </p>
                </div>
                <History className="mt-0.5 h-4 w-4 flex-none text-stone-400" />
              </div>
              <div className="mt-3 flex items-center justify-between gap-4 text-xs text-stone-500">
                <span>{new Date(job.createdAt).toLocaleString()}</span>
                <span>{job.durationMs}ms</span>
              </div>
            </article>
          ))
        )}
      </div>
    </Card>
  )
}
