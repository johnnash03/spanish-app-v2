import type { ReactNode } from "react";

type CalloutVariant = "accent" | "bad";

interface CalloutProps {
  variant?: CalloutVariant;
  icon?: ReactNode;
  children: ReactNode;
}

const styles: Record<
  CalloutVariant,
  { background: string; borderColor: string; color: string }
> = {
  accent: {
    background: "var(--accent-tint)",
    borderColor: "var(--accent)",
    color: "var(--accent-2)",
  },
  bad: {
    background: "var(--bad-soft)",
    borderColor: "var(--bad)",
    color: "var(--bad)",
  },
};

export function Callout({ variant = "accent", icon, children }: CalloutProps) {
  const s = styles[variant];
  return (
    <div
      style={{
        padding: "14px 18px",
        background: s.background,
        borderRadius: "var(--r-md)",
        borderLeft: `2px solid ${s.borderColor}`,
        display: "flex",
        alignItems: "center",
        gap: 10,
        color: s.color,
        fontSize: 14,
      }}
    >
      {icon && (
        <span style={{ color: s.borderColor, display: "flex", flexShrink: 0 }}>
          {icon}
        </span>
      )}
      {children}
    </div>
  );
}
