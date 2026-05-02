import { Button } from "../components";
import { TopBar } from "../components/TopBar";
import type { LocalAttempt, Screen } from "../types";

interface SessionReviewScreenProps {
  attempts: LocalAttempt[];
  go: (screen: Screen) => void;
}

export function SessionReviewScreen({
  attempts,
  go,
}: SessionReviewScreenProps) {
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
            : `${count} item${count === 1 ? "" : "s"} answered`}
        </h1>
        <p className="muted" style={{ marginTop: 8, fontSize: 14 }}>
          Your answers have been recorded. Keep practicing to build mastery.
        </p>

        {count > 0 && (
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
