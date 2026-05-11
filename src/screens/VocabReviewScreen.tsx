import { Button } from "../components";
import { TopBar } from "../components/TopBar";
import type { Screen, VocabCardResult } from "../types";

interface VocabReviewScreenProps {
  results: VocabCardResult[];
  go: (screen: Screen) => void;
}

export function VocabReviewScreen({ results, go }: VocabReviewScreenProps) {
  const correctCount = results.filter((r) => r.correct).length;
  const incorrectCount = results.length - correctCount;
  const wrong = results.filter((r) => !r.correct);
  const correct = results.filter((r) => r.correct);

  return (
    <div className="app fade-in">
      <TopBar showHome onHome={() => go({ name: "home" })} hasRule />

      <div
        className="container"
        style={{ paddingTop: 40, paddingBottom: 80, maxWidth: 560 }}
      >
        {/* Summary */}
        <div className="eyebrow" style={{ marginBottom: 10 }}>
          Session complete
        </div>
        <h1
          className="serif"
          style={{ fontSize: 30, fontWeight: 400, letterSpacing: "-0.015em" }}
        >
          {results.length === 0
            ? "Nothing reviewed"
            : correctCount === results.length
              ? "Perfect session"
              : `${correctCount} of ${results.length} correct`}
        </h1>

        {results.length > 0 && (
          <div
            style={{
              display: "flex",
              gap: 16,
              marginTop: 16,
              marginBottom: 32,
            }}
          >
            <StatChip
              value={correctCount}
              label="correct"
              color="var(--good, #2e7d32)"
            />
            {incorrectCount > 0 && (
              <StatChip
                value={incorrectCount}
                label="again"
                color="var(--bad, #c62828)"
              />
            )}
          </div>
        )}

        {/* Wrong cards — expanded */}
        {wrong.length > 0 && (
          <section style={{ marginBottom: 28 }}>
            <div
              className="eyebrow"
              style={{ marginBottom: 10, color: "var(--bad, #c62828)" }}
            >
              Review these
            </div>
            <div
              style={{
                border: "1px solid var(--rule-soft)",
                borderRadius: "var(--r-lg)",
                overflow: "hidden",
              }}
            >
              {wrong.map((r, i) => (
                <div
                  key={r.card.lemma}
                  style={{
                    display: "grid",
                    gridTemplateColumns: "1fr 1fr",
                    padding: "12px 18px",
                    borderTop: i > 0 ? "1px solid var(--rule-soft)" : undefined,
                    alignItems: "center",
                  }}
                >
                  <span style={{ fontWeight: 600 }}>{r.card.lemma}</span>
                  <span className="muted" style={{ fontSize: 14 }}>
                    {r.card.translation}
                  </span>
                </div>
              ))}
            </div>
          </section>
        )}

        {/* Correct cards — compact */}
        {correct.length > 0 && (
          <section style={{ marginBottom: 36 }}>
            <div className="eyebrow" style={{ marginBottom: 10 }}>
              Got right
            </div>
            <div style={{ display: "flex", flexWrap: "wrap", gap: 8 }}>
              {correct.map((r) => (
                <span
                  key={r.card.lemma}
                  style={{
                    fontSize: 13,
                    padding: "4px 12px",
                    borderRadius: 999,
                    background: "var(--paper-2)",
                    color: "var(--ink-2)",
                    border: "1px solid var(--rule-soft)",
                  }}
                >
                  {r.card.lemma}
                </span>
              ))}
            </div>
          </section>
        )}

        <div style={{ display: "flex", gap: 12 }}>
          <Button variant="primary" onClick={() => go({ name: "home" })}>
            Back to home
          </Button>
          {results.length > 0 && (
            <Button
              variant="secondary"
              onClick={() => go({ name: "vocabSession" })}
            >
              Keep reviewing
            </Button>
          )}
        </div>
      </div>
    </div>
  );
}

function StatChip({
  value,
  label,
  color,
}: {
  value: number;
  label: string;
  color: string;
}) {
  return (
    <div
      style={{
        display: "flex",
        alignItems: "baseline",
        gap: 4,
      }}
    >
      <span style={{ fontSize: 28, fontWeight: 700, color }}>{value}</span>
      <span className="muted" style={{ fontSize: 14 }}>
        {label}
      </span>
    </div>
  );
}
