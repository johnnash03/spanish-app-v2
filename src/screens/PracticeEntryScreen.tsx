import { useEffect, useState } from "react";
import { TopBar, Button } from "../components";
import { LEARNER } from "../data/mockData";
import { getWeakTags, isTauri } from "../lib/tauri";
import type { Screen, WeakTag } from "../types";

interface PracticeEntryScreenProps {
  go: (screen: Screen) => void;
}

export function PracticeEntryScreen({ go }: PracticeEntryScreenProps) {
  const [weakTags, setWeakTags] = useState<WeakTag[]>(LEARNER.weakTags);

  useEffect(() => {
    if (!isTauri()) return;
    getWeakTags()
      .then(setWeakTags)
      .catch(() => {});
  }, []);

  function practiceAll() {
    go({ name: "practiceSession", tagId: null, tagName: null });
  }

  function practiceTag(tag: WeakTag) {
    go({ name: "practiceSession", tagId: tag.id, tagName: tag.name });
  }

  return (
    <div className="app fade-in">
      <TopBar showHome onHome={() => go({ name: "home" })} hasRule />

      <div
        className="container"
        style={{ paddingTop: 48, paddingBottom: 80, maxWidth: 640 }}
      >
        <div className="eyebrow" style={{ marginBottom: 10 }}>
          Deliberate practice
        </div>
        <h1
          className="serif"
          style={{ fontSize: 30, fontWeight: 400, letterSpacing: "-0.015em" }}
        >
          Skills to strengthen
        </h1>
        <p className="muted" style={{ marginTop: 8, fontSize: 14 }}>
          These tags had more errors than usual. Drilling them now builds
          lasting accuracy.
        </p>

        <div style={{ marginTop: 32, marginBottom: 28 }}>
          <Button variant="primary" onClick={practiceAll}>
            Practice all weak skills
          </Button>
        </div>

        <div
          style={{
            border: "1px solid var(--rule-soft)",
            borderRadius: "var(--r-lg)",
            overflow: "hidden",
          }}
        >
          {weakTags.map((tag, i) => (
            <div
              key={tag.id}
              style={{
                display: "flex",
                alignItems: "center",
                justifyContent: "space-between",
                padding: "16px 20px",
                borderBottom:
                  i < weakTags.length - 1
                    ? "1px solid var(--rule-soft)"
                    : "none",
                background: "var(--paper-2)",
              }}
            >
              <div>
                <div style={{ fontSize: 15, color: "var(--ink)" }}>
                  {tag.name}
                </div>
                <div
                  className="muted"
                  style={{
                    fontSize: 13,
                    marginTop: 3,
                    display: "flex",
                    alignItems: "center",
                    gap: 6,
                  }}
                >
                  <span
                    style={{
                      width: 6,
                      height: 6,
                      borderRadius: 999,
                      background: "var(--bad)",
                      flexShrink: 0,
                      display: "inline-block",
                    }}
                  />
                  {tag.wrongOf20} of 20 wrong recently
                </div>
              </div>
              <Button variant="secondary" onClick={() => practiceTag(tag)}>
                Practice
              </Button>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
