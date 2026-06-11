// LEGACY (v1) — Quarantined in S1 (#32); v1 UI stays the default until the v2 home lands (S14, #45), then demotes to a Legacy menu entry. Do not extend. Deleted in S17 (#48).
// Removed in v2 with no direct counterpart: flashcard SRS is gone; the only
// flashcard-like surface is the S11 stuck-word warm-up (#42).

import { useCallback, useEffect, useRef, useState } from "react";
import { TopBar, Button } from "../../components";
import {
  getVocabSessionCards,
  recordVocabReview,
  isTauri,
} from "../../lib/tauri";
import type { Screen, SrsCard, VocabCardResult } from "../../types";

const SESSION_LIMIT = 20;

// Shuffle an array in place using Fisher-Yates.
function shuffle<T>(arr: T[]): T[] {
  const a = [...arr];
  for (let i = a.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [a[i], a[j]] = [a[j], a[i]];
  }
  return a;
}

function buildOptions(card: SrsCard): string[] {
  return shuffle([card.translation, ...card.distractors]);
}

// ── Mock data for non-Tauri dev ───────────────────────────────────────────────

const MOCK_CARDS: SrsCard[] = [
  {
    lemma: "comer",
    translation: "to eat",
    frequencyRank: 142,
    partOfSpeech: "verb",
    pipelineState: "new",
    intervalDays: 1,
    repetitions: 0,
    selfRated: false,
    distractors: ["to drink", "to sleep", "to run"],
  },
  {
    lemma: "hablar",
    translation: "to speak",
    frequencyRank: 98,
    partOfSpeech: "verb",
    pipelineState: "learning",
    intervalDays: 7,
    repetitions: 2,
    selfRated: true,
    distractors: [],
  },
  {
    lemma: "agua",
    translation: "water",
    frequencyRank: 211,
    partOfSpeech: "noun",
    pipelineState: "new",
    intervalDays: 1,
    repetitions: 0,
    selfRated: false,
    distractors: ["fire", "earth", "air"],
  },
];

// ── Sub-components ────────────────────────────────────────────────────────────

interface ProgressBarProps {
  current: number;
  total: number;
}

function ProgressBar({ current, total }: ProgressBarProps) {
  return (
    <div style={{ display: "flex", gap: 4 }}>
      {Array.from({ length: total }).map((_, i) => (
        <div
          key={i}
          style={{
            flex: 1,
            height: 3,
            borderRadius: 2,
            background: i < current ? "var(--accent)" : "var(--rule-soft)",
            transition: "background 0.2s",
          }}
        />
      ))}
    </div>
  );
}

interface McCardProps {
  card: SrsCard;
  options: string[];
  onAnswer: (correct: boolean) => void;
}

function McCard({ card, options, onAnswer }: McCardProps) {
  const [selected, setSelected] = useState<string | null>(null);
  const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  const handlePick = useCallback(
    (opt: string) => {
      if (selected !== null) return;
      setSelected(opt);
      const correct = opt === card.translation;
      onAnswer(correct);
      timerRef.current = setTimeout(() => {
        // auto-advance is driven by parent — nothing to do here
      }, 800);
    },
    [selected, card.translation, onAnswer],
  );

  useEffect(
    () => () => {
      if (timerRef.current) clearTimeout(timerRef.current);
    },
    [],
  );

  return (
    <div style={{ display: "flex", flexDirection: "column", gap: 0 }}>
      {/* Word */}
      <div
        className="card"
        style={{
          padding: "40px 32px",
          textAlign: "center",
          marginBottom: 20,
        }}
      >
        <div
          className="serif"
          style={{ fontSize: 44, letterSpacing: "-0.02em", lineHeight: 1.1 }}
        >
          {card.lemma}
        </div>
        <div className="muted" style={{ fontSize: 13, marginTop: 8 }}>
          {card.partOfSpeech}
        </div>
      </div>

      {/* Options */}
      <div style={{ display: "flex", flexDirection: "column", gap: 10 }}>
        {options.map((opt) => {
          const isCorrect = opt === card.translation;
          const isSelected = opt === selected;
          let bg = "transparent";
          let border = "var(--rule)";
          let color = "var(--ink)";
          if (selected !== null) {
            if (isCorrect) {
              bg = "var(--good-tint, #e6f4ea)";
              border = "var(--good, #2e7d32)";
              color = "var(--good, #2e7d32)";
            } else if (isSelected) {
              bg = "var(--bad-tint, #fce8e6)";
              border = "var(--bad, #c62828)";
              color = "var(--bad, #c62828)";
            }
          }
          return (
            <button
              key={opt}
              onClick={() => handlePick(opt)}
              style={{
                width: "100%",
                padding: "14px 18px",
                borderRadius: "var(--r-lg)",
                border: `1.5px solid ${border}`,
                background: bg,
                color,
                fontSize: 16,
                textAlign: "left",
                cursor: selected !== null ? "default" : "pointer",
                transition: "all 0.15s",
                fontWeight:
                  isSelected || (selected !== null && isCorrect) ? 600 : 400,
              }}
            >
              {opt}
            </button>
          );
        })}
      </div>
    </div>
  );
}

type FlipState = "question" | "revealed";

interface SelfRatedCardProps {
  card: SrsCard;
  onAnswer: (correct: boolean) => void;
}

function SelfRatedCard({ card, onAnswer }: SelfRatedCardProps) {
  const [flip, setFlip] = useState<FlipState>("question");

  return (
    <div style={{ display: "flex", flexDirection: "column", gap: 16 }}>
      {/* Word face */}
      <div
        className="card"
        style={{
          padding: "48px 32px",
          textAlign: "center",
          minHeight: 180,
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
          justifyContent: "center",
          gap: 8,
        }}
      >
        <div
          className="serif"
          style={{ fontSize: 44, letterSpacing: "-0.02em", lineHeight: 1.1 }}
        >
          {card.lemma}
        </div>
        <div className="muted" style={{ fontSize: 13 }}>
          {card.partOfSpeech}
        </div>

        {flip === "revealed" && (
          <div
            style={{
              marginTop: 20,
              paddingTop: 20,
              borderTop: "1px solid var(--rule-soft)",
              width: "100%",
              textAlign: "center",
            }}
          >
            <div style={{ fontSize: 22, fontWeight: 500, color: "var(--ink)" }}>
              {card.translation}
            </div>
          </div>
        )}
      </div>

      {flip === "question" ? (
        <div style={{ display: "flex", justifyContent: "center" }}>
          <Button variant="secondary" onClick={() => setFlip("revealed")}>
            Reveal
          </Button>
        </div>
      ) : (
        <div style={{ display: "flex", gap: 10 }}>
          <button
            onClick={() => onAnswer(false)}
            style={{
              flex: 1,
              padding: "12px 8px",
              borderRadius: "var(--r-lg)",
              border: "1.5px solid var(--bad, #c62828)",
              background: "transparent",
              color: "var(--bad, #c62828)",
              fontSize: 15,
              fontWeight: 500,
              cursor: "pointer",
            }}
          >
            Again
          </button>
          <button
            onClick={() => onAnswer(true)}
            style={{
              flex: 1,
              padding: "12px 8px",
              borderRadius: "var(--r-lg)",
              border: "1.5px solid var(--rule)",
              background: "transparent",
              color: "var(--ink)",
              fontSize: 15,
              fontWeight: 500,
              cursor: "pointer",
            }}
          >
            Good
          </button>
          <button
            onClick={() => onAnswer(true)}
            style={{
              flex: 1,
              padding: "12px 8px",
              borderRadius: "var(--r-lg)",
              border: "1.5px solid var(--accent)",
              background: "var(--accent-tint)",
              color: "var(--accent)",
              fontSize: 15,
              fontWeight: 500,
              cursor: "pointer",
            }}
          >
            Easy
          </button>
        </div>
      )}
    </div>
  );
}

// ── Main screen ───────────────────────────────────────────────────────────────

interface VocabSessionScreenProps {
  go: (screen: Screen) => void;
}

type LoadState = "loading" | "ready" | "empty" | "error";

export function VocabSessionScreen({ go }: VocabSessionScreenProps) {
  const [loadState, setLoadState] = useState<LoadState>("loading");
  const [cards, setCards] = useState<SrsCard[]>([]);
  const [cursor, setCursor] = useState(0);
  const [results, setResults] = useState<VocabCardResult[]>([]);
  // Options are stable per card (shuffled once).
  const [optionsMap, setOptionsMap] = useState<Record<string, string[]>>({});
  const advanceTimer = useRef<ReturnType<typeof setTimeout> | null>(null);

  useEffect(() => {
    const source = isTauri()
      ? getVocabSessionCards(SESSION_LIMIT)
      : Promise.resolve(MOCK_CARDS);

    source
      .then((fetched) => {
        if (fetched.length === 0) {
          setLoadState("empty");
          return;
        }
        const opts: Record<string, string[]> = {};
        for (const c of fetched) {
          if (!c.selfRated) opts[c.lemma] = buildOptions(c);
        }
        setCards(fetched);
        setOptionsMap(opts);
        setLoadState("ready");
      })
      .catch(() => setLoadState("error"));

    return () => {
      if (advanceTimer.current) clearTimeout(advanceTimer.current);
    };
  }, []);

  const handleAnswer = useCallback((card: SrsCard, correct: boolean) => {
    // Record SRS update in background; don't block UI on it.
    if (isTauri()) {
      recordVocabReview(card.lemma, correct).catch(() => {});
    }

    setResults((prev) => [...prev, { card, correct }]);

    // Auto-advance after a brief delay (gives instant feedback time to register).
    advanceTimer.current = setTimeout(
      () => {
        setCursor((c) => c + 1);
      },
      card.selfRated ? 0 : 900,
    );
  }, []);

  const endSession = useCallback(() => {
    if (advanceTimer.current) clearTimeout(advanceTimer.current);
    go({ name: "vocabReview", results });
  }, [go, results]);

  // ─── States ───────────────────────────────────────────────────────────────

  if (loadState === "loading") {
    return (
      <div className="app fade-in">
        <TopBar showHome onHome={() => go({ name: "home" })} hasRule />
        <div
          className="container"
          style={{ paddingTop: 80, textAlign: "center" }}
        >
          <p className="muted">Loading your review cards…</p>
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
            Nothing due right now
          </h2>
          <p className="muted" style={{ marginTop: 8 }}>
            All your active words are scheduled for later. Come back when more
            are due, or add new words to your pipeline.
          </p>
          <div style={{ display: "flex", gap: 12, marginTop: 24 }}>
            <Button
              variant="primary"
              onClick={() => go({ name: "vocabIntake" })}
            >
              Learn new words
            </Button>
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
          <p className="muted">Couldn't load cards. Please try again.</p>
          <div style={{ marginTop: 16 }}>
            <Button variant="secondary" onClick={() => go({ name: "home" })}>
              Back to home
            </Button>
          </div>
        </div>
      </div>
    );
  }

  // All cards done — navigate automatically.
  if (cursor >= cards.length) {
    // Use a stable render while the navigation fires.
    return (
      <div className="app fade-in">
        <TopBar showHome onHome={() => go({ name: "home" })} hasRule />
        <div
          className="container"
          style={{ paddingTop: 80, textAlign: "center" }}
        >
          <p className="muted">Finishing up…</p>
          <EndSessionEffect go={go} results={results} />
        </div>
      </div>
    );
  }

  const card = cards[cursor];
  const reviewedCount = results.length;

  return (
    <div className="app fade-in">
      <TopBar
        showHome
        onHome={() => go({ name: "home" })}
        hasRule={false}
        right={
          reviewedCount > 0 ? (
            <button
              className="text-link"
              style={{ fontSize: 13, color: "var(--ink-3)" }}
              onClick={endSession}
            >
              End &amp; review
            </button>
          ) : (
            <button
              className="text-link"
              style={{ fontSize: 13, color: "var(--ink-3)" }}
              onClick={() => go({ name: "home" })}
            >
              Cancel
            </button>
          )
        }
      />

      <div
        className="container"
        style={{ paddingTop: 24, paddingBottom: 100, maxWidth: 560 }}
      >
        {/* Header */}
        <div
          style={{
            display: "flex",
            alignItems: "center",
            justifyContent: "space-between",
            marginBottom: 16,
          }}
        >
          <span className="eyebrow">
            {cursor + 1} of {cards.length}
          </span>
          <span className="muted" style={{ fontSize: 12 }}>
            {card.selfRated ? "Recall" : "Multiple choice"}
          </span>
        </div>

        <div style={{ marginBottom: 20 }}>
          <ProgressBar current={cursor} total={cards.length} />
        </div>

        {/* Card */}
        {card.selfRated ? (
          <SelfRatedCard
            key={card.lemma}
            card={card}
            onAnswer={(correct) => handleAnswer(card, correct)}
          />
        ) : (
          <McCard
            key={card.lemma}
            card={card}
            options={optionsMap[card.lemma] ?? buildOptions(card)}
            onAnswer={(correct) => handleAnswer(card, correct)}
          />
        )}
      </div>
    </div>
  );
}

// Tiny helper that fires navigation on mount (avoids setState-during-render).
function EndSessionEffect({
  go,
  results,
}: {
  go: (s: Screen) => void;
  results: VocabCardResult[];
}) {
  const fired = useRef(false);
  useEffect(() => {
    if (fired.current) return;
    fired.current = true;
    go({ name: "vocabReview", results });
  }, [go, results]);
  return null;
}
