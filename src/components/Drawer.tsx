import type { ReactNode } from "react";

interface DrawerProps {
  open: boolean;
  onClose: () => void;
  wide?: boolean;
  header?: ReactNode;
  children: ReactNode;
}

export function Drawer({ open, onClose, wide, header, children }: DrawerProps) {
  return (
    <>
      <div className={`drawer-scrim${open ? " open" : ""}`} onClick={onClose} />
      <aside
        className={`drawer${wide ? " drawer-wide" : ""}${open ? " open" : ""}`}
      >
        {header && (
          <div
            style={{
              padding: "20px 24px",
              borderBottom: "1px solid var(--rule-soft)",
              flexShrink: 0,
            }}
          >
            {header}
          </div>
        )}
        <div style={{ padding: "20px 24px", overflowY: "auto", flex: 1 }}>
          {children}
        </div>
      </aside>
    </>
  );
}
