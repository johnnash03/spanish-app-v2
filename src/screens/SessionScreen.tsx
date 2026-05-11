// TEMPORARY — bulk view for manual testing. Revert to cursor-based design when done.
import { useCallback, useEffect, useState } from "react";
import { TopBar, Button } from "../components";
import { assembleSessionQueue, isTauri } from "../lib/tauri";
import type { LocalAttempt, Screen, SessionItem } from "../types";

interface SessionScreenProps {
  unitSkillTag: string;
  go: (screen: Screen) => void;
}

type LoadState = "loading" | "ready" | "empty" | "error";

export function SessionScreen({ unitSkillTag, go }: SessionScreenProps) {
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [queue, setQueue] = useState<SessionItem[]>([]);
  const [answers, setAnswers] = useState<Record<string, string>>({});

  useEffect(() => {
    if (!isTauri()) {
      setLoadState("empty");
      return;
    }
    assembleSessionQueue(unitSkillTag)
      .then((items) => {
        if (items.length === 0) {
          setLoadState("empty");
        } else {
          setQueue(items);
          setLoadState("ready");
        }
      })
      .catch(() => setLoadState("error"));
  }, [unitSkillTag]);

  const endSession = useCallback(() => {
    const attempts: LocalAttempt[] = queue
      .filter((item) => (answers[item.id] ?? "").trim() !== "")
      .map((item) => ({
        itemId: item.id,
        tag: item.primaryTag,
        learnerAnswer: answers[item.id].trim(),
        source: item.source,
      }));
    go({ name: "sessionReview", attempts });
  }, [queue, answers, go]);

  // ─── Loading ───────────────────────────────────────────────────────────────
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

  if (loadState === "empty") {
    return (
      <div className="app fade-in">
        <TopBar showHome onHome={() => go({ name: "home" })} hasRule />
        <div className="container" style={{ paddingTop: 80 }}>
          <h2 className="serif" style={{ fontWeight: 400, fontSize: 24 }}>
            No exercises yet
          </h2>
          <p className="muted" style={{ marginTop: 8 }}>
            Exercises for this unit are still being generated. Come back in a
            moment.
          </p>
          <div style={{ marginTop: 24 }}>
            <Button variant="secondary" onClick={() => go({ name: "home" })}>
              Back to home
            </Button>
          </div>
        </div>
      </div>
    );
  }

  if (loadState === "error") {
    return (
      <div className="app fade-in">
        <TopBar showHome onHome={() => go({ name: "home" })} hasRule />
        <div className="container" style={{ paddingTop: 80 }}>
          <p className="muted">Couldn't load the session. Please try again.</p>
          <div style={{ marginTop: 16 }}>
            <Button variant="secondary" onClick={() => go({ name: "home" })}>
              Back to home
            </Button>
          </div>
        </div>
      </div>
    );
  }

  // ─── Bulk view ─────────────────────────────────────────────────────────────
  const filledCount = queue.filter(
    (item) => (answers[item.id] ?? "").trim() !== "",
  ).length;

  return (
    <div className="app fade-in">
      <TopBar
        showHome
        onHome={() => go({ name: "home" })}
        hasRule={false}
        right={
          <button
            className="text-link"
            style={{ fontSize: 13, color: "var(--ink-3)" }}
            onClick={() => go({ name: "home" })}
          >
            Cancel
          </button>
        }
      />

      <div
        className="container"
        style={{ paddingTop: 48, paddingBottom: 100, maxWidth: 640 }}
      >
        <div className="eyebrow" style={{ marginBottom: 8 }}>
          {queue.length} exercise{queue.length === 1 ? "" : "s"}
        </div>
        <p className="muted" style={{ fontSize: 13, marginBottom: 32 }}>
          Fill in as many as you like, then tap End session.
        </p>

        <div style={{ display: "flex", flexDirection: "column", gap: 32 }}>
          {queue.map((item) => (
            <div key={item.id}>
              <p
                style={{
                  fontSize: 13,
                  color: "var(--ink-3)",
                  marginBottom: 6,
                  textTransform: "uppercase",
                  letterSpacing: "0.06em",
                  fontWeight: 500,
                }}
              >
                {item.primaryTag}
              </p>
              <p
                className="serif"
                style={{
                  fontSize: 20,
                  fontWeight: 400,
                  lineHeight: 1.35,
                  color: "var(--ink)",
                  marginBottom: 10,
                }}
              >
                {item.source}
              </p>
              <input
                className="input-bare"
                value={answers[item.id] ?? ""}
                onChange={(e) =>
                  setAnswers((prev) => ({
                    ...prev,
                    [item.id]: e.target.value,
                  }))
                }
                placeholder="Type your answer…"
                autoComplete="off"
                autoCorrect="off"
                autoCapitalize="off"
                spellCheck={false}
                style={{
                  width: "100%",
                  fontSize: 17,
                  padding: "10px 0",
                  borderBottom: "2px solid var(--rule)",
                  borderRadius: 0,
                  outline: "none",
                  background: "transparent",
                  color: "var(--ink)",
                  caretColor: "var(--accent)",
                }}
              />
            </div>
          ))}
        </div>

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
            disabled={filledCount === 0}
            onClick={endSession}
          >
            End session{filledCount > 0 ? ` (${filledCount})` : ""}
          </Button>
          <Button variant="secondary" onClick={() => go({ name: "home" })}>
            Cancel
          </Button>
        </div>
      </div>
    </div>
  );
}
