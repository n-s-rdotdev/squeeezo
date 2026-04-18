import { zodResolver } from "@hookform/resolvers/zod"
import { Settings2 } from "lucide-react"
import { useEffect } from "react"
import { useForm } from "react-hook-form"
import { z } from "zod"
import { Button } from "@/components/ui/button"
import { Card } from "@/components/ui/card"
import type { AppSettings, PartialAppSettings } from "@/features/compress/types"

const settingsSchema = z.object({
  outputSuffix: z
    .string()
    .min(1, "Suffix is required.")
    .regex(/^\.[a-z0-9._-]+$/i, "Use a dot-prefixed filename suffix."),
  keepRecentJobs: z.coerce.number().int().min(1).max(100),
  revealOutputOnSuccess: z.boolean(),
  openOutputOnSuccess: z.boolean(),
})

type SettingsPanelProps = {
  isSaving: boolean
  settings: AppSettings | null
  onSave: (settings: PartialAppSettings) => Promise<void> | void
}

type SettingsFormInput = z.input<typeof settingsSchema>
type SettingsFormValues = z.output<typeof settingsSchema>

export function SettingsPanel({
  isSaving,
  settings,
  onSave,
}: SettingsPanelProps) {
  const form = useForm<SettingsFormInput, undefined, SettingsFormValues>({
    resolver: zodResolver(settingsSchema),
    defaultValues: settings ?? {
      outputSuffix: ".compressed",
      keepRecentJobs: 20,
      revealOutputOnSuccess: false,
      openOutputOnSuccess: false,
    },
  })

  useEffect(() => {
    if (!settings) {
      return
    }

    form.reset(settings)
  }, [form, settings])

  return (
    <Card>
      <div className="flex items-center gap-2">
        <Settings2 className="h-4 w-4 text-stone-500" />
        <div>
          <p className="text-xs uppercase tracking-[0.2em] text-stone-500">
            Settings
          </p>
          <h2 className="mt-2 text-xl font-semibold text-stone-950">
            Local defaults
          </h2>
        </div>
      </div>

      <form
        className="mt-5 grid gap-4"
        onSubmit={form.handleSubmit(async (values) => {
          await onSave(values)
        })}
      >
        <label className="grid gap-2 text-sm text-stone-700">
          <span>Output suffix</span>
          <input
            className="rounded-2xl border border-stone-300 bg-white px-4 py-3 outline-none transition focus:border-stone-950"
            {...form.register("outputSuffix")}
          />
          <span className="text-xs text-rose-600">
            {form.formState.errors.outputSuffix?.message}
          </span>
        </label>

        <label className="grid gap-2 text-sm text-stone-700">
          <span>Keep recent jobs</span>
          <input
            className="rounded-2xl border border-stone-300 bg-white px-4 py-3 outline-none transition focus:border-stone-950"
            type="number"
            {...form.register("keepRecentJobs", { valueAsNumber: true })}
          />
          <span className="text-xs text-rose-600">
            {form.formState.errors.keepRecentJobs?.message}
          </span>
        </label>

        <label className="flex items-center justify-between gap-4 rounded-2xl border border-stone-200 bg-white px-4 py-3 text-sm text-stone-700">
          <span>Reveal output on success</span>
          <input
            type="checkbox"
            {...form.register("revealOutputOnSuccess")}
          />
        </label>

        <label className="flex items-center justify-between gap-4 rounded-2xl border border-stone-200 bg-white px-4 py-3 text-sm text-stone-700">
          <span>Open output on success</span>
          <input type="checkbox" {...form.register("openOutputOnSuccess")} />
        </label>

        <Button disabled={isSaving} type="submit">
          {isSaving ? "Saving..." : "Save settings"}
        </Button>
      </form>
    </Card>
  )
}
