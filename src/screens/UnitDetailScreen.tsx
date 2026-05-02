import { useEffect, useRef, useState } from "react";
import { TopBar, Button, Callout } from "../components";
import { IconArrowRight } from "../components/icons";
import {
  getUnitByN,
  hasUnmetPrereqs,
  getMissingPrereqNames,
} from "../data/mockData";
import {
  isTauri,
  triggerGeneration,
  getUnitGenerationState,
  retryGeneration,
} from "../lib/tauri";
import type { GenerationState, Screen } from "../types";

interface UnitDetailScreenProps {
  unitN: number;
  go: (screen: Screen) => void;
}

export function UnitDetailScreen({ unitN, go }: UnitDetailScreenProps) {
  const unit = getUnitByN(unitN);
  const mockGenState: GenerationState = unit?.generationState ?? "idle";
  const [genState, setGenState] = useState<GenerationState>(mockGenState);
  const pollRef = useRef<ReturnType<typeof setInterval> | null>(null);

  // Trigger generation on mount when running in Tauri
  useEffect(() => {
    if (!unit?.skillTag || !isTauri()) return;

    triggerGeneration(unit.skillTag)
      .then((state) => setGenState(state))
      .catch(() => {
        /* ignore – non-critical */
      });
  }, [unit?.skillTag]);

  // Poll generation state while generating
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
        .catch(() => {
          /* ignore */
        });
    }, 1000);

    return () => {
      if (pollRef.current) clearInterval(pollRef.current);
    };
  }, [genState, unit?.skillTag]);

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

  const unmetPrereqs = hasUnmetPrereqs(unitN);
  const missingNames = getMissingPrereqNames(unitN);
  const isGenerating = genState === "generating";
  const isFailed = genState === "failed";
  const canStart = !isGenerating && !isFailed;

  function handleRetry() {
    if (!unit?.skillTag || !isTauri()) return;
    retryGeneration(unit.skillTag)
      .then(() => setGenState("generating"))
      .catch(() => {
        /* ignore */
      });
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

        {/* Prerequisite warning */}
        {unmetPrereqs && (
          <div style={{ marginTop: 28 }}>
            <Callout variant="bad">
              <span>
                You haven't completed{" "}
                <em className="serif" style={{ fontStyle: "italic" }}>
                  {missingNames.length > 0
                    ? missingNames.join(", ")
                    : "earlier units"}
                </em>
                . You can practice anyway, or finish those units first.
              </span>
            </Callout>
            <div style={{ marginTop: 10, display: "flex", gap: 12 }}>
              <button
                className="text-link text-link-accent"
                style={{ fontSize: 13 }}
                onClick={() => go({ name: "units" })}
              >
                Browse earlier units <IconArrowRight size={13} />
              </button>
            </div>
          </div>
        )}

        {/* Notes glossary */}
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
              Exercise generation failed. Your notes are ready, but exercises
              couldn't be prepared.
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
          {isGenerating && (
            <span style={{ fontSize: 13, color: "var(--ink-3)" }}>
              Preparing your exercises…
            </span>
          )}
        </div>
      </div>
    </div>
  );
}
