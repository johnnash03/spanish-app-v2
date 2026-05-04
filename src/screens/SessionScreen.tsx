import { useCallback, useEffect, useRef, useState } from "react";
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
  const [cursor, setCursor] = useState(0);
  const [answer, setAnswer] = useState("");
  const [attempts, setAttempts] = useState<LocalAttempt[]>([]);
  const inputRef = useRef<HTMLInputElement>(null);

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

  // Auto-focus input whenever cursor advances
  useEffect(() => {
    if (loadState === "ready") {
      inputRef.current?.focus();
    }
  }, [loadState, cursor]);

  const currentItem = queue[cursor] as SessionItem | undefined;
  const isLastItem = cursor === queue.length - 1;
  const progressPct = queue.length > 0 ? (cursor / queue.length) * 100 : 0;

  const endSession = useCallback(
    (finalAttempts: LocalAttempt[]) => {
      go({ name: "sessionReview", attempts: finalAttempts });
    },
    [go],
  );

  function handleSubmit() {
    if (!currentItem || answer.trim() === "") return;

    const newAttempt: LocalAttempt = {
      itemId: currentItem.id,
      tag: currentItem.primaryTag,
      learnerAnswer: answer.trim(),
      source: currentItem.source,
    };
    const updated = [...attempts, newAttempt];
    setAttempts(updated);
    setAnswer("");

    if (isLastItem) {
      endSession(updated);
    } else {
      setCursor((c) => c + 1);
    }
  }

  function handleKeyDown(e: React.KeyboardEvent<HTMLInputElement>) {
    if (e.key === "Enter") {
      e.preventDefault();
      handleSubmit();
    } else if (e.key === "Escape") {
      e.preventDefault();
      endSession(attempts);
    }
  }

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

  // ─── Active session ────────────────────────────────────────────────────────
  return (
    <div className="app fade-in">
      <TopBar
        showHome
        onHome={() => endSession(attempts)}
        hasRule={false}
        right={
          <button
            className="text-link"
            style={{ fontSize: 13, color: "var(--ink-3)" }}
            onClick={() => endSession(attempts)}
          >
            End session
          </button>
        }
      />

      {/* Progress bar */}
      <div
        style={{
          height: 2,
          background: "var(--rule-soft)",
          position: "relative",
        }}
      >
        <div
          style={{
            position: "absolute",
            left: 0,
            top: 0,
            height: "100%",
            width: `${progressPct}%`,
            background: "var(--accent)",
            transition: "width 0.25s ease",
          }}
        />
      </div>

      <div
        className="container"
        style={{
          paddingTop: 64,
          paddingBottom: 80,
          maxWidth: 640,
          display: "flex",
          flexDirection: "column",
          gap: 0,
        }}
      >
        {/* Item counter */}
        <div
          className="eyebrow"
          style={{ marginBottom: 32, color: "var(--ink-3)" }}
        >
          {cursor + 1} of {queue.length}
        </div>

        {/* Prompt */}
        <div style={{ marginBottom: 40 }}>
          <p
            style={{
              fontSize: 13,
              color: "var(--ink-3)",
              marginBottom: 10,
              textTransform: "uppercase",
              letterSpacing: "0.06em",
              fontWeight: 500,
            }}
          >
            Translate to Spanish
          </p>
          <p
            className="serif"
            style={{
              fontSize: 28,
              fontWeight: 400,
              lineHeight: 1.35,
              letterSpacing: "-0.01em",
              color: "var(--ink)",
            }}
          >
            {currentItem?.source}
          </p>
        </div>

        {/* Answer input */}
        <div style={{ position: "relative" }}>
          <input
            ref={inputRef}
            className="input-bare"
            value={answer}
            onChange={(e) => setAnswer(e.target.value)}
            onKeyDown={handleKeyDown}
            placeholder="Type your answer…"
            autoComplete="off"
            autoCorrect="off"
            autoCapitalize="off"
            spellCheck={false}
            style={{
              width: "100%",
              fontSize: 20,
              padding: "14px 0",
              borderBottom: "2px solid var(--rule)",
              borderRadius: 0,
              outline: "none",
              background: "transparent",
              color: "var(--ink)",
              caretColor: "var(--accent)",
            }}
          />
        </div>

        {/* Submit button + hint */}
        <div
          style={{
            marginTop: 28,
            display: "flex",
            alignItems: "center",
            gap: 16,
          }}
        >
          <Button
            variant="primary"
            size="lg"
            disabled={answer.trim() === ""}
            onClick={handleSubmit}
          >
            {isLastItem ? "Finish" : "Next"}
          </Button>
          <span style={{ fontSize: 12, color: "var(--ink-4)" }}>
            Enter to submit · Esc to end
          </span>
        </div>
      </div>
    </div>
  );
}
