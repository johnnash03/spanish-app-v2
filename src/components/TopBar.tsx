import type { ReactNode } from "react";
import { IconHouse, IconQuestion } from "./icons";

interface TopBarProps {
  onHome?: () => void;
  showHome?: boolean;
  right?: ReactNode;
  hasRule?: boolean;
  hideWordmark?: boolean;
}

export function TopBar({
  onHome,
  showHome,
  right,
  hasRule,
  hideWordmark,
}: TopBarProps) {
  return (
    <div className={`topbar${hasRule ? " has-rule" : ""}`}>
      <div className="left">
        {showHome && (
          <button
            className="icon-btn"
            onClick={onHome}
            aria-label="Home"
            title="Home"
          >
            <IconHouse />
          </button>
        )}
        {!hideWordmark && (
          <span className="wordmark">
            léxico<span className="dot">.</span>
          </span>
        )}
      </div>
      <div className="right">
        {right}
        <button
          className="icon-btn"
          aria-label="Shortcuts"
          title="Shortcuts (?)"
        >
          <IconQuestion />
        </button>
      </div>
    </div>
  );
}
