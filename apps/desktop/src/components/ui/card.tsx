import type { HTMLAttributes } from "react"
import { cn } from "@/lib/cn"

export function Card({ className, ...props }: HTMLAttributes<HTMLDivElement>) {
  return (
    <div
      className={cn(
        "rounded-[1.2rem] border border-white/10 bg-[#12141b]/90 p-5 shadow-[0_24px_80px_rgba(0,0,0,0.28)] backdrop-blur sm:p-6",
        className,
      )}
      {...props}
    />
  )
}
