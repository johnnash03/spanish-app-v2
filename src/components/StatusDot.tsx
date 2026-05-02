type DotStatus = "default" | "in-progress" | "complete";
type BadgeState = "new" | "learning" | "mastered" | "untouched";

interface StatusDotProps {
  status?: DotStatus;
}

export function StatusDot({ status = "default" }: StatusDotProps) {
  const cls = [
    "status-dot",
    status === "in-progress" ? "in-progress" : "",
    status === "complete" ? "complete" : "",
  ]
    .filter(Boolean)
    .join(" ");
  return <span className={cls} />;
}

interface StateBadgeProps {
  state: BadgeState;
}

export function StateBadge({ state }: StateBadgeProps) {
  return <span className={`state-badge ${state}`}>{state}</span>;
}
