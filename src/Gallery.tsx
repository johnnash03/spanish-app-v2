import { useState } from "react";
import {
  Button,
  BadgeCount,
  Card,
  Callout,
  Chip,
  Drawer,
  InputBare,
  SearchInput,
  ListRow,
  Pill,
  StatusDot,
  StateBadge,
  TopBar,
  IconArrowRight,
  IconCheck,
  IconSpark,
  IconX,
  IconLayers,
  IconCards,
  IconNotebook,
  IconChevronDown,
} from "./components";

function Section({
  title,
  children,
}: {
  title: string;
  children: React.ReactNode;
}) {
  return (
    <section style={{ marginBottom: 64 }}>
      <div className="eyebrow" style={{ marginBottom: 20 }}>
        {title}
      </div>
      {children}
    </section>
  );
}

function Row({
  children,
  gap = 12,
}: {
  children: React.ReactNode;
  gap?: number;
}) {
  return (
    <div
      style={{ display: "flex", flexWrap: "wrap", alignItems: "center", gap }}
    >
      {children}
    </div>
  );
}

function Swatch({ name, variable }: { name: string; variable: string }) {
  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
        gap: 6,
      }}
    >
      <div
        style={{
          width: 56,
          height: 56,
          borderRadius: "var(--r-md)",
          background: `var(${variable})`,
          border: "1px solid var(--rule)",
        }}
      />
      <span
        className="mono"
        style={{ fontSize: 10, color: "var(--ink-3)", textAlign: "center" }}
      >
        {name}
      </span>
    </div>
  );
}

export function Gallery() {
  const [drawerOpen, setDrawerOpen] = useState(false);
  const [activeChip, setActiveChip] = useState("All");
  const chips = ["All", "New", "Learning", "Mastered", "Untouched"];

  return (
    <div className="app fade-in">
      <TopBar />

      <div
        className="container"
        style={{ paddingTop: 40, paddingBottom: 80, maxWidth: 900 }}
      >
        <div style={{ marginBottom: 48 }}>
          <h1
            className="serif"
            style={{ fontSize: 36, fontWeight: 400, letterSpacing: "-0.015em" }}
          >
            Component gallery
          </h1>
          <p className="muted" style={{ marginTop: 8, fontSize: 14 }}>
            Design system verification — all tokens and components from the
            handoff.
          </p>
        </div>

        {/* ── Color tokens ── */}
        <Section title="Color tokens">
          <div style={{ display: "flex", flexWrap: "wrap", gap: 16 }}>
            <Swatch name="--paper" variable="--paper" />
            <Swatch name="--paper-2" variable="--paper-2" />
            <Swatch name="--paper-3" variable="--paper-3" />
            <Swatch name="--rule" variable="--rule" />
            <Swatch name="--ink" variable="--ink" />
            <Swatch name="--ink-2" variable="--ink-2" />
            <Swatch name="--ink-3" variable="--ink-3" />
            <Swatch name="--ink-4" variable="--ink-4" />
            <Swatch name="--accent" variable="--accent" />
            <Swatch name="--accent-2" variable="--accent-2" />
            <Swatch name="--accent-soft" variable="--accent-soft" />
            <Swatch name="--accent-tint" variable="--accent-tint" />
            <Swatch name="--bad" variable="--bad" />
            <Swatch name="--bad-soft" variable="--bad-soft" />
          </div>
        </Section>

        {/* ── Typography ── */}
        <Section title="Typography">
          <div style={{ display: "flex", flexDirection: "column", gap: 20 }}>
            <div>
              <div
                className="eyebrow"
                style={{ marginBottom: 6, fontSize: 10 }}
              >
                Serif · Lora · content
              </div>
              <p
                className="serif"
                style={{
                  fontSize: 32,
                  fontWeight: 400,
                  letterSpacing: "-0.015em",
                  margin: 0,
                  lineHeight: 1.3,
                }}
              >
                Buenas tardes.
              </p>
            </div>
            <div>
              <div
                className="eyebrow"
                style={{ marginBottom: 6, fontSize: 10 }}
              >
                Sans · Manrope · UI
              </div>
              <p style={{ fontSize: 15, margin: 0 }}>
                Three tracks. Pick where to put your attention today.
              </p>
            </div>
            <div>
              <div
                className="eyebrow"
                style={{ marginBottom: 6, fontSize: 10 }}
              >
                Mono · JetBrains Mono
              </div>
              <p
                className="mono"
                style={{
                  fontSize: 13,
                  margin: 0,
                  background: "var(--paper-2)",
                  padding: "4px 8px",
                  borderRadius: "var(--r-sm)",
                  display: "inline-block",
                }}
              >
                U07 · #247
              </p>
            </div>
            <div>
              <div
                className="eyebrow"
                style={{ marginBottom: 6, fontSize: 10 }}
              >
                Cue · serif large
              </div>
              <p className="cue" style={{ margin: 0 }}>
                I walked to the café yesterday.
              </p>
            </div>
            <Row gap={20}>
              <span className="eyebrow">Eyebrow label</span>
              <span className="muted">Muted text</span>
              <span className="muted-2">More muted</span>
              <span className="counter">42 reviewed</span>
            </Row>
          </div>
        </Section>

        {/* ── Buttons ── */}
        <Section title="Buttons">
          <div style={{ display: "flex", flexDirection: "column", gap: 16 }}>
            <Row>
              <Button variant="primary">Primary</Button>
              <Button variant="accent">
                Accent <BadgeCount>12</BadgeCount>
              </Button>
              <Button variant="secondary">Secondary</Button>
              <Button variant="ghost">Ghost</Button>
              <Button variant="disabled" disabled>
                Disabled
              </Button>
            </Row>
            <Row>
              <Button variant="primary" size="sm">
                Small
              </Button>
              <Button variant="secondary" size="sm">
                Small secondary
              </Button>
              <Button variant="primary" size="lg">
                Large <IconArrowRight size={16} />
              </Button>
              <Button variant="accent" size="lg">
                Large accent
              </Button>
            </Row>
            <Row>
              <button className="text-link">Text link</button>
              <button className="text-link text-link-accent">
                Accent link
              </button>
            </Row>
          </div>
        </Section>

        {/* ── Cards ── */}
        <Section title="Cards">
          <div
            style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: 16 }}
          >
            <Card>
              <div className="row-between">
                <span className="eyebrow">Grammar</span>
                <IconLayers size={18} stroke={1.4} />
              </div>
              <div style={{ marginTop: 16 }} className="serif">
                Default card with content
              </div>
            </Card>
            <Card locked>
              <div className="row-between">
                <span className="eyebrow">Locked</span>
              </div>
              <div style={{ marginTop: 16 }} className="serif">
                Locked / gated card
              </div>
            </Card>
          </div>
        </Section>

        {/* ── Pills & badges ── */}
        <Section title="Pills & badges">
          <Row>
            <Pill>Default</Pill>
            <Pill variant="accent">Accent · current</Pill>
            <Pill variant="ghost">Ghost</Pill>
            <Pill>verb · regular -ar</Pill>
          </Row>
        </Section>

        {/* ── Status indicators ── */}
        <Section title="Status indicators">
          <div style={{ display: "flex", flexDirection: "column", gap: 16 }}>
            <Row gap={24}>
              <div style={{ display: "flex", alignItems: "center", gap: 8 }}>
                <StatusDot status="default" />
                <span style={{ fontSize: 13 }} className="muted">
                  Not started
                </span>
              </div>
              <div style={{ display: "flex", alignItems: "center", gap: 8 }}>
                <StatusDot status="in-progress" />
                <span style={{ fontSize: 13 }} className="muted">
                  In progress
                </span>
              </div>
              <div style={{ display: "flex", alignItems: "center", gap: 8 }}>
                <StatusDot status="complete" />
                <span style={{ fontSize: 13 }} className="muted">
                  Complete
                </span>
              </div>
            </Row>
            <Row gap={20}>
              <StateBadge state="untouched" />
              <StateBadge state="new" />
              <StateBadge state="learning" />
              <StateBadge state="mastered" />
            </Row>
          </div>
        </Section>

        {/* ── Chips ── */}
        <Section title="Filter chips">
          <Row>
            {chips.map((c) => (
              <Chip
                key={c}
                active={activeChip === c}
                onClick={() => setActiveChip(c)}
              >
                {c}
              </Chip>
            ))}
          </Row>
        </Section>

        {/* ── Inputs ── */}
        <Section title="Input fields">
          <div
            style={{
              display: "flex",
              flexDirection: "column",
              gap: 24,
              maxWidth: 480,
            }}
          >
            <div>
              <div
                className="eyebrow"
                style={{ marginBottom: 8, fontSize: 10 }}
              >
                Bare input · translate answer
              </div>
              <InputBare placeholder="Type your translation, press Enter" />
            </div>
            <div>
              <div
                className="eyebrow"
                style={{ marginBottom: 8, fontSize: 10 }}
              >
                Search input
              </div>
              <SearchInput placeholder="Search lemma or translation" />
            </div>
          </div>
        </Section>

        {/* ── List rows ── */}
        <Section title="List rows">
          <div
            style={{
              border: "1px solid var(--rule-soft)",
              borderRadius: "var(--r-md)",
              overflow: "hidden",
            }}
          >
            {[
              {
                rank: "#1",
                lemma: "el",
                en: "the (m.)",
                state: "mastered" as const,
              },
              {
                rank: "#87",
                lemma: "libro",
                en: "book",
                state: "mastered" as const,
              },
              {
                rank: "#213",
                lemma: "caminar",
                en: "to walk",
                state: "learning" as const,
              },
              {
                rank: "#358",
                lemma: "tardar",
                en: "to take (time)",
                state: "new" as const,
              },
            ].map((w) => (
              <ListRow
                key={w.lemma}
                columns="70px 1fr 1.4fr 120px"
                onClick={() => {}}
              >
                <span className="muted mono" style={{ fontSize: 12 }}>
                  {w.rank}
                </span>
                <span className="serif" style={{ fontSize: 16 }}>
                  {w.lemma}
                </span>
                <span className="muted" style={{ fontSize: 14 }}>
                  {w.en}
                </span>
                <span style={{ textAlign: "right" }}>
                  <StateBadge state={w.state} />
                </span>
              </ListRow>
            ))}
          </div>
        </Section>

        {/* ── Callouts / banners ── */}
        <Section title="Callouts & banners">
          <div
            style={{
              display: "flex",
              flexDirection: "column",
              gap: 12,
              maxWidth: 600,
            }}
          >
            <Callout
              variant="accent"
              icon={<IconSpark size={16} stroke={1.6} />}
            >
              You mastered{" "}
              <em className="serif" style={{ fontStyle: "italic" }}>
                Preterite — regular -ar verbs
              </em>
            </Callout>
            <Callout variant="bad" icon={<IconX size={16} />}>
              3 items need review · Ser vs Estar
            </Callout>
            <Callout variant="accent" icon={<IconCheck size={16} />}>
              Pipeline healthy · room for 3 more words
            </Callout>
          </div>
        </Section>

        {/* ── TopBar ── */}
        <Section title="TopBar">
          <div
            style={{
              border: "1px solid var(--rule-soft)",
              borderRadius: "var(--r-md)",
              overflow: "hidden",
            }}
          >
            <TopBar />
          </div>
          <div
            style={{
              border: "1px solid var(--rule-soft)",
              borderRadius: "var(--r-md)",
              overflow: "hidden",
              marginTop: 12,
            }}
          >
            <TopBar
              showHome
              hasRule
              right={
                <>
                  <span className="counter">7 attempted</span>
                  <button className="icon-btn">
                    <IconNotebook />
                  </button>
                </>
              }
            />
          </div>
        </Section>

        {/* ── Drawer ── */}
        <Section title="Drawer / modal">
          <Button variant="secondary" onClick={() => setDrawerOpen(true)}>
            Open drawer <IconChevronDown size={14} />
          </Button>
          <Drawer
            open={drawerOpen}
            onClose={() => setDrawerOpen(false)}
            wide
            header={
              <div className="row-between">
                <div>
                  <div className="eyebrow">Unit 7 · Notes</div>
                  <div
                    className="serif"
                    style={{
                      fontSize: 18,
                      marginTop: 2,
                      letterSpacing: "-0.01em",
                    }}
                  >
                    Preterite — regular verbs
                  </div>
                </div>
                <button
                  className="icon-btn"
                  onClick={() => setDrawerOpen(false)}
                  aria-label="Close"
                >
                  <IconX />
                </button>
              </div>
            }
          >
            <p
              className="serif"
              style={{ fontSize: 16, lineHeight: 1.6, color: "var(--ink-2)" }}
            >
              The preterite is Spanish's "snapshot" past tense. Use it for
              actions you can pin to a moment — <em>Cené a las ocho</em>,{" "}
              <em>Llegamos ayer</em>.
            </p>
          </Drawer>
        </Section>

        {/* ── Misc ── */}
        <Section title="Miscellaneous">
          <div style={{ display: "flex", flexDirection: "column", gap: 16 }}>
            <div style={{ display: "flex", gap: 16 }}>
              <div className="placeholder" style={{ width: 120, height: 80 }}>
                img
              </div>
              <div style={{ display: "flex", flexDirection: "column", gap: 8 }}>
                <div style={{ display: "flex", alignItems: "center", gap: 8 }}>
                  <IconCards size={18} stroke={1.4} />
                  <span style={{ fontSize: 13 }}>Icon — 18px stroke 1.4</span>
                </div>
                <div style={{ display: "flex", alignItems: "center", gap: 8 }}>
                  <IconSpark
                    size={20}
                    stroke={1.5}
                    style={{ color: "var(--accent)" }}
                  />
                  <span style={{ fontSize: 13 }}>Icon — accent color</span>
                </div>
                <hr className="divider" style={{ width: 200 }} />
                <span className="muted" style={{ fontSize: 12 }}>
                  divider
                </span>
              </div>
            </div>
            <div>
              <span className="serif wrong-answer" style={{ fontSize: 16 }}>
                Ella comió cena tarde anoche.
              </span>
              <div className="muted" style={{ fontSize: 12, marginTop: 4 }}>
                wrong-answer strikethrough
              </div>
            </div>
          </div>
        </Section>
      </div>
    </div>
  );
}
