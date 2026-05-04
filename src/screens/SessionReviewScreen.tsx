import { useEffect, useRef, useState } from "react";
import { Button } from "../components";
import { TopBar } from "../components/TopBar";
import { evaluateSession, isTauri } from "../lib/tauri";
import type {
  EvalSessionResponse,
  EvaluationResult,
  LocalAttempt,
  Screen,
} from "../types";

interface SessionReviewScreenProps {
  attempts: LocalAttempt[];
  go: (screen: Screen) => void;
}

type EvalState = "loading" | "done" | "failed";

export function SessionReviewScreen({
  attempts,
  go,
}: SessionReviewScreenProps) {
  const [evalState, setEvalState] = useState<EvalState>("loading");
  const [sessionId, setSessionId] = useState<string | null>(null);
  const [results, setResults] = useState<EvaluationResult[]>([]);
  const [showReassurance, setShowReassurance] = useState(false);
  const reassuranceTimer = useRef<ReturnType<typeof setTimeout> | null>(null);

  function runEval(sid: string | null) {
    setEvalState("loading");
    setShowReassurance(false);

    reassuranceTimer.current = setTimeout(
      () => setShowReassurance(true),
      10_000,
    );

    if (!isTauri() || attempts.length === 0) {
      clearTimeout(reassuranceTimer.current);
      setEvalState("done");
      return;
    }

    evaluateSession(sid, attempts)
      .then((resp: EvalSessionResponse) => {
        clearTimeout(reassuranceTimer.current!);
        setSessionId(resp.sessionId);
        setResults(resp.results);
        setEvalState("done");
      })
      .catch(() => {
        clearTimeout(reassuranceTimer.current!);
        setEvalState("failed");
      });
  }

  useEffect(() => {
    runEval(null);
    return () => {
      if (reassuranceTimer.current) clearTimeout(reassuranceTimer.current);
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  // ─── Loading state ─────────────────────────────────────────────────────────
  if (evalState === "loading") {
    return (
      <div className="app fade-in">
        <TopBar showHome onHome={() => go({ name: "home" })} hasRule />
        <div
          className="container"
          style={{ paddingTop: 80, maxWidth: 640, textAlign: "center" }}
        >
          <p className="muted">Evaluating your answers…</p>
          {showReassurance && (
            <p
              className="muted"
              style={{ marginTop: 12, fontSize: 13, color: "var(--ink-4)" }}
            >
              This is taking a moment. Hang tight — your answers are saved.
            </p>
          )}
        </div>
      </div>
    );
  }

  // ─── Failure state ─────────────────────────────────────────────────────────
  if (evalState === "failed") {
    return (
      <div className="app fade-in">
        <TopBar showHome onHome={() => go({ name: "home" })} hasRule />
        <div className="container" style={{ paddingTop: 80, maxWidth: 640 }}>
          <h2
            className="serif"
            style={{ fontSize: 24, fontWeight: 400, marginBottom: 10 }}
          >
            Couldn't reach the evaluator
          </h2>
          <p className="muted" style={{ fontSize: 14 }}>
            We couldn't reach the evaluator. Your answers are saved — try again
            in a moment.
          </p>
          <div style={{ marginTop: 28, display: "flex", gap: 12 }}>
            <Button variant="primary" onClick={() => runEval(sessionId)}>
              Retry evaluation
            </Button>
            <Button variant="secondary" onClick={() => go({ name: "home" })}>
              Back to home
            </Button>
          </div>
        </div>
      </div>
    );
  }

  // ─── Results state ─────────────────────────────────────────────────────────
  const resultMap = new Map(results.map((r) => [r.itemId, r]));
  const correctCount = results.filter((r) => r.correct).length;
  const wrongItems = attempts.filter((a) => {
    const r = resultMap.get(a.itemId);
    return r && !r.correct;
  });
  const correctItems = attempts.filter((a) => {
    const r = resultMap.get(a.itemId);
    return r && r.correct;
  });
  const count = attempts.length;

  return (
    <div className="app fade-in">
      <TopBar showHome onHome={() => go({ name: "home" })} hasRule />

      <div
        className="container"
        style={{ paddingTop: 48, paddingBottom: 80, maxWidth: 640 }}
      >
        <div className="eyebrow" style={{ marginBottom: 10 }}>
          Session complete
        </div>
        <h1
          className="serif"
          style={{ fontSize: 30, fontWeight: 400, letterSpacing: "-0.015em" }}
        >
          {count === 0
            ? "No items answered"
            : results.length > 0
              ? `${correctCount} of ${count} correct`
              : `${count} item${count === 1 ? "" : "s"} answered`}
        </h1>
        {count > 0 && results.length > 0 && (
          <p className="muted" style={{ marginTop: 8, fontSize: 14 }}>
            {correctCount === count
              ? "Perfect session. Keep it up."
              : "Review the items below to reinforce the rules."}
          </p>
        )}

        {/* Wrong items — detailed */}
        {wrongItems.length > 0 && (
          <div style={{ marginTop: 36 }}>
            <div className="eyebrow" style={{ marginBottom: 14 }}>
              To review
            </div>
            <div
              style={{
                border: "1px solid var(--rule-soft)",
                borderRadius: "var(--r-lg)",
                overflow: "hidden",
              }}
            >
              {wrongItems.map((a, i) => {
                const r = resultMap.get(a.itemId)!;
                return (
                  <div
                    key={i}
                    style={{
                      padding: "16px 20px",
                      borderBottom:
                        i < wrongItems.length - 1
                          ? "1px solid var(--rule-soft)"
                          : "none",
                      background: "var(--paper-2)",
                    }}
                  >
                    <p
                      style={{
                        fontSize: 13,
                        color: "var(--ink-3)",
                        marginBottom: 4,
                      }}
                    >
                      {a.source}
                    </p>
                    <p
                      className="serif"
                      style={{
                        fontSize: 15,
                        color: "var(--ink-3)",
                        fontStyle: "italic",
                        textDecoration: "line-through",
                        marginBottom: 4,
                      }}
                    >
                      {a.learnerAnswer}
                    </p>
                    {r.explanation && (
                      <p
                        style={{
                          fontSize: 13,
                          color: "var(--ink-2)",
                          marginTop: 6,
                          lineHeight: 1.5,
                        }}
                      >
                        {r.explanation}
                      </p>
                    )}
                    {r.remarks.map((rem, ri) => (
                      <p
                        key={ri}
                        style={{
                          fontSize: 12,
                          color: "var(--ink-3)",
                          marginTop: 4,
                          fontStyle: "italic",
                        }}
                      >
                        {rem}
                      </p>
                    ))}
                  </div>
                );
              })}
            </div>
          </div>
        )}

        {/* Correct items — compact */}
        {correctItems.length > 0 && (
          <div style={{ marginTop: 28 }}>
            <div className="eyebrow" style={{ marginBottom: 14 }}>
              Correct
            </div>
            <div
              style={{
                border: "1px solid var(--rule-soft)",
                borderRadius: "var(--r-lg)",
                overflow: "hidden",
              }}
            >
              {correctItems.map((a, i) => {
                const r = resultMap.get(a.itemId);
                return (
                  <div
                    key={i}
                    style={{
                      padding: "12px 20px",
                      borderBottom:
                        i < correctItems.length - 1
                          ? "1px solid var(--rule-soft)"
                          : "none",
                      background: "var(--paper-2)",
                    }}
                  >
                    <p
                      style={{
                        fontSize: 13,
                        color: "var(--ink-3)",
                        marginBottom: 2,
                      }}
                    >
                      {a.source}
                    </p>
                    <p
                      className="serif"
                      style={{
                        fontSize: 15,
                        color: "var(--ink)",
                        fontStyle: "italic",
                      }}
                    >
                      {a.learnerAnswer}
                    </p>
                    {r?.remarks.map((rem, ri) => (
                      <p
                        key={ri}
                        style={{
                          fontSize: 12,
                          color: "var(--ink-3)",
                          marginTop: 4,
                          fontStyle: "italic",
                        }}
                      >
                        {rem}
                      </p>
                    ))}
                  </div>
                );
              })}
            </div>
          </div>
        )}

        {/* No-results fallback (non-Tauri env) */}
        {count > 0 && results.length === 0 && (
          <div style={{ marginTop: 36 }}>
            <div className="eyebrow" style={{ marginBottom: 14 }}>
              What you answered
            </div>
            <div
              style={{
                border: "1px solid var(--rule-soft)",
                borderRadius: "var(--r-lg)",
                overflow: "hidden",
              }}
            >
              {attempts.map((a, i) => (
                <div
                  key={i}
                  style={{
                    padding: "14px 20px",
                    borderBottom:
                      i < attempts.length - 1
                        ? "1px solid var(--rule-soft)"
                        : "none",
                    background: "var(--paper-2)",
                  }}
                >
                  <p
                    style={{
                      fontSize: 14,
                      color: "var(--ink-2)",
                      marginBottom: 4,
                    }}
                  >
                    {a.source}
                  </p>
                  <p
                    className="serif"
                    style={{
                      fontSize: 16,
                      color: "var(--ink)",
                      fontStyle: "italic",
                    }}
                  >
                    {a.learnerAnswer}
                  </p>
                </div>
              ))}
            </div>
          </div>
        )}

        <div style={{ marginTop: 32, display: "flex", gap: 12 }}>
          <Button variant="primary" onClick={() => go({ name: "home" })}>
            Back to home
          </Button>
          <Button variant="secondary" onClick={() => go({ name: "units" })}>
            Browse units
          </Button>
        </div>
      </div>
    </div>
  );
}
