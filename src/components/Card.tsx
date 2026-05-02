import type { CSSProperties, ReactNode } from "react";

interface CardProps {
  locked?: boolean;
  children: ReactNode;
  style?: CSSProperties;
  className?: string;
}

export function Card({ locked, children, style, className }: CardProps) {
  const cls = ["card", locked ? "card-locked" : "", className]
    .filter(Boolean)
    .join(" ");
  return (
    <div className={cls} style={style}>
      {children}
    </div>
  );
}
