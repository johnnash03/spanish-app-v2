// V2 end-of-session review (S6, #37) — the batched review entry. With
// only Tier 0 live, attempts are either deterministically correct (shown
// compactly, with any accent/orthography remark) or pending the Tier 1
// evaluator (S7, #38). S14 brings this screen to its final shape.

import { TopBar, Button } from "../../components";
import type { Screen, V2ReviewAttempt } from "../../types";

interface V2ReviewScreenProps {
  attempts: V2ReviewAttempt[];
  go: (screen: Screen) => void;
}

export function V2ReviewScreen({ attempts, go }: V2ReviewScreenProps) {
  const corrects = attempts.filter((a) => a.status === "correct");
  const pendings = attempts.filter((a) => a.status === "pending");

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
