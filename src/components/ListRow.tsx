import type { ReactNode } from "react";

interface ListRowProps {
  children: ReactNode;
  onClick?: () => void;
  columns?: string;
}

export function ListRow({
  children,
  onClick,
  columns = "60px 1fr 200px 100px",
}: ListRowProps) {
  return (
    <div
      className="list-row"
      style={{
        gridTemplateColumns: columns,
        cursor: onClick ? "pointer" : "default",
      }}
      onClick={onClick}
      role={onClick ? "button" : undefined}
      tabIndex={onClick ? 0 : undefined}
    >
      {children}
    </div>
  );
}
