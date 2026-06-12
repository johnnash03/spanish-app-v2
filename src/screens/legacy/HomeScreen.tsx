// LEGACY (v1) — Quarantined in S1 (#32); v1 UI stays the default until the v2 home lands (S14, #45), then demotes to a Legacy menu entry. Do not extend. Deleted in S17 (#48).
// Replaced by: S14 home rework (#45).

import { useEffect, useState } from "react";
import { TopBar, Button, BadgeCount } from "../../components";
import {
  IconLayers,
  IconCards,
  IconSpark,
  IconArrowRight,
} from "../../components/icons";
import { LEARNER } from "../../data/mockData";
import {
  isTauri,
  getCurrentUnitNumber,
  getUnitByN,
  getPendingSession,
  getPipelineHealth,
} from "../../lib/tauri";
import type {
  LocalAttempt,
  Screen,
  PipelineHealth,
  PipelineBand,
} from "../../types";

interface HomeScreenProps {
  go: (screen: Screen) => void;
}

const BAND_LABELS: Record<PipelineBand, { label: string; color: string }> = {
  light: { label: "Light", color: "var(--accent)" },
  healthy: { label: "Healthy", color: "var(--accent)" },
  full: { label: "Full", color: "#8a6c30" },
  overloaded: { label: "Overloaded", color: "var(--bad)" },
};

export function HomeScreen({ go }: HomeScreenProps) {
  const [currentUnitN, setCurrentUnitN] = useState(LEARNER.currentUnit.number);
  const [currentUnitName, setCurrentUnitName] = useState(
    LEARNER.currentUnit.name,
  );
  const [pendingSession, setPendingSession] = useState<LocalAttempt[] | null>(
    null,
  );
  const [pipelineHealth, setPipelineHealth] = useState<PipelineHealth | null>(
    null,
  );

  useEffect(() => {
    if (!isTauri()) return;
    getCurrentUnitNumber()
      .then((n) => {
        setCurrentUnitN(n);
        return getUnitByN(n);
      })
      .then((u) => {
        if (u) setCurrentUnitName(u.name);
      })
      .catch(() => {});

    getPendingSession()
      .then((attempts) => setPendingSession(attempts))
      .catch(() => {});

    getPipelineHealth()
      .then(setPipelineHealth)
      .catch(() => {});
  }, []);

  const u = LEARNER;
  const hasWeakTags = u.weakTags.length > 0;
  const combinedUnlocked = u.activeWords >= 10;

  return (
    <div className="app fade-in">
      {/* TEMPORARY (S6, #37): entry into the v2 practice loop while the
          legacy home is still the default. The S14 home rework (#45)
          replaces this whole screen. */}
      <TopBar
        right={
          <button
            className="text-link"
            style={{ fontSize: 13, color: "var(--ink-3)" }}
            onClick={() => go({ name: "v2Units" })}
          >
            Practice v2 →
          </button>
        }
      />

      <div className="container" style={{ paddingTop: 28, paddingBottom: 80 }}>
        {/* Pending session banner */}
        {pendingSession && (
          <div
            style={{
              display: "flex",
              alignItems: "center",
              justifyContent: "space-between",
              background: "var(--paper-2)",
              border: "1px solid var(--accent)",
              borderRadius: "var(--r-md)",
              padding: "12px 18px",
              marginBottom: 16,
              gap: 12,
            }}
          >
            <p style={{ fontSize: 14, color: "var(--ink)", margin: 0 }}>
              You have an unsubmitted session — review now
            </p>
            <Button
              variant="primary"
              onClick={() =>
                go({ name: "sessionReview", attempts: pendingSession })
              }
            >
              Review
            </Button>
          </div>
        )}

        {/* Continue strip */}
        <button
          onClick={() => go({ name: "unitDetail", unitN: currentUnitN })}
          style={{
            width: "100%",
            display: "flex",
            alignItems: "center",
            justifyContent: "space-between",
            background: "var(--paper-2)",
            border: "1px solid var(--rule-soft)",
            borderRadius: "var(--r-md)",
            padding: "14px 18px",
            marginBottom: 28,
            textAlign: "left",
          }}
        >
          <div style={{ display: "flex", alignItems: "center", gap: 12 }}>
            <span className="eyebrow" style={{ marginRight: 4 }}>
              Continue
            </span>
            <span style={{ color: "var(--ink)" }}>
              {u.continueSession.label}
            </span>
            <span className="muted" style={{ fontSize: 13 }}>
              · last practiced 14 minutes ago
            </span>
          </div>
          <span
            style={{
              display: "inline-flex",
              alignItems: "center",
              gap: 6,
              color: "var(--ink-2)",
              fontSize: 13,
            }}
          >
            Resume <IconArrowRight size={16} />
          </span>
        </button>

        {/* Headline */}
        <div style={{ marginBottom: 36 }}>
          <h1
            className="serif"
            style={{ fontSize: 30, fontWeight: 400, letterSpacing: "-0.015em" }}
          >
            Buenas tardes.
          </h1>
          <p className="muted" style={{ marginTop: 6, fontSize: 14 }}>
            Three tracks. Pick where to put your attention today.
          </p>
        </div>

        {/* Track cards */}
        <div
          style={{
            display: "grid",
            gridTemplateColumns: "1.1fr 1fr 0.9fr",
            gap: 20,
          }}
        >
          {/* Grammar card */}
          <div
            className="card"
            style={{ display: "flex", flexDirection: "column", minHeight: 280 }}
          >
            <div className="row-between">
              <span className="eyebrow">Grammar</span>
              <IconLayers size={18} stroke={1.4} />
            </div>
            <div style={{ marginTop: 22, flex: 1 }}>
              <div className="serif" style={{ fontSize: 22, lineHeight: 1.25 }}>
                Unit {currentUnitN}
              </div>
              <div
                className="serif muted"
                style={{ fontSize: 16, marginTop: 2 }}
              >
                {currentUnitName}
              </div>
              <div className="muted" style={{ fontSize: 13, marginTop: 14 }}>
                {u.currentUnit.toward} of {u.currentUnit.of} toward mastery
              </div>
              <div
                style={{
                  height: 3,
                  background: "var(--rule-soft)",
                  borderRadius: 2,
                  marginTop: 10,
                }}
              >
                <div
                  style={{
                    width: `${(u.currentUnit.toward / u.currentUnit.of) * 100}%`,
                    height: "100%",
                    background: "var(--accent)",
                    borderRadius: 2,
                  }}
                />
              </div>

              {hasWeakTags && (
                <button
                  onClick={() => go({ name: "practiceEntry" })}
                  style={{
                    marginTop: 18,
                    display: "inline-flex",
                    alignItems: "center",
                    gap: 8,
                    padding: "8px 12px",
                    borderRadius: 999,
                    background: "transparent",
                    border: "1px solid var(--rule)",
                    fontSize: 13,
                    color: "var(--ink-2)",
                  }}
                >
                  <span
                    style={{
                      width: 6,
                      height: 6,
                      borderRadius: 999,
                      background: "var(--bad)",
                      flexShrink: 0,
                    }}
                  />
                  {u.weakTags.length} skills need review
                  <IconArrowRight size={14} />
                </button>
              )}
            </div>
            <div
              style={{
                display: "flex",
                alignItems: "center",
                gap: 16,
                marginTop: 22,
              }}
            >
              <Button
                variant="primary"
                onClick={() => go({ name: "unitDetail", unitN: currentUnitN })}
              >
                Continue Unit {currentUnitN}
              </Button>
              <button
                className="text-link"
                onClick={() => go({ name: "units" })}
              >
                Browse all units
              </button>
            </div>
          </div>

          {/* Vocabulary card */}
          <div
            className="card"
            style={{ display: "flex", flexDirection: "column", minHeight: 280 }}
          >
            <div className="row-between">
              <span className="eyebrow">Vocabulary</span>
              <IconCards size={18} stroke={1.4} />
            </div>
            <div style={{ marginTop: 22, flex: 1 }}>
              <div
                className="serif"
                style={{
                  fontSize: 44,
                  lineHeight: 1,
                  letterSpacing: "-0.02em",
                }}
              >
                {u.masteredCount}
              </div>
              <div className="muted" style={{ fontSize: 14, marginTop: 4 }}>
                words mastered
              </div>
              {pipelineHealth ? (
                <div
                  style={{
                    marginTop: 18,
                    fontSize: 13,
                    display: "flex",
                    alignItems: "center",
                    gap: 6,
                  }}
                >
                  <span
                    style={{
                      width: 6,
                      height: 6,
                      borderRadius: 999,
                      background: BAND_LABELS[pipelineHealth.band].color,
                      flexShrink: 0,
                    }}
                  />
                  <span
                    style={{
                      color: BAND_LABELS[pipelineHealth.band].color,
                      fontWeight: 600,
                    }}
                  >
                    {BAND_LABELS[pipelineHealth.band].label}
                  </span>
                  <span className="muted">
                    · {pipelineHealth.activeCount} active
                  </span>
                </div>
              ) : (
                <div
                  style={{ marginTop: 18, fontSize: 13, color: "var(--ink-2)" }}
                >
                  Pipeline {u.pipelineStatus.label.toLowerCase()} ·{" "}
                  {u.pipelineStatus.detail}
                </div>
              )}
              <div className="muted" style={{ fontSize: 12, marginTop: 4 }}>
                {u.learningCount} learning · {u.newCount} new
              </div>
            </div>
            <div
              style={{
                display: "flex",
                alignItems: "center",
                gap: 12,
                marginTop: 22,
              }}
            >
              <Button
                variant="accent"
                onClick={() => go({ name: "vocabSession" })}
              >
                Review <BadgeCount>{u.dueCount} due</BadgeCount>
              </Button>
              <button
                className="text-link"
                onClick={() => go({ name: "vocabIntake" })}
              >
                Learn new words
              </button>
            </div>
          </div>

          {/* Combined card */}
          <div
            className="card"
            style={{
              display: "flex",
              flexDirection: "column",
              minHeight: 280,
              opacity: combinedUnlocked ? 1 : 0.6,
            }}
          >
            <div className="row-between">
              <span className="eyebrow">Combined</span>
              <IconSpark size={18} stroke={1.4} />
            </div>
            {combinedUnlocked ? (
              <>
                <div
                  style={{
                    marginTop: 22,
                    flex: 1,
                    display: "flex",
                    flexDirection: "column",
                    justifyContent: "center",
                  }}
                >
                  <div
                    className="serif"
                    style={{
                      fontSize: 22,
                      lineHeight: 1.3,
                      letterSpacing: "-0.01em",
                    }}
                  >
                    Grammar &<br />
                    vocabulary,
                    <br />
                    woven together.
                  </div>
                  <div
                    style={{
                      marginTop: 16,
                      display: "inline-flex",
                      alignItems: "center",
                      gap: 8,
                      fontSize: 13,
                      color: "var(--accent)",
                    }}
                  >
                    <span
                      style={{
                        width: 6,
                        height: 6,
                        borderRadius: 999,
                        background: "var(--accent)",
                      }}
                    />
                    Ready
                  </div>
                </div>
                <div style={{ marginTop: 22 }}>
                  <Button
                    variant="secondary"
                    style={{ width: "100%" }}
                    onClick={() => go({ name: "combinedSession" })}
                  >
                    Practice
                  </Button>
                </div>
              </>
            ) : (
              <div
                style={{
                  marginTop: 22,
                  flex: 1,
                  display: "flex",
                  flexDirection: "column",
                  justifyContent: "center",
                }}
              >
                <div
                  className="serif"
                  style={{
                    fontSize: 16,
                    lineHeight: 1.5,
                    color: "var(--ink-2)",
                  }}
                >
                  Grammar &amp; vocabulary, woven together.
                </div>
                <div
                  className="muted"
                  style={{ fontSize: 13, marginTop: 14, lineHeight: 1.5 }}
                >
                  Unlocks when you have 10 words in your pipeline. You have{" "}
                  {u.activeWords}.
                </div>
              </div>
            )}
          </div>
        </div>

        {/* Footer */}
        <div
          style={{
            marginTop: 60,
            paddingTop: 20,
            borderTop: "1px solid var(--rule-soft)",
            display: "flex",
            justifyContent: "space-between",
            fontSize: 12,
          }}
          className="muted"
        >
          <span>
            Phase 2 of 4 · {u.masteredCount + 17} total reviews logged
          </span>
          <span>
            Press{" "}
            <span
              className="mono"
              style={{
                background: "var(--paper-2)",
                padding: "1px 6px",
                borderRadius: 3,
              }}
            >
              ?
            </span>{" "}
            for shortcuts
          </span>
        </div>
      </div>
    </div>
  );
}
