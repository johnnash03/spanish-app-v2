import { TopBar, Button, Callout } from "../components";
import { IconArrowRight } from "../components/icons";
import {
  getUnitByN,
  hasUnmetPrereqs,
  getMissingPrereqNames,
} from "../data/mockData";
import type { Screen } from "../types";

interface UnitDetailScreenProps {
  unitN: number;
  go: (screen: Screen) => void;
}

export function UnitDetailScreen({ unitN, go }: UnitDetailScreenProps) {
  const unit = getUnitByN(unitN);

  if (!unit) {
    return (
      <div className="app fade-in">
        <TopBar showHome onHome={() => go({ name: "home" })} hasRule />
        <div className="container" style={{ paddingTop: 36 }}>
          <p className="muted">Unit not found.</p>
        </div>
      </div>
    );
  }

  const unmetPrereqs = hasUnmetPrereqs(unitN);
  const missingNames = getMissingPrereqNames(unitN);

  return (
    <div className="app fade-in">
      <TopBar
        showHome
        onHome={() => go({ name: "home" })}
        hasRule
        right={
          <button
            className="text-link"
            style={{ fontSize: 13 }}
            onClick={() => go({ name: "units" })}
          >
            All units
          </button>
        }
      />

      <div
        className="container"
        style={{ paddingTop: 36, paddingBottom: 80, maxWidth: 680 }}
      >
        <div className="eyebrow" style={{ marginBottom: 8 }}>
          Phase {unit.phase} · Unit {String(unit.n).padStart(2, "0")}
        </div>
        <h1
          className="serif"
          style={{ fontSize: 32, fontWeight: 400, letterSpacing: "-0.015em" }}
        >
          {unit.name}
        </h1>
        <p
          style={{
            marginTop: 12,
            fontSize: 16,
            lineHeight: 1.6,
            color: "var(--ink-2)",
            maxWidth: 560,
          }}
        >
          {unit.description}
        </p>

        {/* Prerequisite warning */}
        {unmetPrereqs && (
          <div style={{ marginTop: 28 }}>
            <Callout variant="bad">
              <span>
                You haven't completed{" "}
                <em className="serif" style={{ fontStyle: "italic" }}>
                  {missingNames.length > 0
                    ? missingNames.join(", ")
                    : "earlier units"}
                </em>
                . You can practice anyway, or finish those units first.
              </span>
            </Callout>
            <div style={{ marginTop: 10, display: "flex", gap: 12 }}>
              <button
                className="text-link text-link-accent"
                style={{ fontSize: 13 }}
                onClick={() => go({ name: "units" })}
              >
                Browse earlier units <IconArrowRight size={13} />
              </button>
            </div>
          </div>
        )}

        {/* Start practice CTA */}
        <div style={{ marginTop: 40 }}>
          <Button variant="primary" size="lg">
            Start practice
          </Button>
        </div>
      </div>
    </div>
  );
}
