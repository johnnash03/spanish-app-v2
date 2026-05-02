import { useState } from "react";
import { TopBar } from "../components";
import { StatusDot } from "../components/StatusDot";
import { IconChevronDown } from "../components/icons";
import { PHASES, LEARNER } from "../data/mockData";
import type { Screen } from "../types";

interface UnitListScreenProps {
  go: (screen: Screen) => void;
}

function statusLabel(status: string) {
  if (status === "complete") return "Mastered";
  if (status === "in-progress") return "In progress";
  return "Not started";
}

export function UnitListScreen({ go }: UnitListScreenProps) {
  const currentPhase = LEARNER.currentUnit.phase;
  const [openPhases, setOpenPhases] = useState<Record<number, boolean>>(
    Object.fromEntries(
      PHASES.map((p) => [p.number, p.number === currentPhase]),
    ),
  );

  function togglePhase(n: number) {
    setOpenPhases((prev) => ({ ...prev, [n]: !prev[n] }));
  }

  return (
    <div className="app fade-in">
      <TopBar showHome onHome={() => go({ name: "home" })} hasRule />

      <div
        className="container"
        style={{ paddingTop: 36, paddingBottom: 80, maxWidth: 820 }}
      >
        <div className="eyebrow" style={{ marginBottom: 8 }}>
          Grammar
        </div>
        <h1
          className="serif"
          style={{ fontSize: 32, fontWeight: 400, letterSpacing: "-0.015em" }}
        >
          All units
        </h1>
        <p
          className="muted"
          style={{ marginTop: 6, fontSize: 14, maxWidth: 520 }}
        >
          Four phases, each building on the one before. Any unit is reachable —
          the unit detail will warn you if prerequisites aren't complete.
        </p>

        <div style={{ marginTop: 36 }}>
          {PHASES.map((phase) => {
            const open = !!openPhases[phase.number];
            const isCurrent = phase.number === currentPhase;
            const completedCount = phase.units.filter(
              (u) => u.status === "complete",
            ).length;

            return (
              <div
                key={phase.number}
                style={{ borderTop: "1px solid var(--rule-soft)" }}
              >
                <button
                  onClick={() => togglePhase(phase.number)}
                  style={{
                    width: "100%",
                    textAlign: "left",
                    padding: "18px 4px",
                    display: "flex",
                    alignItems: "center",
                    gap: 14,
                  }}
                >
                  <span
                    style={{
                      transform: open ? "rotate(0deg)" : "rotate(-90deg)",
                      transition: "transform 160ms ease",
                      color: "var(--ink-3)",
                      display: "flex",
                    }}
                  >
                    <IconChevronDown size={16} />
                  </span>
                  <span className="eyebrow" style={{ minWidth: 60 }}>
                    Phase {phase.number}
                  </span>
                  <span className="serif" style={{ fontSize: 18 }}>
                    {phase.name}
                  </span>
                  {isCurrent ? (
                    <span
                      className="pill pill-accent"
                      style={{ marginLeft: "auto" }}
                    >
                      Current
                    </span>
                  ) : (
                    <span
                      className="muted"
                      style={{ marginLeft: "auto", fontSize: 13 }}
                    >
                      {completedCount} / {phase.units.length}
                    </span>
                  )}
                </button>

                {open && (
                  <div style={{ paddingBottom: 8 }}>
                    {phase.units.map((unit) => (
                      <button
                        key={unit.n}
                        onClick={() =>
                          go({ name: "unitDetail", unitN: unit.n })
                        }
                        style={{
                          width: "100%",
                          textAlign: "left",
                          display: "grid",
                          gridTemplateColumns: "28px 60px 1fr 110px",
                          gap: 14,
                          alignItems: "center",
                          padding: "12px 18px",
                          borderRadius: "var(--r-md)",
                          transition: "background 100ms ease",
                        }}
                        onMouseEnter={(e) =>
                          (e.currentTarget.style.background = "var(--paper-2)")
                        }
                        onMouseLeave={(e) =>
                          (e.currentTarget.style.background = "transparent")
                        }
                      >
                        <StatusDot
                          status={
                            unit.status === "complete"
                              ? "complete"
                              : unit.status === "in-progress"
                                ? "in-progress"
                                : "default"
                          }
                        />
                        <span className="muted mono" style={{ fontSize: 13 }}>
                          U{String(unit.n).padStart(2, "0")}
                        </span>
                        <span className="serif" style={{ fontSize: 17 }}>
                          {unit.name}
                        </span>
                        <span
                          className="muted"
                          style={{ fontSize: 12, textAlign: "right" }}
                        >
                          {statusLabel(unit.status)}
                        </span>
                      </button>
                    ))}
                  </div>
                )}
              </div>
            );
          })}
          <div style={{ borderTop: "1px solid var(--rule-soft)" }} />
        </div>
      </div>
    </div>
  );
}
