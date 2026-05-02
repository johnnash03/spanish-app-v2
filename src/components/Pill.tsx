import type { ReactNode } from "react";

type PillVariant = "default" | "accent" | "ghost";

interface PillProps {
  variant?: PillVariant;
  children: ReactNode;
  className?: string;
}

const variantClass: Record<PillVariant, string> = {
  default: "",
  accent: "pill-accent",
  ghost: "pill-ghost",
};

export function Pill({ variant = "default", children, className }: PillProps) {
  const cls = ["pill", variantClass[variant], className]
    .filter(Boolean)
    .join(" ");
  return <span className={cls}>{children}</span>;
}
