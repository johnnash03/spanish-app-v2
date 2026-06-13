// V2 end-of-session review (S6, #37; S7, #38) — the batched review entry.
// Tier 0 corrects arrive resolved; Tier 1 verdicts (wrong with hint +
// explanation, structure dodges with their nudge) land in the background,
// so the screen polls while anything is still pending. Dodges read as
// correct to the learner — the nudge is the only difference (user story
// 15). S14 brings this screen to its final shape.

import { useEffect, useState } from "react";
import { TopBar, Button } from "../../components";
import { isTauri, v2SessionReview } from "../../lib/tauri";
import type { Screen, V2ReviewAttempt } from "../../types";

interface V2ReviewScreenProps {
  attempts: V2ReviewAttempt[];
  sessionId: string;
  go: (screen: Screen) => void;
}

const POLL_MS = 2000;

export function V2ReviewScreen({
  attempts: initial,
  sessionId,
  go,
}: V2ReviewScreenProps) {
  const [attempts, setAttempts] = useState(initial);

  const pendings = attempts.filter((a) => a.status === "pending");

  // Background Tier 1 evaluations resolve after session end; poll until
  // nothing is pending.
  useEffect(() => {
    if (!isTauri() || pendings.length === 0) return;
    const timer = setInterval(() => {
      v2SessionReview(sessionId)
        .then(setAttempts)
        .catch(() => {});
    }, POLL_MS);
    return () => clearInterval(timer);
  }, [sessionId, pendings.length]);

  // The learner reads a dodge as correct; the nudge remark carries the
  // difference.
  const corrects = attempts.filter(
    (a) => a.status === "correct" || a.status === "dodge",
  );
  const wrongs = attempts.filter((a) => a.status === "wrong");

  return (
    <div className="app fade-in">
      <TopBar showHome onHome={() => go({ name: "home" })} hasRule />

      <div
        className="container"
        style={{ paddingTop: 48, paddingBottom: 80, maxWidth: 640 }}
      >
        <h1 className="serif" style={{ fontSize: 30, fontWeight: 400 }}>
          {corrects.length} of {attempts.length} correct
        </h1>
        {pendings.length > 0 && (
          <p className="muted" style={{ marginTop: 6, fontSize: 14 }}>
            {pendings.length} answer{pendings.length === 1 ? "" : "s"} awaiting
            full evaluation.
          </p>
        )}

        {wrongs.length > 0 && (
          <section style={{ marginTop: 36 }}>
            <div className="eyebrow" style={{ marginBottom: 16 }}>
              Needs work · {wrongs.length}
            </div>
            <div style={{ display: "flex", flexDirection: "column", gap: 28 }}>
              {wrongs.map((a, i) => (
                <div key={`${a.itemId}-${i}`}>
                  <p
                    className="muted"
                    style={{ fontSize: 13, marginBottom: 4 }}
                  >
                    {a.source}
                  </p>
                  <p style={{ fontSize: 16, color: "var(--ink)" }}>
                    <span style={{ color: "var(--bad)", marginRight: 8 }}>
                      ✗
                    </span>
                    <span lang="es">{a.answer}</span>
                  </p>
                  <p className="muted" style={{ fontSize: 13, marginTop: 4 }}>
                    Correct: <span lang="es">{a.canonical}</span>
                  </p>
                  {a.hint && (
                    <p
                      style={{
                        fontSize: 14,
                        marginTop: 8,
                        color: "var(--ink)",
                        fontStyle: "italic",
                      }}
                    >
                      {a.hint}
                    </p>
                  )}
                  {a.explanation && (
                    <p className="muted" style={{ fontSize: 13, marginTop: 6 }}>
                      {a.explanation}
                    </p>
                  )}
                </div>
              ))}
            </div>
          </section>
        )}

        {pendings.length > 0 && (
          <section style={{ marginTop: 36 }}>
            <div className="eyebrow" style={{ marginBottom: 16 }}>
              Awaiting evaluation · {pendings.length}
            </div>
            <div style={{ display: "flex", flexDirection: "column", gap: 24 }}>
              {pendings.map((a, i) => (
                <div key={`${a.itemId}-${i}`}>
                  <p
                    className="muted"
                    style={{ fontSize: 13, marginBottom: 4 }}
                  >
                    {a.source}
                  </p>
                  <p style={{ fontSize: 16, color: "var(--ink)" }}>
                    {a.answer}
                  </p>
                  <p className="muted" style={{ fontSize: 13, marginTop: 4 }}>
                    Target: <span lang="es">{a.canonical}</span>
                  </p>
                </div>
              ))}
            </div>
          </section>
        )}

        {corrects.length > 0 && (
          <section style={{ marginTop: 36 }}>
            <div className="eyebrow" style={{ marginBottom: 16 }}>
              Correct · {corrects.length}
            </div>
            <div style={{ display: "flex", flexDirection: "column", gap: 18 }}>
              {corrects.map((a, i) => (
                <div key={`${a.itemId}-${i}`}>
                  <p style={{ fontSize: 15, color: "var(--ink)" }}>
                    <span style={{ color: "var(--accent)", marginRight: 8 }}>
                      ✓
                    </span>
                    <span className="muted" style={{ marginRight: 8 }}>
                      {a.source}
                    </span>
                    <span lang="es">{a.answer}</span>
                  </p>
                  {a.remarks.map((remark, j) => (
                    <p
                      key={j}
                      className="muted"
                      style={{ fontSize: 13, marginTop: 4, marginLeft: 22 }}
                    >
                      {remark}
                    </p>
                  ))}
                </div>
              ))}
            </div>
          </section>
        )}

        <div style={{ marginTop: 48, display: "flex", gap: 16 }}>
          <Button variant="primary" onClick={() => go({ name: "home" })}>
            Done
          </Button>
          <Button variant="secondary" onClick={() => go({ name: "v2Units" })}>
            Back to units
          </Button>
        </div>
      </div>
    </div>
  );
}
