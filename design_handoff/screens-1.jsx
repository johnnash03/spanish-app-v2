// Screens — Part 1: Home, Unit list, Unit detail, Practice, Review

const { useState, useEffect, useRef } = React;

// ---------- TopBar ----------
function TopBar({ onHome, showHome, right, hasRule, hideWordmark }) {
  return (
    <div className={`topbar${hasRule ? ' has-rule' : ''}`}>
      <div className="left">
        {showHome ? (
          <button className="icon-btn" onClick={onHome} aria-label="Home" title="Home">
            <IconHouse />
          </button>
        ) : null}
        {!hideWordmark ? (
          <span className="wordmark">léxico<span className="dot">.</span></span>
        ) : null}
      </div>
      <div className="right">
        {right}
        <button className="icon-btn" aria-label="Shortcuts" title="Shortcuts (?)">
          <IconQuestion />
        </button>
      </div>
    </div>
  );
}

// ---------- Home ----------
function HomeScreen({ go }) {
  const u = MOCK.learner;
  return (
    <div className="app fade-in">
      <TopBar />

      <div className="container" style={{ paddingTop: 28, paddingBottom: 80 }}>
        {/* Continue strip */}
        <button
          onClick={() => go('practice')}
          style={{
            width: '100%',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'space-between',
            background: '#FBF9F3',
            border: '1px solid var(--rule-soft)',
            borderRadius: 'var(--r-md)',
            padding: '14px 18px',
            marginBottom: 28,
            textAlign: 'left',
          }}
        >
          <div style={{ display: 'flex', alignItems: 'center', gap: 12 }}>
            <span className="eyebrow" style={{ marginRight: 4 }}>Continue</span>
            <span style={{ color: 'var(--ink)' }}>{u.continueSession.label}</span>
            <span className="muted" style={{ fontSize: 13 }}>· last practiced 14 minutes ago</span>
          </div>
          <span style={{ display: 'inline-flex', alignItems: 'center', gap: 6, color: 'var(--ink-2)', fontSize: 13 }}>
            Resume <IconArrowRight size={16} />
          </span>
        </button>

        {/* Headline */}
        <div style={{ marginBottom: 36 }}>
          <h1 className="serif" style={{ fontSize: 30, fontWeight: 400, letterSpacing: '-0.015em' }}>
            Buenas tardes.
          </h1>
          <p className="muted" style={{ marginTop: 6, fontSize: 14 }}>
            Three tracks. Pick where to put your attention today.
          </p>
        </div>

        {/* Cards */}
        <div style={{ display: 'grid', gridTemplateColumns: '1.1fr 1fr 0.9fr', gap: 20 }}>
          {/* Grammar card */}
          <div className="card" style={{ display: 'flex', flexDirection: 'column', minHeight: 280 }}>
            <div className="row-between">
              <span className="eyebrow">Grammar</span>
              <IconLayers size={18} stroke={1.4} />
            </div>
            <div style={{ marginTop: 22, flex: 1 }}>
              <div className="serif" style={{ fontSize: 22, lineHeight: 1.25 }}>
                Unit 7
              </div>
              <div className="serif muted" style={{ fontSize: 16, marginTop: 2 }}>
                Preterite — regular verbs
              </div>
              <div className="muted" style={{ fontSize: 13, marginTop: 14 }}>
                {u.currentUnit.toward} of {u.currentUnit.of} toward mastery
              </div>
              {/* slim progress bar */}
              <div style={{ height: 3, background: 'var(--rule-soft)', borderRadius: 2, marginTop: 10 }}>
                <div style={{
                  width: `${(u.currentUnit.toward / u.currentUnit.of) * 100}%`,
                  height: '100%', background: 'var(--accent)', borderRadius: 2,
                }} />
              </div>

              {/* deliberate practice pill */}
              <button
                onClick={() => go('deliberate')}
                style={{
                  marginTop: 18, display: 'inline-flex', alignItems: 'center', gap: 8,
                  padding: '8px 12px', borderRadius: 999,
                  background: 'transparent', border: '1px solid var(--rule)',
                  fontSize: 13, color: 'var(--ink-2)',
                }}
              >
                <span style={{ width: 6, height: 6, borderRadius: 999, background: 'var(--bad)' }} />
                3 skills need review
                <IconArrowRight size={14} />
              </button>
            </div>
            <div style={{ display: 'flex', alignItems: 'center', gap: 16, marginTop: 22 }}>
              <button className="btn btn-primary" onClick={() => go('practice')}>
                Continue Unit 7
              </button>
              <button className="text-link" onClick={() => go('units')}>Browse all units</button>
            </div>
          </div>

          {/* Vocab card */}
          <div className="card" style={{ display: 'flex', flexDirection: 'column', minHeight: 280 }}>
            <div className="row-between">
              <span className="eyebrow">Vocabulary</span>
              <IconCards size={18} stroke={1.4} />
            </div>
            <div style={{ marginTop: 22, flex: 1 }}>
              <div className="serif" style={{ fontSize: 44, lineHeight: 1, letterSpacing: '-0.02em' }}>
                {u.masteredCount}
              </div>
              <div className="muted" style={{ fontSize: 14, marginTop: 4 }}>
                words mastered
              </div>
              <div style={{ marginTop: 18, fontSize: 13, color: 'var(--ink-2)' }}>
                Pipeline {u.pipelineStatus.label.toLowerCase()} · {u.pipelineStatus.detail}
              </div>
              <div className="muted" style={{ fontSize: 12, marginTop: 4 }}>
                {u.learningCount} learning · {u.newCount} new
              </div>
            </div>
            <div style={{ display: 'flex', alignItems: 'center', gap: 12, marginTop: 22 }}>
              <button className="btn btn-accent" onClick={() => go('flashcards')}>
                Review
                <span className="badge-count">{u.dueCount} due</span>
              </button>
              <button className="text-link" onClick={() => go('learn')}>Learn new words</button>
            </div>
          </div>

          {/* Combined card */}
          <div className="card" style={{ display: 'flex', flexDirection: 'column', minHeight: 280 }}>
            <div className="row-between">
              <span className="eyebrow">Combined</span>
              <IconSpark size={18} stroke={1.4} />
            </div>
            <div style={{ marginTop: 22, flex: 1, display: 'flex', flexDirection: 'column', justifyContent: 'center' }}>
              <div className="serif" style={{ fontSize: 22, lineHeight: 1.3, letterSpacing: '-0.01em' }}>
                Grammar &<br/>vocabulary,<br/>woven together.
              </div>
              <div style={{ marginTop: 16, display: 'inline-flex', alignItems: 'center', gap: 8, fontSize: 13, color: 'var(--accent)' }}>
                <span style={{ width: 6, height: 6, borderRadius: 999, background: 'var(--accent)' }} />
                Ready
              </div>
            </div>
            <div style={{ marginTop: 22 }}>
              <button className="btn btn-secondary" onClick={() => go('practice')} style={{ width: '100%' }}>
                Practice
              </button>
            </div>
          </div>
        </div>

        {/* Footnote */}
        <div style={{ marginTop: 60, paddingTop: 20, borderTop: '1px solid var(--rule-soft)', display: 'flex', justifyContent: 'space-between', fontSize: 12 }} className="muted">
          <span>Phase 2 of 4 · {u.masteredCount + 17} total reviews logged</span>
          <span>Press <span className="mono" style={{ background: 'var(--paper-2)', padding: '1px 6px', borderRadius: 3 }}>?</span> for shortcuts</span>
        </div>
      </div>
    </div>
  );
}

// ---------- Unit List ----------
function UnitListScreen({ go }) {
  const [openPhases, setOpenPhases] = useState({ 1: false, 2: true, 3: false, 4: false });
  const u = MOCK.learner;

  return (
    <div className="app fade-in">
      <TopBar showHome onHome={() => go('home')} hasRule />

      <div className="container-narrow" style={{ paddingTop: 36, paddingBottom: 80, maxWidth: 820 }}>
        <div className="eyebrow" style={{ marginBottom: 8 }}>Grammar</div>
        <h1 className="serif" style={{ fontSize: 32, fontWeight: 400, letterSpacing: '-0.015em' }}>
          All units
        </h1>
        <p className="muted" style={{ marginTop: 6, fontSize: 14, maxWidth: 520 }}>
          Four phases, each gated by the one before. Anything is reachable — finish in order, or jump ahead and the unit detail will warn you.
        </p>

        <div style={{ marginTop: 36 }}>
          {MOCK.phases.map((phase) => {
            const open = !!openPhases[phase.number];
            const isCurrent = phase.number === u.currentUnit.phase;
            return (
              <div key={phase.number} style={{ borderTop: '1px solid var(--rule-soft)' }}>
                <button
                  onClick={() => setOpenPhases({ ...openPhases, [phase.number]: !open })}
                  style={{
                    width: '100%', textAlign: 'left',
                    padding: '18px 4px', display: 'flex', alignItems: 'center', gap: 14,
                  }}
                >
                  <span style={{ transform: open ? 'rotate(0deg)' : 'rotate(-90deg)', transition: 'transform 160ms ease', color: 'var(--ink-3)' }}>
                    <IconChevronDown size={16} />
                  </span>
                  <span className="eyebrow" style={{ minWidth: 60 }}>Phase {phase.number}</span>
                  <span className="serif" style={{ fontSize: 18 }}>{phase.name}</span>
                  {isCurrent ? <span className="pill pill-accent" style={{ marginLeft: 'auto' }}>Current</span> :
                    <span className="muted" style={{ marginLeft: 'auto', fontSize: 13 }}>
                      {phase.units.filter(uu => uu.status === 'complete').length} / {phase.units.length}
                    </span>
                  }
                </button>
                {open ? (
                  <div style={{ paddingBottom: 8 }}>
                    {phase.units.map((unit) => (
                      <button
                        key={unit.n}
                        onClick={() => go('unitDetail')}
                        style={{
                          width: '100%', textAlign: 'left',
                          display: 'grid', gridTemplateColumns: '32px 60px 1fr 100px',
                          gap: 14, alignItems: 'center',
                          padding: '12px 18px',
                          borderRadius: 'var(--r-md)',
                          transition: 'background 100ms ease',
                        }}
                        onMouseEnter={(e) => e.currentTarget.style.background = '#FBF9F3'}
                        onMouseLeave={(e) => e.currentTarget.style.background = 'transparent'}
                      >
                        <span className={`status-dot${unit.status === 'in-progress' ? ' in-progress' : unit.status === 'complete' ? ' complete' : ''}`} />
                        <span className="muted mono" style={{ fontSize: 13 }}>U{String(unit.n).padStart(2, '0')}</span>
                        <span className="serif" style={{ fontSize: 17 }}>{unit.name}</span>
                        <span className="muted" style={{ fontSize: 12, textAlign: 'right' }}>
                          {unit.status === 'complete' ? 'Mastered' : unit.status === 'in-progress' ? 'In progress' : 'Not started'}
                        </span>
                      </button>
                    ))}
                  </div>
                ) : null}
              </div>
            );
          })}
          <div style={{ borderTop: '1px solid var(--rule-soft)' }} />
        </div>
      </div>
    </div>
  );
}

// ---------- Unit notes content (shared between Unit Detail and Drawer) ----------
function renderInline(text) {
  // Split on *...* into italic spans (Lora italic)
  const parts = text.split(/(\*[^*]+\*)/g);
  return parts.map((p, i) => {
    if (p.startsWith('*') && p.endsWith('*')) {
      return <em key={i} className="serif" style={{ fontStyle: 'italic' }}>{p.slice(1, -1)}</em>;
    }
    return <React.Fragment key={i}>{p}</React.Fragment>;
  });
}

function NotesBody({ unit, compact }) {
  const fontBody = compact ? 15 : 17;
  const fontH = compact ? 16 : 20;
  return (
    <div>
      {unit.reading.sections.map((s) => (
        <section key={s.id} id={`notes-${s.id}`} style={{ marginBottom: compact ? 28 : 40, scrollMarginTop: 20 }}>
          <h3 className="serif" style={{ fontSize: fontH, fontWeight: 500, marginBottom: 10, letterSpacing: '-0.01em' }}>
            {s.title}
          </h3>
          {s.body ? (
            <p className="serif" style={{ fontSize: fontBody, lineHeight: 1.6, color: 'var(--ink-2)', margin: 0 }}>
              {renderInline(s.body)}
            </p>
          ) : null}

          {s.table ? (
            <div style={{ marginTop: 14, border: '1px solid var(--rule-soft)', borderRadius: 'var(--r-md)', overflow: 'hidden', background: '#FBF9F3' }}>
              <table style={{ width: '100%', borderCollapse: 'collapse', fontSize: compact ? 13 : 14 }}>
                <thead>
                  <tr>
                    {s.table.head.map((h, i) => (
                      <th key={i} style={{
                        textAlign: 'left', padding: compact ? '8px 10px' : '10px 14px',
                        fontFamily: 'var(--sans)', fontWeight: 500,
                        fontSize: 11, textTransform: 'uppercase', letterSpacing: '0.06em',
                        color: 'var(--ink-3)',
                        borderBottom: '1px solid var(--rule)',
                        background: 'var(--paper-2)',
                      }}>{h}</th>
                    ))}
                  </tr>
                </thead>
                <tbody>
                  {s.table.rows.map((row, ri) => (
                    <tr key={ri}>
                      {row.map((cell, ci) => (
                        <td key={ci} style={{
                          padding: compact ? '8px 10px' : '10px 14px',
                          borderBottom: ri < s.table.rows.length - 1 ? '1px dotted var(--rule)' : 'none',
                          fontFamily: ci === 0 ? 'var(--sans)' : 'var(--serif)',
                          fontStyle: ci === 0 ? 'normal' : 'normal',
                          color: ci === 0 ? 'var(--ink-3)' : 'var(--ink)',
                          fontSize: ci === 0 ? (compact ? 12 : 13) : (compact ? 14 : 15),
                        }}>{cell}</td>
                      ))}
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          ) : null}

          {s.examples ? (
            <div style={{ marginTop: 12 }}>
              {s.examples.map((ex, i) => (
                <div key={i} style={{ padding: '10px 0', borderBottom: i < s.examples.length - 1 ? '1px dotted var(--rule)' : 'none' }}>
                  <div className="serif" style={{ fontSize: fontBody }}>{ex.es}</div>
                  <div className="muted" style={{ fontSize: compact ? 12 : 13, marginTop: 2 }}>{ex.en}</div>
                </div>
              ))}
            </div>
          ) : null}

          {s.bullets ? (
            <ul style={{ paddingLeft: 18, marginTop: 10, marginBottom: 0 }}>
              {s.bullets.map((b, i) => (
                <li key={i} className="serif" style={{ fontSize: fontBody, lineHeight: 1.6, color: 'var(--ink-2)', marginBottom: 8 }}>
                  {renderInline(b)}
                </li>
              ))}
            </ul>
          ) : null}
        </section>
      ))}

      {/* Glossary */}
      <section id="notes-glossary" style={{ scrollMarginTop: 20, marginBottom: 8 }}>
        <h3 className="serif" style={{ fontSize: fontH, fontWeight: 500, marginBottom: 12, letterSpacing: '-0.01em' }}>
          Glossary
        </h3>
        <p className="muted" style={{ fontSize: 12, marginTop: -4, marginBottom: 14 }}>
          Vocabulary you may meet in this unit's exercises.
        </p>
        <div style={{ display: 'grid', gridTemplateColumns: compact ? '1fr' : '1fr 1fr', gap: compact ? '8px 0' : '10px 32px' }}>
          {unit.glossary.map((g) => (
            <div key={g.lemma} style={{ display: 'flex', justifyContent: 'space-between', borderBottom: '1px dotted var(--rule)', padding: '8px 0' }}>
              <span className="serif" style={{ fontSize: compact ? 15 : 16 }}>{g.lemma}</span>
              <span className="muted" style={{ fontSize: compact ? 13 : 14 }}>{g.en}</span>
            </div>
          ))}
        </div>
      </section>
    </div>
  );
}

function NotesTOC({ unit, compact, onJump }) {
  const items = [
    ...unit.reading.sections.map(s => ({ id: s.id, label: s.title })),
    { id: 'glossary', label: 'Glossary' },
  ];
  return (
    <div style={{
      display: 'flex', flexWrap: 'wrap', gap: 6,
      paddingBottom: compact ? 14 : 20,
      marginBottom: compact ? 18 : 28,
      borderBottom: '1px solid var(--rule-soft)',
    }}>
      {items.map((it, i) => (
        <button
          key={it.id}
          onClick={() => onJump(it.id)}
          style={{
            display: 'inline-flex', alignItems: 'center',
            height: compact ? 26 : 28,
            padding: '0 10px', borderRadius: 999,
            background: 'transparent', border: '1px solid var(--rule)',
            fontSize: 12, color: 'var(--ink-2)', cursor: 'pointer',
          }}
        >
          {it.label}
        </button>
      ))}
    </div>
  );
}

window.NotesBody = NotesBody;
window.NotesTOC = NotesTOC;

// ---------- Unit Detail ----------
function UnitDetailScreen({ go }) {
  const u = MOCK.currentUnit;

  const jumpTo = (id) => {
    const el = document.getElementById(`notes-${id}`);
    if (el) el.scrollIntoView({ block: 'start' });
  };

  return (
    <div className="app fade-in">
      <TopBar showHome onHome={() => go('home')} hasRule />

      <div className="container-narrow" style={{ paddingTop: 36, paddingBottom: 80, maxWidth: 720 }}>
        <button onClick={() => go('units')} className="text-link" style={{ display: 'inline-flex', alignItems: 'center', gap: 6, border: 'none', padding: 0 }}>
          <IconArrowLeft size={14} /> All units
        </button>

        <div style={{ marginTop: 28 }}>
          <div className="eyebrow">Phase {u.phase} · Unit {u.number}</div>
          <h1 className="serif" style={{ fontSize: 36, fontWeight: 400, letterSpacing: '-0.015em', marginTop: 6 }}>
            {u.name}
          </h1>
          <p className="serif muted" style={{ fontSize: 17, marginTop: 14, lineHeight: 1.5, maxWidth: 600 }}>
            {u.description}
          </p>
        </div>

        {/* Stats row */}
        <div style={{ display: 'flex', gap: 36, marginTop: 28, paddingTop: 24, paddingBottom: 4, borderTop: '1px solid var(--rule-soft)' }}>
          <Stat n="12 / 20" label="Toward mastery" />
          <Stat n="84%" label="Recent accuracy" />
          <Stat n="3" label="Sessions" />
          <Stat n="4 days ago" label="Last practiced" />
        </div>

        {/* Notes / reading */}
        <div style={{ marginTop: 44 }}>
          <div className="row-between" style={{ marginBottom: 18 }}>
            <h2 className="serif" style={{ fontSize: 22, letterSpacing: '-0.01em' }}>Notes</h2>
            <span className="muted" style={{ fontSize: 13 }}>~3 min read</span>
          </div>
          <NotesTOC unit={u} onJump={jumpTo} />
          <NotesBody unit={u} />
        </div>

        <div style={{ marginTop: 32, paddingTop: 24, borderTop: '1px solid var(--rule-soft)', display: 'flex', alignItems: 'center', gap: 16 }}>
          <button className="btn btn-primary btn-lg" onClick={() => go('practice')}>
            Start practice <IconArrowRight size={16} />
          </button>
          <span className="muted" style={{ fontSize: 13 }}>~10–15 min · variable length · notes available mid-session</span>
        </div>
      </div>
    </div>
  );
}

function Stat({ n, label }) {
  return (
    <div>
      <div className="serif" style={{ fontSize: 22, letterSpacing: '-0.01em' }}>{n}</div>
      <div className="muted" style={{ fontSize: 12, marginTop: 2, textTransform: 'uppercase', letterSpacing: '0.06em' }}>{label}</div>
    </div>
  );
}

// ---------- Practice (Grammar) ----------
function PracticeScreen({ go }) {
  const [count, setCount] = useState(7);
  const [val, setVal] = useState('');
  const [drawerOpen, setDrawerOpen] = useState(false);
  const [cueIdx, setCueIdx] = useState(0);
  const cues = [
    'I walked to the café yesterday.',
    'She ate dinner late last night.',
    'We lived in Madrid last week.',
    'They arrived early.',
    'You (tú) drank coffee at home.',
  ];
  const cue = cues[cueIdx % cues.length];

  const submit = () => {
    if (!val.trim()) return;
    setCount(c => c + 1);
    setVal('');
    setCueIdx(i => i + 1);
  };

  return (
    <div className="app fade-in" style={{ background: 'var(--paper)' }}>
      <TopBar
        showHome onHome={() => go('home')}
        hideWordmark
        right={
          <>
            <span className="counter">{count} attempted</span>
            <button className="icon-btn" onClick={() => setDrawerOpen(true)} title="Notes">
              <IconNotebook />
            </button>
          </>
        }
      />

      {/* Practice canvas */}
      <div style={{
        flex: 1, display: 'flex', flexDirection: 'column',
        alignItems: 'center', justifyContent: 'center',
        padding: '40px 28px',
        minHeight: 'calc(100vh - 70px)',
      }}>
        <div style={{ width: '100%', maxWidth: 680 }}>
          <div className="eyebrow" style={{ marginBottom: 14, color: 'var(--ink-4)' }}>Translate to Spanish</div>
          <h1 className="cue">{cue}</h1>

          <div style={{ marginTop: 28 }}>
            <input
              autoFocus
              className="input-bare"
              placeholder="Type your translation, press Enter"
              value={val}
              onChange={(e) => setVal(e.target.value)}
              onKeyDown={(e) => { if (e.key === 'Enter') submit(); }}
            />
            <div style={{ display: 'flex', justifyContent: 'space-between', marginTop: 12 }}>
              <span className="muted" style={{ fontSize: 12 }}>
                Enter to submit · acentos: opt+e then a
              </span>
              <span className="muted" style={{ fontSize: 12 }}>
                Feedback shows at end of session
              </span>
            </div>
          </div>
        </div>
      </div>

      {/* Bottom bar — End & review */}
      <div style={{
        position: 'fixed', bottom: 0, left: 0, right: 0,
        padding: '20px 28px', display: 'flex', justifyContent: 'flex-end',
        gap: 12,
      }}>
        <button className="btn btn-secondary" onClick={() => go('review')}>
          End & review
        </button>
      </div>

      {/* Notes drawer */}
      <div className={`drawer-scrim${drawerOpen ? ' open' : ''}`} onClick={() => setDrawerOpen(false)} />
      <aside className={`drawer drawer-wide${drawerOpen ? ' open' : ''}`}>
        <div className="row-between" style={{ padding: '20px 24px', borderBottom: '1px solid var(--rule-soft)' }}>
          <div>
            <div className="eyebrow">Unit 7 · Notes</div>
            <div className="serif" style={{ fontSize: 18, marginTop: 2, letterSpacing: '-0.01em' }}>
              {MOCK.currentUnit.name}
            </div>
          </div>
          <button className="icon-btn" onClick={() => setDrawerOpen(false)} aria-label="Close">
            <IconX />
          </button>
        </div>
        <div style={{ padding: '20px 24px', overflowY: 'auto' }}>
          <NotesTOC
            unit={MOCK.currentUnit}
            compact
            onJump={(id) => {
              const el = document.getElementById(`notes-${id}`);
              if (el) el.scrollIntoView({ block: 'start', behavior: 'smooth' });
            }}
          />
          <NotesBody unit={MOCK.currentUnit} compact />
        </div>
      </aside>
    </div>
  );
}

// ---------- Review ----------
function ReviewScreen({ go }) {
  const r = MOCK.reviewItems;
  const [correctsOpen, setCorrectsOpen] = useState(false);

  return (
    <div className="app fade-in">
      <TopBar showHome onHome={() => go('home')} hasRule />

      <div className="container-narrow" style={{ paddingTop: 40, paddingBottom: 100, maxWidth: 760 }}>
        {/* Hero */}
        <div className="eyebrow">Session review</div>
        <h1 className="serif" style={{ fontSize: 56, fontWeight: 400, letterSpacing: '-0.02em', marginTop: 4, lineHeight: 1.1 }}>
          {r.correct} <span className="muted">/ {r.total}</span> <span style={{ fontSize: 24, color: 'var(--ink-3)', fontStyle: 'italic' }}>correct</span>
        </h1>

        {/* Mastery callout */}
        <div style={{ marginTop: 24, padding: '14px 18px', background: 'var(--accent-tint)', borderRadius: 'var(--r-md)', borderLeft: '2px solid var(--accent)', display: 'flex', alignItems: 'center', gap: 10 }}>
          <IconSpark size={16} stroke={1.6} style={{ color: 'var(--accent)' }} />
          <span style={{ color: 'var(--accent-2)', fontSize: 14 }}>
            You mastered <em className="serif" style={{ fontStyle: 'italic' }}>Preterite — regular -ar verbs</em>
          </span>
        </div>

        {/* Wrongs */}
        <section style={{ marginTop: 44 }}>
          <div className="row-between" style={{ marginBottom: 18 }}>
            <h2 className="serif" style={{ fontSize: 22 }}>Needs review</h2>
            <span className="muted" style={{ fontSize: 13 }}>{r.actualWrongs.length} items</span>
          </div>

          <div style={{ display: 'flex', flexDirection: 'column', gap: 24 }}>
            {r.actualWrongs.map((w, i) => (
              <article key={i} style={{ padding: '20px 22px', background: '#FBF9F3', border: '1px solid var(--rule-soft)', borderLeft: '2px solid var(--bad)', borderRadius: 'var(--r-md)' }}>
                <div className="muted" style={{ fontSize: 12, textTransform: 'uppercase', letterSpacing: '0.06em' }}>You saw</div>
                <p className="serif" style={{ fontSize: 18, marginTop: 4 }}>{w.en}</p>

                <div style={{ marginTop: 14, display: 'grid', gridTemplateColumns: '70px 1fr', gap: '8px 16px', alignItems: 'baseline' }}>
                  <span className="muted" style={{ fontSize: 12 }}>You</span>
                  <span className="serif wrong-answer" style={{ fontSize: 16 }}>{w.user}</span>
                  <span className="muted" style={{ fontSize: 12 }}>Correct</span>
                  <span className="serif" style={{ fontSize: 16, color: 'var(--accent-2)' }}>{w.correct}</span>
                </div>

                <div style={{ marginTop: 16, paddingTop: 14, borderTop: '1px dotted var(--rule)' }}>
                  <div style={{ fontSize: 13, color: 'var(--ink-2)', marginBottom: 6 }}>
                    <span className="muted" style={{ marginRight: 8 }}>Hint</span>{w.hint}
                  </div>
                  <div style={{ fontSize: 13, color: 'var(--ink-2)', lineHeight: 1.6 }}>
                    <span className="muted" style={{ marginRight: 8 }}>Why</span>{w.explain}
                  </div>
                </div>
              </article>
            ))}
          </div>
        </section>

        {/* Corrects */}
        <section style={{ marginTop: 36 }}>
          <button
            onClick={() => setCorrectsOpen(o => !o)}
            style={{
              width: '100%', textAlign: 'left',
              display: 'flex', alignItems: 'center', gap: 10,
              padding: '14px 16px',
              border: '1px solid var(--rule-soft)',
              borderRadius: 'var(--r-md)',
              background: '#FBF9F3',
            }}
          >
            <IconCheck size={16} style={{ color: 'var(--accent)' }} />
            <span style={{ color: 'var(--ink)', fontSize: 15 }}>{r.correct} correct</span>
            <span className="muted" style={{ fontSize: 13 }}>· tap to {correctsOpen ? 'collapse' : 'expand'}</span>
            <span style={{ marginLeft: 'auto', transform: correctsOpen ? 'rotate(180deg)' : 'rotate(0deg)', transition: 'transform 160ms ease', color: 'var(--ink-3)' }}>
              <IconChevronDown size={16} />
            </span>
          </button>

          {correctsOpen ? (
            <div style={{ padding: '8px 4px', marginTop: 4 }}>
              {r.corrects.map((c, i) => (
                <div key={i} style={{ display: 'grid', gridTemplateColumns: '20px 1fr 1fr 80px', gap: 14, alignItems: 'center', padding: '10px 14px', borderBottom: '1px dotted var(--rule)' }}>
                  <IconCheck size={14} style={{ color: 'var(--accent)' }} />
                  <span style={{ fontSize: 14 }} className="muted">{c.en}</span>
                  <span className="serif" style={{ fontSize: 15 }}>{c.user}</span>
                  <button className="text-link" style={{ fontSize: 12, justifySelf: 'end' }}>explain</button>
                </div>
              ))}
            </div>
          ) : null}
        </section>

        {/* Follow-up */}
        <section style={{ marginTop: 40, padding: '20px 22px', border: '1px solid var(--rule-soft)', borderRadius: 'var(--r-md)' }}>
          <div className="eyebrow" style={{ marginBottom: 6 }}>Follow-up</div>
          <div className="row-between">
            <div>
              <div className="serif" style={{ fontSize: 17 }}>5 items on Ser vs Estar</div>
              <div className="muted" style={{ fontSize: 13, marginTop: 2 }}>You missed three Ser vs Estar items today.</div>
            </div>
            <button className="btn btn-secondary btn-sm">
              Start follow-up <IconArrowRight size={14} />
            </button>
          </div>
        </section>

        {/* Footer CTAs */}
        <div style={{ marginTop: 40, display: 'flex', gap: 14, justifyContent: 'flex-end' }}>
          <button className="btn btn-secondary" onClick={() => go('home')}>Done</button>
          <button className="btn btn-primary" onClick={() => go('practice')}>
            Practice again <IconArrowRight size={16} />
          </button>
        </div>
      </div>
    </div>
  );
}

Object.assign(window, {
  TopBar, HomeScreen, UnitListScreen, UnitDetailScreen, PracticeScreen, ReviewScreen,
});
