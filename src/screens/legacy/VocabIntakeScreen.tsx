// LEGACY (v1) — Quarantined in S1 (#32); v1 UI stays the default until the v2 home lands (S14, #45), then demotes to a Legacy menu entry. Do not extend. Deleted in S17 (#48).
// Replaced by: S10 Words track intake flow (#41).

import { useEffect, useState } from "react";
import { TopBar, Button } from "../../components";
import {
  isTauri,
  getNextUntouchedWords,
  commitIntakeBatch,
  getPipelineHealth,
} from "../../lib/tauri";
import type {
  Screen,
  VocabWord,
  PipelineHealth,
  PipelineBand,
} from "../../types";

const BATCH_OPTIONS = [3, 5, 10] as const;
type BatchSize = (typeof BATCH_OPTIONS)[number];

const BAND_CONFIG: Record<
  PipelineBand,
  { label: string; message: string; color: string }
> = {
  light: {
    label: "Light",
    message: "Great time to add more words.",
    color: "var(--accent)",
  },
  healthy: {
    label: "Healthy",
    message: "Pipeline is healthy — adding more is fine.",
    color: "var(--accent)",
  },
  full: {
    label: "Full",
    message:
      "Pipeline is getting full — consider reviewing before adding more.",
    color: "#8a6c30",
  },
  overloaded: {
    label: "Overloaded",
    message: "Pipeline is overloaded — focus on reviewing before adding more.",
    color: "var(--bad)",
  },
};

const MOCK_WORDS: VocabWord[] = [
  {
    lemma: "comer",
    translation: "to eat",
    frequencyRank: 142,
    partOfSpeech: "verb",
  },
  {
    lemma: "hablar",
    translation: "to speak",
    frequencyRank: 98,
    partOfSpeech: "verb",
  },
  {
    lemma: "agua",
    translation: "water",
    frequencyRank: 211,
    partOfSpeech: "noun",
  },
  {
    lemma: "grande",
    translation: "big, large",
    frequencyRank: 175,
    partOfSpeech: "adjective",
  },
  {
    lemma: "ciudad",
    translation: "city",
    frequencyRank: 263,
    partOfSpeech: "noun",
  },
];

const MOCK_HEALTH: PipelineHealth = { activeCount: 9, band: "light" };

interface VocabIntakeScreenProps {
  go: (screen: Screen) => void;
}

export function VocabIntakeScreen({ go }: VocabIntakeScreenProps) {
  const [batchSize, setBatchSize] = useState<BatchSize>(5);
  const [words, setWords] = useState<VocabWord[]>([]);
  const [health, setHealth] = useState<PipelineHealth>(MOCK_HEALTH);
  const [cardIndex, setCardIndex] = useState<number | null>(null); // null = config screen
  const [committed, setCommitted] = useState(false);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (!isTauri()) return;
    getPipelineHealth()
      .then(setHealth)
      .catch(() => {});
  }, []);

  function startFlow() {
    if (isTauri()) {
      setLoading(true);
      getNextUntouchedWords(batchSize)
        .then((w) => {
          setWords(w);
          setCardIndex(0);
        })
        .catch(() => {})
        .finally(() => setLoading(false));
    } else {
      setWords(MOCK_WORDS.slice(0, batchSize));
      setCardIndex(0);
    }
  }

  function advance() {
    if (cardIndex === null) return;
    const next = cardIndex + 1;
    if (next >= words.length) {
      // Show confirmation screen
      setCardIndex(words.length);
    } else {
      setCardIndex(next);
      if (isTauri()) {
        // Refresh health live as cards are acknowledged
        getPipelineHealth()
          .then(setHealth)
          .catch(() => {});
      }
    }
  }

  function commit() {
    const lemmas = words.map((w) => w.lemma);
    if (isTauri()) {
      commitIntakeBatch(lemmas)
        .then(() => setCommitted(true))
        .catch(() => {});
    } else {
      setCommitted(true);
    }
  }

  const isConfirmScreen = cardIndex !== null && cardIndex >= words.length;
  const isCardScreen = cardIndex !== null && cardIndex < words.length;
  const currentWord = isCardScreen ? words[cardIndex!] : null;

  // Done screen
  if (committed) {
    return (
      <div className="app fade-in">
        <TopBar showHome onHome={() => go({ name: "home" })} hasRule />
        <div
          className="container"
          style={{
            paddingTop: 80,
            paddingBottom: 80,
            maxWidth: 560,
            display: "flex",
            flexDirection: "column",
            alignItems: "center",
            textAlign: "center",
          }}
        >
          <div className="serif" style={{ fontSize: 28, marginBottom: 12 }}>
            {words.length} words added
          </div>
          <p className="muted" style={{ fontSize: 14, marginBottom: 32 }}>
            They're now in your pipeline and ready to review.
          </p>
          <Button variant="primary" onClick={() => go({ name: "home" })}>
            Back to home
          </Button>
        </div>
      </div>
    );
  }

  // Confirm screen
  if (isConfirmScreen) {
    return (
      <div className="app fade-in">
        <TopBar showHome onHome={() => go({ name: "home" })} hasRule />
        <div
          className="container"
          style={{ paddingTop: 48, paddingBottom: 80, maxWidth: 560 }}
        >
          <PipelineAdvisory health={health} />
          <div
            className="serif"
            style={{ fontSize: 28, marginTop: 32, marginBottom: 8 }}
          >
            Add {words.length} words to your pipeline?
          </div>
          <p className="muted" style={{ fontSize: 14, marginBottom: 28 }}>
            These words will enter your SRS queue as <strong>new</strong> and
            appear in your next review session.
          </p>
          <div
            style={{
              border: "1px solid var(--rule-soft)",
              borderRadius: "var(--r-lg)",
              overflow: "hidden",
              marginBottom: 32,
            }}
          >
            {words.map((w, i) => (
              <div
                key={w.lemma}
                style={{
                  display: "flex",
                  justifyContent: "space-between",
                  alignItems: "center",
                  padding: "12px 18px",
                  borderTop: i > 0 ? "1px solid var(--rule-soft)" : undefined,
                }}
              >
                <span style={{ fontWeight: 500 }}>{w.lemma}</span>
                <span className="muted" style={{ fontSize: 13 }}>
                  {w.translation}
                </span>
              </div>
            ))}
          </div>
          <div style={{ display: "flex", gap: 12, alignItems: "center" }}>
            <Button variant="primary" onClick={commit}>
              Add to pipeline
            </Button>
            <button className="text-link" onClick={() => go({ name: "home" })}>
              Cancel
            </button>
          </div>
        </div>
      </div>
    );
  }

  // Individual word card screen
  if (isCardScreen && currentWord) {
    const progress = cardIndex! + 1;
    return (
      <div className="app fade-in">
        <TopBar showHome onHome={() => go({ name: "home" })} hasRule />
        <div
          className="container"
          style={{ paddingTop: 32, paddingBottom: 80, maxWidth: 560 }}
        >
          <PipelineAdvisory health={health} />

          {/* Progress */}
          <div
            style={{
              display: "flex",
              alignItems: "center",
              justifyContent: "space-between",
              marginTop: 24,
              marginBottom: 28,
            }}
          >
            <span className="eyebrow">
              Word {progress} of {words.length}
            </span>
            <div
              style={{
                display: "flex",
                gap: 6,
              }}
            >
              {words.map((_, i) => (
                <div
                  key={i}
                  style={{
                    width: 24,
                    height: 3,
                    borderRadius: 2,
                    background:
                      i < progress ? "var(--accent)" : "var(--rule-soft)",
                    transition: "background 0.2s",
                  }}
                />
              ))}
            </div>
          </div>

          {/* Word card */}
          <div
            className="card"
            style={{
              padding: "48px 40px",
              display: "flex",
              flexDirection: "column",
              alignItems: "center",
              textAlign: "center",
              gap: 12,
              marginBottom: 28,
            }}
          >
            <div
              className="serif"
              style={{
                fontSize: 48,
                letterSpacing: "-0.02em",
                lineHeight: 1.1,
              }}
            >
              {currentWord.lemma}
            </div>
            <div
              className="muted"
              style={{ fontSize: 15, fontStyle: "italic", marginTop: 4 }}
            >
              {currentWord.translation}
            </div>
            <div
              style={{
                display: "flex",
                gap: 10,
                marginTop: 16,
              }}
            >
              <span
                style={{
                  fontSize: 12,
                  padding: "3px 10px",
                  borderRadius: 999,
                  background: "var(--paper-2)",
                  color: "var(--ink-3)",
                  textTransform: "capitalize",
                }}
              >
                {currentWord.partOfSpeech}
              </span>
              <span
                style={{
                  fontSize: 12,
                  padding: "3px 10px",
                  borderRadius: 999,
                  background: "var(--paper-2)",
                  color: "var(--ink-3)",
                }}
              >
                #{currentWord.frequencyRank} most common
              </span>
            </div>
          </div>

          <div style={{ display: "flex", justifyContent: "center" }}>
            <Button variant="primary" size="lg" onClick={advance}>
              {progress < words.length ? "Got it" : "Done"}
            </Button>
          </div>
        </div>
      </div>
    );
  }

  // Config screen (initial)
  return (
    <div className="app fade-in">
      <TopBar showHome onHome={() => go({ name: "home" })} hasRule />
      <div
        className="container"
        style={{ paddingTop: 48, paddingBottom: 80, maxWidth: 560 }}
      >
        <PipelineAdvisory health={health} />

        <div style={{ marginTop: 32 }}>
          <div className="eyebrow" style={{ marginBottom: 10 }}>
            Learn new words
          </div>
          <h1
            className="serif"
            style={{ fontSize: 30, fontWeight: 400, letterSpacing: "-0.015em" }}
          >
            How many words today?
          </h1>
          <p
            className="muted"
            style={{ fontSize: 14, marginTop: 8, marginBottom: 32 }}
          >
            You'll be shown each word once, then asked to confirm adding them to
            your review pipeline.
          </p>

          <div style={{ display: "flex", gap: 12, marginBottom: 36 }}>
            {BATCH_OPTIONS.map((n) => (
              <button
                key={n}
                onClick={() => setBatchSize(n)}
                style={{
                  padding: "12px 24px",
                  borderRadius: "var(--r-lg)",
                  border: `1.5px solid ${batchSize === n ? "var(--accent)" : "var(--rule)"}`,
                  background:
                    batchSize === n ? "var(--accent-tint)" : "transparent",
                  color: batchSize === n ? "var(--accent)" : "var(--ink-2)",
                  fontSize: 18,
                  fontWeight: batchSize === n ? 600 : 400,
                  minWidth: 72,
                  transition: "all 0.15s",
                }}
              >
                {n}
              </button>
            ))}
          </div>

          <Button
            variant="primary"
            size="lg"
            onClick={startFlow}
            disabled={loading}
          >
            {loading ? "Loading…" : `Start — ${batchSize} words`}
          </Button>
        </div>
      </div>
    </div>
  );
}

function PipelineAdvisory({ health }: { health: PipelineHealth }) {
  const info = BAND_CONFIG[health.band];
  return (
    <div
      style={{
        display: "flex",
        alignItems: "flex-start",
        gap: 10,
        padding: "10px 14px",
        borderRadius: "var(--r-md)",
        background: "var(--paper-2)",
        border: "1px solid var(--rule-soft)",
        fontSize: 13,
      }}
    >
      <span
        style={{
          width: 7,
          height: 7,
          borderRadius: 999,
          background: info.color,
          flexShrink: 0,
          marginTop: 4,
        }}
      />
      <div>
        <span style={{ color: info.color, fontWeight: 600 }}>{info.label}</span>
        <span className="muted" style={{ marginLeft: 6 }}>
          {health.activeCount} active · {info.message}
        </span>
      </div>
    </div>
  );
}
