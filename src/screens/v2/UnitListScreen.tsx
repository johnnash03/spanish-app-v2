// V2 unit picker (S6, #37) — a minimal entry point into the v2 session
// loop while the legacy home is still the default UI. Visiting a unit with
// no bank triggers background generation (S5). The full unit list/detail
// surface lands in S14 (#45).

import { useCallback, useEffect, useRef, useState } from "react";
import { TopBar, Button } from "../../components";
import {
  isTauri,
  v2GenerationState,
  v2ListUnits,
  v2TriggerGeneration,
} from "../../lib/tauri";
import type { Screen, V2Unit } from "../../types";

interface V2UnitListScreenProps {
  go: (screen: Screen) => void;
}

export function V2UnitListScreen({ go }: V2UnitListScreenProps) {
  const [units, setUnits] = useState<V2Unit[] | null>(null);
  const pollTimer = useRef<ReturnType<typeof setInterval> | null>(null);

  const refresh = useCallback(() => {
    if (!isTauri()) {
      setUnits([]);
      return;
    }
    v2ListUnits()
      .then(setUnits)
      .catch(() => setUnits([]));
  }, []);

  useEffect(() => {
    refresh();
    return () => {
      if (pollTimer.current) clearInterval(pollTimer.current);
    };
  }, [refresh]);

  const generate = useCallback(
    async (unitId: string) => {
      await v2TriggerGeneration(unitId).catch(() => {});
      refresh();
      if (pollTimer.current) clearInterval(pollTimer.current);
      pollTimer.current = setInterval(async () => {
        const state = await v2GenerationState(unitId).catch(() => "failed");
        if (state !== "generating") {
          if (pollTimer.current) clearInterval(pollTimer.current);
          refresh();
        }
      }, 3000);
    },
    [refresh],
  );

  return (
    <div className="app fade-in">
      <TopBar showHome onHome={() => go({ name: "home" })} hasRule />

      <div
        className="container"
        style={{ paddingTop: 48, paddingBottom: 80, maxWidth: 640 }}
      >
        <div className="eyebrow" style={{ marginBottom: 8 }}>
          Practice · v2
        </div>
        <p className="muted" style={{ fontSize: 13, marginBottom: 28 }}>
          Pick a unit. Units without exercises generate them in the background
          first.
        </p>

        {units === null ? (
          <p className="muted">Loading units…</p>
        ) : units.length === 0 ? (
          <p className="muted">No v2 curriculum available.</p>
        ) : (
          <div style={{ display: "flex", flexDirection: "column", gap: 14 }}>
            {units.map((u) => (
              <div
                key={u.id}
                style={{
                  display: "flex",
                  alignItems: "center",
                  justifyContent: "space-between",
                  gap: 12,
                  padding: "10px 0",
                  borderBottom: "1px solid var(--rule)",
                }}
              >
                <div>
                  <p style={{ fontSize: 15, color: "var(--ink)" }}>{u.title}</p>
                  <p className="muted" style={{ fontSize: 12, marginTop: 2 }}>
                    Phase {u.phase} ·{" "}
                    {u.bankCount > 0
                      ? `${u.bankCount} exercises`
                      : u.generationState === "generating"
                        ? "generating…"
                        : u.generationState === "failed"
                          ? "generation failed"
                          : "no exercises yet"}
                  </p>
                </div>
                {u.bankCount > 0 ? (
                  <Button
                    variant="primary"
                    size="sm"
                    onClick={() =>
                      go({
                        name: "v2Session",
                        unitId: u.id,
                        unitTitle: u.title,
                      })
                    }
                  >
                    Start
                  </Button>
                ) : (
                  <Button
                    variant="secondary"
                    size="sm"
                    disabled={u.generationState === "generating"}
                    onClick={() => generate(u.id)}
                  >
                    {u.generationState === "generating"
                      ? "Generating…"
                      : u.generationState === "failed"
                        ? "Retry"
                        : "Generate"}
                  </Button>
                )}
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
