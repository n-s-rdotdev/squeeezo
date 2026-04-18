import { cva, type VariantProps } from "class-variance-authority"
import type { ButtonHTMLAttributes } from "react"
import { cn } from "@/lib/cn"

const buttonVariants = cva(
  "inline-flex items-center justify-center gap-2 rounded-[0.95rem] text-sm font-medium transition disabled:pointer-events-none disabled:opacity-50",
  {
    variants: {
      variant: {
        primary:
          "bg-zinc-100 px-4 py-2 text-zinc-950 shadow-[0_14px_30px_rgba(0,0,0,0.24)] hover:bg-white",
        secondary:
          "border border-white/10 bg-white/[0.06] px-4 py-2 text-zinc-100 hover:border-white/20 hover:bg-white/[0.1]",
        ghost: "px-3 py-2 text-zinc-300 hover:bg-white/[0.08] hover:text-white",
      },
    },
    defaultVariants: {
      variant: "primary",
    },
  },
)

type ButtonProps = ButtonHTMLAttributes<HTMLButtonElement> &
  VariantProps<typeof buttonVariants>

export function Button({ className, variant, ...props }: ButtonProps) {
  return (
    <button className={cn(buttonVariants({ variant }), className)} {...props} />
  )
}
