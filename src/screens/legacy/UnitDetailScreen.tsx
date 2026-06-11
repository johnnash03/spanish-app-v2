// LEGACY (v1) — Quarantined in S1 (#32); v1 UI stays the default until the v2 home lands (S14, #45), then demotes to a Legacy menu entry. Do not extend. Deleted in S17 (#48).
// Replaced by: v2 unit detail over the S2/S3 curriculum (#33/#34), default UI in S14 (#45).

import { useEffect, useRef, useState } from "react";
import { TopBar, Button, Callout } from "../../components";
import { getUnitByN as getMockUnit } from "../../data/mockData";
import {
  isTauri,
  triggerGeneration,
  getUnitGenerationState,
  retryGeneration,
  getUnitByN as getRealUnit,
} from "../../lib/tauri";
import type { GenerationState, Screen, Unit } from "../../types";

interface UnitDetailScreenProps {
  unitN: number;
  go: (screen: Screen) => void;
}

export function UnitDetailScreen({ unitN, go }: UnitDetailScreenProps) {
  const [unit, setUnit] = useState<Unit | null>(
    isTauri() ? null : (getMockUnit(unitN) ?? null),
  );
  const [genState, setGenState] = useState<GenerationState | null>(
    isTauri() ? null : (getMockUnit(unitN)?.generationState ?? "idle"),
  );
  const pollRef = useRef<ReturnType<typeof setInterval> | null>(null);

  // Load real unit data in Tauri
  useEffect(() => {
    if (!isTauri()) return;
    getRealUnit(unitN)
      .then((u) => setUnit(u))
      .catch(() => setUnit(null));
  }, [unitN]);

  // Trigger generation once we have the skill tag
  useEffect(() => {
    if (!unit?.skillTag || !isTauri()) return;
    triggerGeneration(unit.skillTag)
      .then((state) => setGenState(state))
      .catch(() => setGenState("idle"));
  }, [unit?.skillTag]);

  // Poll while generating
  useEffect(() => {
    if (!unit?.skillTag || !isTauri()) return;
    if (genState !== "generating") {
      if (pollRef.current) {
        clearInterval(pollRef.current);
        pollRef.current = null;
      }
      return;
    }
    pollRef.current = setInterval(() => {
      getUnitGenerationState(unit.skillTag!)
        .then((state) => setGenState(state))
        .catch(() => {});
    }, 1000);
    return () => {
      if (pollRef.current) clearInterval(pollRef.current);
    };
  }, [genState, unit?.skillTag]);

  if (isTauri() && unit === null) {
    return (
      <div className="app fade-in">
        <TopBar showHome onHome={() => go({ name: "home" })} hasRule />
        <div className="container" style={{ paddingTop: 36 }}>
          <p className="muted">Loading…</p>
        </div>
      </div>
    );
  }

  if (!unit) {
    return (
      <div className="app fade-in">
        <TopBar showHome onHome={() => go({ name: "home" })} hasRule />
        <div className="container" style={{ paddingTop: 36 }}>
          <p className="muted">Unit not found.</p>
        </div>
      </div>
    );
  }

  const isLoading = genState === null;
  const isGenerating = genState === "generating";
  const isFailed = genState === "failed";
  const canStart = !isLoading && !isGenerating && !isFailed;

  function handleRetry() {
    if (!unit?.skillTag || !isTauri()) return;
    retryGeneration(unit.skillTag)
      .then(() => setGenState("generating"))
      .catch(() => {});
  }

  return (
    <div className="app fade-in">
      <TopBar
        showHome
        onHome={() => go({ name: "home" })}
        hasRule
        right={
          <button
            className="text-link"
            style={{ fontSize: 13 }}
            onClick={() => go({ name: "units" })}
          >
            All units
          </button>
        }
      />

      <div
        className="container"
        style={{ paddingTop: 36, paddingBottom: 80, maxWidth: 680 }}
      >
        <div className="eyebrow" style={{ marginBottom: 8 }}>
          Phase {unit.phase} · Unit {String(unit.n).padStart(2, "0")}
        </div>
        <h1
          className="serif"
          style={{ fontSize: 32, fontWeight: 400, letterSpacing: "-0.015em" }}
        >
          {unit.name}
        </h1>
        {unit.description && (
          <p
            style={{
              marginTop: 12,
              fontSize: 16,
              lineHeight: 1.6,
              color: "var(--ink-2)",
              maxWidth: 560,
            }}
          >
            {unit.description}
          </p>
        )}

        {/* Notes glossary — only present in mock/browser preview */}
        {unit.notes && unit.notes.length > 0 && (
          <div style={{ marginTop: 40 }}>
            <div className="eyebrow" style={{ marginBottom: 14 }}>
              Vocabulary notes
            </div>
            <div
              style={{
                background: "var(--paper-2)",
                border: "1px solid var(--rule-soft)",
                borderRadius: "var(--r-lg)",
                overflow: "hidden",
              }}
            >
              {unit.notes.map((note, i) => (
                <div
                  key={i}
                  style={{
                    display: "grid",
                    gridTemplateColumns: "1fr 1fr",
                    gap: 16,
                    padding: "13px 20px",
                    borderBottom:
                      i < unit.notes!.length - 1
                        ? "1px solid var(--rule-soft)"
                        : "none",
                    alignItems: "baseline",
                  }}
                >
                  <span
                    className="serif"
                    style={{
                      fontSize: 14,
                      color: "var(--ink)",
                      fontStyle: "italic",
                    }}
                  >
                    {note.term}
                  </span>
                  <span
                    style={{
                      fontSize: 13,
                      color: "var(--ink-3)",
                      lineHeight: 1.5,
                    }}
                  >
                    {note.definition}
                  </span>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* Generation failure */}
        {isFailed && (
          <div style={{ marginTop: 36 }}>
            <Callout variant="bad">
              Exercise generation failed. Exercises couldn't be prepared.
            </Callout>
            <div style={{ marginTop: 12 }}>
              <Button variant="secondary" size="sm" onClick={handleRetry}>
                Retry generation
              </Button>
            </div>
          </div>
        )}

        {/* Start practice CTA */}
        <div
          style={{
            marginTop: 40,
            display: "flex",
            alignItems: "center",
            gap: 16,
          }}
        >
          <Button
            variant="primary"
            size="lg"
            disabled={!canStart}
            onClick={() =>
              unit.skillTag &&
              go({ name: "session", unitSkillTag: unit.skillTag })
            }
          >
            Start practice
          </Button>
          {(isLoading || isGenerating) && (
            <span style={{ fontSize: 13, color: "var(--ink-3)" }}>
              {isLoading ? "Checking…" : "Preparing your exercises…"}
            </span>
          )}
        </div>
      </div>
    </div>
  );
}
