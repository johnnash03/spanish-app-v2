// LEGACY (v1) — Quarantined in S1 (#32); v1 UI stays the default until the v2 home lands (S14, #45), then demotes to a Legacy menu entry. Do not extend. Deleted in S17 (#48).
// Removed in v2 with no direct counterpart: the combined track is absorbed
// into all practice (S5 generator #36, S6 session loop #37).

import { useEffect, useRef, useState } from "react";
import { Button } from "../../components";
import { TopBar } from "../../components/TopBar";
import {
  evaluateSession,
  isTauri,
  recordCombinedSessionReviews,
} from "../../lib/tauri";
import type {
  EvalSessionResponse,
  EvaluationResult,
  LocalAttempt,
  Screen,
} from "../../types";

interface CombinedReviewScreenProps {
  attempts: LocalAttempt[];
  vocabLemmasByItemId: Record<string, string[]>;
  go: (screen: Screen) => void;
}

type EvalState = "loading" | "done" | "failed";

export function CombinedReviewScreen({
  attempts,
  vocabLemmasByItemId,
  go,
}: CombinedReviewScreenProps) {
  const [evalState, setEvalState] = useState<EvalState>("loading");
  const [sessionId, setSessionId] = useState<string | null>(null);
  const [results, setResults] = useState<EvaluationResult[]>([]);
  const [showReassurance, setShowReassurance] = useState(false);
  const reassuranceTimer = useRef<ReturnType<typeof setTimeout> | null>(null);
  const evalStarted = useRef(false);

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

        const correctLemmas = resp.results
          .filter((r) => r.correct)
          .flatMap((r) => vocabLemmasByItemId[r.itemId] ?? []);

        if (correctLemmas.length > 0) {
          recordCombinedSessionReviews(correctLemmas).catch(() => {});
        }

        setEvalState("done");
      })
      .catch(() => {
        clearTimeout(reassuranceTimer.current!);
        setEvalState("failed");
      });
  }

  useEffect(() => {
    if (evalStarted.current) return;
    evalStarted.current = true;
    runEval(null);
    return () => {
      if (reassuranceTimer.current) clearTimeout(reassuranceTimer.current);
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

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

  const resultMap = new Map(results.map((r) => [r.itemId, r]));
  const correctCount = results.filter((r) => r.correct).length;
  const count = attempts.length;

  const wrongItems = attempts.filter((a) => {
    const r = resultMap.get(a.itemId);
    return r && !r.correct;
  });
  const correctItems = attempts.filter((a) => {
    const r = resultMap.get(a.itemId);
    return r && r.correct;
  });

  return (
    <div className="app fade-in">
      <TopBar showHome onHome={() => go({ name: "home" })} hasRule />

      <div
        className="container"
        style={{ paddingTop: 48, paddingBottom: 80, maxWidth: 640 }}
      >
        <div className="eyebrow" style={{ marginBottom: 10 }}>
          Combined practice complete
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
                        marginBottom: 2,
                      }}
                    >
                      {a.learnerAnswer}
                    </p>
                    <p
                      className="serif"
                      style={{
                        fontSize: 15,
                        color: "var(--ink)",
                        fontStyle: "italic",
                        marginBottom: 4,
                      }}
                    >
                      {r.canonical}
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

        {/* Correct items — with vocab annotations */}
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
                const lemmas = vocabLemmasByItemId[a.itemId] ?? [];
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
                    {lemmas.length > 0 && (
                      <div
                        style={{
                          marginTop: 6,
                          display: "flex",
                          flexDirection: "column",
                          gap: 2,
                        }}
                      >
                        {lemmas.map((lemma) => (
                          <p
                            key={lemma}
                            style={{
                              fontSize: 12,
                              color: "var(--accent)",
                              margin: 0,
                            }}
                          >
                            ✓ <em>{lemma}</em> advanced in pipeline
                          </p>
                        ))}
                      </div>
                    )}
                  </div>
                );
              })}
            </div>
          </div>
        )}

        <div style={{ marginTop: 32, display: "flex", gap: 12 }}>
          <Button variant="primary" onClick={() => go({ name: "home" })}>
            Back to home
          </Button>
        </div>
      </div>
    </div>
  );
}
