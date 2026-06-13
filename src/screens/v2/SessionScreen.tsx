// V2 practice session (S6, #37) — one-at-a-time typing flow per
// ui-design-spec 4.4: auto-focused input, Enter advances, no per-item
// verdicts mid-session, persistent End & review. Every submit resolves
// eagerly in the backend (Tier 0 instant/offline; later tiers async), so
// leaving at any moment loses nothing.

import { useCallback, useEffect, useRef, useState } from "react";
import { TopBar, Button } from "../../components";
import {
  isTauri,
  v2EndSession,
  v2StartSession,
  v2SubmitAttempt,
} from "../../lib/tauri";
import type { Screen, V2SessionItem } from "../../types";

interface V2SessionScreenProps {
  unitId: string;
  unitTitle: string;
  go: (screen: Screen) => void;
}

type LoadState = "loading" | "ready" | "empty" | "error";

export function V2SessionScreen({
  unitId,
  unitTitle,
  go,
}: V2SessionScreenProps) {
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [sessionId, setSessionId] = useState<string | null>(null);
  const [items, setItems] = useState<V2SessionItem[]>([]);
  const [index, setIndex] = useState(0);
  const [value, setValue] = useState("");
  const [attempted, setAttempted] = useState(0);
  const [ending, setEnding] = useState(false);
  const [endFailed, setEndFailed] = useState(false);
  // Submits fire in the background while the learner keeps typing; ending
  // waits for them so the review reads a complete log.
  const inflight = useRef<Promise<unknown>[]>([]);

  useEffect(() => {
    if (!isTauri()) {
      setLoadState("empty");
      return;
    }
    v2StartSession(unitId)
      .then(({ sessionId, items }) => {
        if (items.length === 0) {
          setLoadState("empty");
        } else {
          setSessionId(sessionId);
          setItems(items);
          setLoadState("ready");
        }
      })
      .catch(() => setLoadState("empty"));
  }, [unitId]);

  const current: V2SessionItem | undefined = items[index];

  const submit = useCallback(() => {
    const answer = value.trim();
    if (!current || !sessionId || answer === "") return;
    inflight.current.push(
      v2SubmitAttempt(sessionId, current.id, answer).catch(() => {}),
    );
    setAttempted((n) => n + 1);
    setValue("");
    setIndex((i) => i + 1);
  }, [value, current, sessionId]);

  const skip = useCallback(() => {
    setValue("");
    setIndex((i) => i + 1);
  }, []);

  const endSession = useCallback(async () => {
    if (ending) return;
    if (attempted === 0 || !sessionId) {
      go({ name: "home" });
      return;
    }
    setEnding(true);
    setEndFailed(false);
    await Promise.allSettled(inflight.current);
    try {
      const attempts = await v2EndSession(sessionId);
      go({ name: "v2Review", attempts, sessionId });
    } catch {
      setEnding(false);
      setEndFailed(true);
    }
  }, [ending, attempted, sessionId, go]);

  if (loadState === "loading") {
    return (
      <div className="app fade-in">
        <TopBar showHome onHome={() => go({ name: "home" })} hasRule />
        <div
          className="container"
          style={{ paddingTop: 80, textAlign: "center" }}
        >
          <p className="muted">Assembling your session…</p>
        </div>
      </div>
    );
  }

  if (loadState === "empty" || loadState === "error") {
    return (
      <div className="app fade-in">
        <TopBar showHome onHome={() => go({ name: "home" })} hasRule />
        <div className="container" style={{ paddingTop: 80 }}>
          <h2 className="serif" style={{ fontWeight: 400, fontSize: 24 }}>
            No exercises ready
          </h2>
          <p className="muted" style={{ marginTop: 8 }}>
            This unit's exercises aren't ready yet. Generate them from the unit
            list and come back.
          </p>
          <div style={{ marginTop: 24 }}>
            <Button variant="secondary" onClick={() => go({ name: "v2Units" })}>
              Back to units
            </Button>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="app fade-in">
      <TopBar
        showHome
        onHome={() => go({ name: "home" })}
        hasRule={false}
        right={
          <span className="muted" style={{ fontSize: 13 }}>
            {attempted} attempted
          </span>
        }
      />

      <div
        className="container"
        style={{ paddingTop: 64, paddingBottom: 120, maxWidth: 640 }}
      >
        <div className="eyebrow" style={{ marginBottom: 24 }}>
          {unitTitle}
        </div>

        {current ? (
          <div>
            <p
              className="serif"
              style={{
                fontSize: 28,
                fontWeight: 400,
                lineHeight: 1.35,
                color: "var(--ink)",
                marginBottom: 24,
              }}
            >
              {current.source}
            </p>
            <input
              key={current.id}
              className="input-bare"
              value={value}
              onChange={(e) => setValue(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === "Enter") submit();
              }}
              placeholder="Type the Spanish…"
              autoFocus
              autoComplete="off"
              autoCorrect="off"
              autoCapitalize="off"
              spellCheck={false}
              style={{
                width: "100%",
                fontSize: 19,
                padding: "10px 0",
                borderBottom: "2px solid var(--rule)",
                borderRadius: 0,
                outline: "none",
                background: "transparent",
                color: "var(--ink)",
                caretColor: "var(--accent)",
              }}
            />
            <div
              style={{
                marginTop: 14,
                display: "flex",
                justifyContent: "space-between",
                alignItems: "center",
              }}
            >
              <span className="muted" style={{ fontSize: 12 }}>
                Enter to submit — feedback comes in the review
              </span>
              <button
                className="text-link"
                style={{ fontSize: 13, color: "var(--ink-3)" }}
                onClick={skip}
              >
                Skip
              </button>
            </div>
          </div>
        ) : (
          <div>
            <p className="serif" style={{ fontSize: 22, fontWeight: 400 }}>
              That's every exercise in this unit.
            </p>
            <p className="muted" style={{ marginTop: 8, fontSize: 14 }}>
              End the session to see how it went.
            </p>
          </div>
        )}

        {endFailed && (
          <p style={{ marginTop: 24, fontSize: 13, color: "var(--bad)" }}>
            Couldn't load the review — your attempts are saved. Try again.
          </p>
        )}
      </div>

      {/* Persistent end-session affordance (user story 8). */}
      <div
        style={{
          position: "fixed",
          bottom: 28,
          right: 32,
        }}
      >
        <Button
          variant={current ? "secondary" : "primary"}
          onClick={endSession}
          disabled={ending}
        >
          {ending ? "Ending…" : "End & review"}
        </Button>
      </div>
    </div>
  );
}
