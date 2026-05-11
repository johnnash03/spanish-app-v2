// Screens — Part 2: Deliberate practice, Vocab bank, Word detail, Learn flow, Flashcards, Summary

const { useState: useState2 } = React;

// ---------- Deliberate practice list ----------
function DeliberateScreen({ go }) {
  const tags = MOCK.learner.weakTags;
  return (
    <div className="app fade-in">
      <TopBar showHome onHome={() => go('home')} hasRule />

      <div className="container-narrow" style={{ paddingTop: 36, paddingBottom: 80, maxWidth: 720 }}>
        <button onClick={() => go('home')} className="text-link" style={{ display: 'inline-flex', alignItems: 'center', gap: 6, border: 'none', padding: 0 }}>
          <IconArrowLeft size={14} /> Home
        </button>

        <div style={{ marginTop: 28 }}>
          <div className="eyebrow">Grammar</div>
          <h1 className="serif" style={{ fontSize: 32, fontWeight: 400, letterSpacing: '-0.015em', marginTop: 6 }}>
            Deliberate practice
          </h1>
          <p className="serif muted" style={{ fontSize: 16, marginTop: 12, lineHeight: 1.5, maxWidth: 540 }}>
            These skills look weak based on your last 20 attempts. Drill one, or sweep all three.
          </p>
        </div>

        <div style={{ marginTop: 32, border: '1px solid var(--rule-soft)', borderRadius: 'var(--r-md)', overflow: 'hidden' }}>
          {tags.map((t, i) => (
            <div key={t.id} style={{
              display: 'grid', gridTemplateColumns: '1fr auto auto',
              gap: 20, alignItems: 'center',
              padding: '18px 20px',
              borderTop: i === 0 ? 'none' : '1px solid var(--rule-soft)',
              background: i === 0 ? '#FBF9F3' : 'transparent',
            }}>
              <div>
                <div className="serif" style={{ fontSize: 18 }}>{t.name}</div>
                <div className="muted" style={{ fontSize: 13, marginTop: 4 }}>
                  {t.wrongOf20} wrong of last 20
                </div>
              </div>
              {/* mini bar */}
              <div style={{ width: 120 }}>
                <div style={{ height: 4, background: 'var(--rule-soft)', borderRadius: 2 }}>
                  <div style={{ width: `${(t.wrongOf20 / 20) * 100}%`, height: '100%', background: 'var(--bad)', borderRadius: 2 }} />
                </div>
                <div className="muted" style={{ fontSize: 11, marginTop: 4, textAlign: 'right' }}>{Math.round((1 - t.wrongOf20/20) * 100)}% accuracy</div>
              </div>
              <button className="btn btn-secondary btn-sm" onClick={() => go('practice')}>Practice</button>
            </div>
          ))}
        </div>

        <div style={{ marginTop: 28, display: 'flex', gap: 14 }}>
          <button className="btn btn-primary" onClick={() => go('practice')}>
            Practice all weak skills <IconArrowRight size={16} />
          </button>
          <span className="muted" style={{ fontSize: 13, alignSelf: 'center' }}>Tag names visible during this session</span>
        </div>
      </div>
    </div>
  );
}

// ---------- Vocab bank ----------
function VocabBankScreen({ go }) {
  const [filter, setFilter] = useState2('All');
  const [query, setQuery] = useState2('');
  const filters = ['All', 'New', 'Learning', 'Mastered', 'Untouched'];

  const rows = MOCK.vocabBank.filter((w) => {
    if (filter !== 'All' && w.state !== filter.toLowerCase()) return false;
    if (query && !(w.lemma.includes(query) || w.en.includes(query))) return false;
    return true;
  });

  return (
    <div className="app fade-in">
      <TopBar showHome onHome={() => go('home')} hasRule />

      <div className="container" style={{ paddingTop: 32, paddingBottom: 80, maxWidth: 1000 }}>
        <div className="row-between" style={{ marginBottom: 18 }}>
          <div>
            <div className="eyebrow">Vocabulary</div>
            <h1 className="serif" style={{ fontSize: 30, fontWeight: 400, letterSpacing: '-0.015em', marginTop: 4 }}>
              Word bank
            </h1>
          </div>
          <div style={{ textAlign: 'right' }}>
            <div className="serif" style={{ fontSize: 22 }}>247 <span className="muted">/ 2000</span></div>
            <div className="muted" style={{ fontSize: 12 }}>mastered from SUBTLEX-ESP top 2000</div>
          </div>
        </div>

        {/* Search + filters */}
        <div style={{ position: 'relative', marginTop: 8 }}>
          <span style={{ position: 'absolute', left: 14, top: '50%', transform: 'translateY(-50%)', color: 'var(--ink-3)' }}>
            <IconSearch size={16} />
          </span>
          <input
            className="search-input"
            placeholder="Search lemma or translation"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
          />
        </div>

        <div style={{ display: 'flex', gap: 8, marginTop: 16, alignItems: 'center', flexWrap: 'wrap' }}>
          {filters.map((f) => (
            <button key={f} className={`chip${filter === f ? ' active' : ''}`} onClick={() => setFilter(f)}>
              {f}
              {f !== 'All' ? (
                <span style={{ marginLeft: 6, opacity: 0.6 }}>
                  {MOCK.vocabBank.filter(w => w.state === f.toLowerCase()).length}
                </span>
              ) : null}
            </button>
          ))}
          <span style={{ marginLeft: 'auto', display: 'inline-flex', alignItems: 'center', gap: 6 }} className="muted">
            <span style={{ fontSize: 12 }}>Sort:</span>
            <button className="text-link" style={{ fontSize: 13 }}>
              Frequency rank <IconCaret size={12} />
            </button>
          </span>
        </div>

        {/* Header */}
        <div style={{
          display: 'grid', gridTemplateColumns: '70px 1fr 1.4fr 100px 120px',
          gap: 16, alignItems: 'center',
          padding: '14px 16px', marginTop: 22,
          borderBottom: '1px solid var(--rule)',
        }}>
          <span className="eyebrow">Rank</span>
          <span className="eyebrow">Lemma</span>
          <span className="eyebrow">Translation</span>
          <span className="eyebrow">POS</span>
          <span className="eyebrow" style={{ textAlign: 'right' }}>State</span>
        </div>

        {/* Rows */}
        <div>
          {rows.map((w) => (
            <button
              key={w.lemma}
              onClick={() => go('wordDetail')}
              style={{
                display: 'grid', gridTemplateColumns: '70px 1fr 1.4fr 100px 120px',
                gap: 16, alignItems: 'center',
                width: '100%', textAlign: 'left',
                padding: '14px 16px',
                borderBottom: '1px solid var(--rule-soft)',
                transition: 'background 100ms ease',
              }}
              onMouseEnter={(e) => e.currentTarget.style.background = '#FBF9F3'}
              onMouseLeave={(e) => e.currentTarget.style.background = 'transparent'}
            >
              <span className="muted mono" style={{ fontSize: 12 }}>#{w.rank}</span>
              <span className="serif" style={{ fontSize: 18 }}>{w.lemma}</span>
              <span className="muted" style={{ fontSize: 14 }}>{w.en}</span>
              <span className="muted" style={{ fontSize: 12, fontStyle: 'italic' }}>{w.pos}</span>
              <span style={{ textAlign: 'right' }}>
                <span className={`state-badge ${w.state}`}>{w.state}</span>
              </span>
            </button>
          ))}
        </div>
      </div>
    </div>
  );
}

// ---------- Word Detail ----------
function WordDetailScreen({ go }) {
  return (
    <div className="app fade-in">
      <TopBar showHome onHome={() => go('home')} hasRule />

      <div className="container-narrow" style={{ paddingTop: 36, paddingBottom: 80, maxWidth: 680 }}>
        <button onClick={() => go('vocab')} className="text-link" style={{ display: 'inline-flex', alignItems: 'center', gap: 6, border: 'none', padding: 0 }}>
          <IconArrowLeft size={14} /> Word bank
        </button>

        <div style={{ marginTop: 28 }}>
          <div className="muted mono" style={{ fontSize: 12 }}>#247 most common</div>
          <h1 className="serif" style={{ fontSize: 56, fontWeight: 400, letterSpacing: '-0.02em', marginTop: 6, lineHeight: 1.1 }}>
            cenar
          </h1>
          <div style={{ display: 'flex', alignItems: 'baseline', gap: 14, marginTop: 6 }}>
            <span className="serif muted" style={{ fontSize: 22, fontStyle: 'italic' }}>to eat dinner, to have supper</span>
          </div>
          <div style={{ display: 'flex', gap: 12, marginTop: 14 }}>
            <span className="pill">verb · regular -ar</span>
            <span className="state-badge learning">learning</span>
          </div>
        </div>

        {/* History */}
        <div style={{ marginTop: 36, padding: '20px 22px', background: '#FBF9F3', border: '1px solid var(--rule-soft)', borderRadius: 'var(--r-md)' }}>
          <div className="eyebrow" style={{ marginBottom: 12 }}>Review history</div>
          <div style={{ display: 'grid', gridTemplateColumns: 'repeat(4, 1fr)', gap: 24 }}>
            <Stat n="8" label="Reviews" />
            <Stat n="6 / 8" label="Correct" />
            <Stat n="2 days" label="Last seen" />
            <Stat n="Tomorrow" label="Next due" />
          </div>
        </div>

        {/* Examples */}
        <div style={{ marginTop: 40 }}>
          <h2 className="serif" style={{ fontSize: 20, marginBottom: 12 }}>Seen in</h2>
          <p className="muted" style={{ fontSize: 13, marginTop: 0, marginBottom: 18 }}>
            Sentences from your Combined-track sessions that used this word.
          </p>
          {[
            { es: 'Cenamos tarde anoche.', en: 'We ate dinner late last night.', when: '4 days ago' },
            { es: 'Ella cenó con sus padres.', en: 'She had dinner with her parents.', when: '8 days ago' },
            { es: '¿Quieres cenar conmigo?', en: 'Do you want to have dinner with me?', when: '11 days ago' },
          ].map((s, i) => (
            <div key={i} style={{ padding: '14px 0', borderBottom: '1px dotted var(--rule)' }}>
              <div className="serif" style={{ fontSize: 17 }}>{s.es}</div>
              <div style={{ display: 'flex', justifyContent: 'space-between', marginTop: 4 }}>
                <span className="muted" style={{ fontSize: 13 }}>{s.en}</span>
                <span className="muted" style={{ fontSize: 12 }}>{s.when}</span>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}

// ---------- Learn new words flow ----------
function LearnScreen({ go }) {
  const [step, setStep] = useState2('setup'); // setup | card | confirm
  const [count, setCount] = useState2(5);
  const [idx, setIdx] = useState2(0);

  const word = MOCK.newWords[idx % MOCK.newWords.length];
  const afterCount = 17 + idx + 1;

  return (
    <div className="app fade-in">
      <TopBar showHome onHome={() => go('home')} />

      {step === 'setup' && (
        <div style={{ flex: 1, display: 'flex', alignItems: 'center', justifyContent: 'center', minHeight: 'calc(100vh - 70px)' }}>
          <div style={{ maxWidth: 520, width: '100%', padding: '0 28px' }}>
            <div className="eyebrow">Vocabulary · Learn new words</div>
            <h1 className="serif" style={{ fontSize: 36, fontWeight: 400, letterSpacing: '-0.015em', marginTop: 8 }}>
              How many words today?
            </h1>
            <p className="serif muted" style={{ fontSize: 17, marginTop: 12 }}>
              First-encounter exposure — no quiz, just meet them. They'll enter the pipeline when you confirm.
            </p>

            <div style={{ display: 'flex', gap: 10, marginTop: 32 }}>
              {[3, 5, 10].map((n) => (
                <button
                  key={n}
                  onClick={() => setCount(n)}
                  style={{
                    flex: 1, padding: '24px 0',
                    border: `1px solid ${count === n ? 'var(--ink)' : 'var(--rule)'}`,
                    background: count === n ? 'var(--ink)' : 'transparent',
                    color: count === n ? 'var(--paper)' : 'var(--ink)',
                    borderRadius: 'var(--r-md)',
                    fontFamily: 'var(--serif)', fontSize: 28,
                  }}
                >
                  {n}
                </button>
              ))}
            </div>

            <div style={{ marginTop: 24, padding: '14px 18px', background: '#FBF9F3', border: '1px solid var(--rule-soft)', borderRadius: 'var(--r-md)', display: 'flex', alignItems: 'center', gap: 10 }}>
              <span style={{ width: 8, height: 8, borderRadius: 999, background: 'var(--accent)' }} />
              <span style={{ fontSize: 14 }}>You have <strong>17 active words</strong> — Pipeline Healthy</span>
            </div>

            <div style={{ marginTop: 32, display: 'flex', gap: 12 }}>
              <button className="btn btn-primary btn-lg" onClick={() => setStep('card')}>
                Begin <IconArrowRight size={16} />
              </button>
              <button className="btn btn-ghost" onClick={() => go('home')}>Cancel</button>
            </div>
          </div>
        </div>
      )}

      {step === 'card' && (
        <div style={{ flex: 1, display: 'flex', alignItems: 'center', justifyContent: 'center', minHeight: 'calc(100vh - 70px)' }}>
          <div style={{ maxWidth: 520, width: '100%', padding: '0 28px' }}>
            <div className="counter" style={{ marginBottom: 32 }}>{idx + 1} of {count}</div>

            <div className="muted mono" style={{ fontSize: 12 }}>#{word.rank} most common</div>
            <h1 className="serif" style={{ fontSize: 80, fontWeight: 400, letterSpacing: '-0.025em', marginTop: 8, lineHeight: 1 }}>
              {word.lemma}
            </h1>
            <div className="serif" style={{ fontSize: 22, marginTop: 16, color: 'var(--ink-2)' }}>
              {word.en}
            </div>
            <div className="muted" style={{ fontSize: 13, marginTop: 6, fontStyle: 'italic' }}>
              {word.pos}
            </div>

            <div style={{ marginTop: 64, display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
              <span className="muted" style={{ fontSize: 13 }}>
                After this word: <strong style={{ color: 'var(--ink-2)' }}>{afterCount} active</strong>
                <span style={{ marginLeft: 8, color: 'var(--accent)' }}>· Healthy</span>
              </span>
              <button
                className="btn btn-primary"
                onClick={() => {
                  if (idx + 1 >= count) setStep('confirm');
                  else setIdx(i => i + 1);
                }}
              >
                Got it <IconArrowRight size={16} />
              </button>
            </div>

            <div style={{ height: 2, background: 'var(--rule-soft)', borderRadius: 1, marginTop: 28 }}>
              <div style={{ width: `${((idx + 1) / count) * 100}%`, height: '100%', background: 'var(--ink)', borderRadius: 1, transition: 'width 240ms ease' }} />
            </div>
          </div>
        </div>
      )}

      {step === 'confirm' && (
        <div style={{ flex: 1, display: 'flex', alignItems: 'center', justifyContent: 'center', minHeight: 'calc(100vh - 70px)' }}>
          <div style={{ maxWidth: 520, width: '100%', padding: '0 28px' }}>
            <div className="eyebrow">Confirm</div>
            <h1 className="serif" style={{ fontSize: 30, fontWeight: 400, letterSpacing: '-0.015em', marginTop: 6 }}>
              Add these {count} words to your pipeline?
            </h1>

            <ul style={{ marginTop: 24, padding: 0, listStyle: 'none', borderTop: '1px solid var(--rule-soft)' }}>
              {MOCK.newWords.slice(0, count).map((w) => (
                <li key={w.lemma} style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'baseline', padding: '14px 4px', borderBottom: '1px solid var(--rule-soft)' }}>
                  <span className="serif" style={{ fontSize: 19 }}>{w.lemma}</span>
                  <span className="muted" style={{ fontSize: 14 }}>{w.en}</span>
                </li>
              ))}
            </ul>

            <div style={{ marginTop: 32, display: 'flex', gap: 12 }}>
              <button className="btn btn-accent btn-lg" onClick={() => go('home')}>Add to pipeline</button>
              <button className="btn btn-ghost" onClick={() => go('home')}>Cancel</button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}

// ---------- Flashcards ----------
function FlashcardScreen({ go }) {
  const [mode, setMode] = useState2('mc'); // mc | recall
  const [revealed, setRevealed] = useState2(false);
  const [count, setCount] = useState2(7);
  const [picked, setPicked] = useState2(null); // { idx, correct }

  const mc = MOCK.flashcardCurrent;
  const re = MOCK.flashcardCurrentRecall;

  const pick = (i) => {
    const correctIdx = mc.options.indexOf('early');
    setPicked({ idx: i, correct: i === correctIdx });
    setTimeout(() => {
      setPicked(null);
      setCount(c => c + 1);
      setMode(m => m === 'mc' ? 'recall' : 'mc');
    }, 800);
  };

  return (
    <div className="app fade-in">
      <TopBar
        showHome onHome={() => go('home')}
        hideWordmark
        right={
          <>
            <span className="counter">{count} reviewed</span>
            <button className="btn btn-secondary btn-sm" onClick={() => go('vocabSummary')}>End & summary</button>
          </>
        }
      />

      <div style={{ flex: 1, display: 'flex', alignItems: 'center', justifyContent: 'center', minHeight: 'calc(100vh - 70px)', padding: '0 28px' }}>
        {mode === 'mc' ? (
          <div style={{ maxWidth: 520, width: '100%' }}>
            <div className="eyebrow" style={{ textAlign: 'center', color: 'var(--ink-4)' }}>What does this mean?</div>
            <h1 className="serif" style={{ fontSize: 72, fontWeight: 400, letterSpacing: '-0.025em', textAlign: 'center', marginTop: 14, lineHeight: 1 }}>
              {mc.lemma}
            </h1>

            <div style={{ marginTop: 56, display: 'flex', flexDirection: 'column', gap: 10 }}>
              {mc.options.map((opt, i) => {
                let style = {};
                let extra = null;
                if (picked) {
                  if (i === picked.idx && !picked.correct) {
                    style = { borderColor: 'var(--bad)', background: 'var(--bad-soft)', color: 'var(--bad)' };
                    extra = <IconX size={16} />;
                  } else if (opt === 'early') {
                    style = { borderColor: 'var(--accent)', background: 'var(--accent-tint)', color: 'var(--accent-2)' };
                    extra = <IconCheck size={16} />;
                  }
                }
                return (
                  <button
                    key={opt}
                    onClick={() => !picked && pick(i)}
                    style={{
                      display: 'flex', alignItems: 'center', justifyContent: 'space-between',
                      padding: '18px 22px',
                      border: '1px solid var(--rule)',
                      borderRadius: 'var(--r-md)',
                      background: 'transparent', textAlign: 'left',
                      fontFamily: 'var(--serif)', fontSize: 18,
                      transition: 'all 120ms ease',
                      ...style,
                    }}
                    onMouseEnter={(e) => !picked && (e.currentTarget.style.background = '#FBF9F3')}
                    onMouseLeave={(e) => !picked && (e.currentTarget.style.background = 'transparent')}
                  >
                    <span>{opt}</span>
                    {extra}
                  </button>
                );
              })}
            </div>

            <div className="muted" style={{ fontSize: 12, textAlign: 'center', marginTop: 24 }}>
              1, 2, 3, 4 to pick · learning word, multiple choice
            </div>
          </div>
        ) : (
          <div style={{ maxWidth: 520, width: '100%', textAlign: 'center' }}>
            <div className="eyebrow" style={{ color: 'var(--ink-4)' }}>Recall</div>
            <h1 className="serif" style={{ fontSize: 80, fontWeight: 400, letterSpacing: '-0.025em', marginTop: 14, lineHeight: 1 }}>
              {re.lemma}
            </h1>

            {!revealed ? (
              <div style={{ marginTop: 56 }}>
                <button className="btn btn-secondary btn-lg" onClick={() => setRevealed(true)}>
                  Show answer <span className="muted mono" style={{ marginLeft: 8, fontSize: 12 }}>space</span>
                </button>
              </div>
            ) : (
              <div style={{ marginTop: 28 }}>
                <div className="serif" style={{ fontSize: 24, color: 'var(--ink-2)', fontStyle: 'italic' }}>{re.en}</div>
                <div style={{ marginTop: 40, display: 'flex', gap: 10, justifyContent: 'center' }}>
                  <button className="btn btn-secondary btn-lg" onClick={() => { setRevealed(false); setCount(c => c+1); setMode('mc'); }}>
                    Again <span className="muted mono" style={{ marginLeft: 6, fontSize: 12 }}>1</span>
                  </button>
                  <button className="btn btn-primary btn-lg" onClick={() => { setRevealed(false); setCount(c => c+1); setMode('mc'); }}>
                    Good <span style={{ opacity: 0.5, marginLeft: 6, fontSize: 12, fontFamily: 'var(--mono)' }}>2</span>
                  </button>
                  <button className="btn btn-accent btn-lg" onClick={() => { setRevealed(false); setCount(c => c+1); setMode('mc'); }}>
                    Easy <span style={{ opacity: 0.6, marginLeft: 6, fontSize: 12, fontFamily: 'var(--mono)' }}>3</span>
                  </button>
                </div>
              </div>
            )}

            <div className="muted" style={{ fontSize: 12, marginTop: 36 }}>
              mature word, self-rated recall
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

// ---------- Vocab summary ----------
function VocabSummaryScreen({ go }) {
  return (
    <div className="app fade-in">
      <TopBar showHome onHome={() => go('home')} hasRule />

      <div className="container-narrow" style={{ paddingTop: 48, paddingBottom: 80, maxWidth: 640 }}>
        <div className="eyebrow">Session summary</div>
        <h1 className="serif" style={{ fontSize: 56, fontWeight: 400, letterSpacing: '-0.02em', marginTop: 4, lineHeight: 1.1 }}>
          23 reviewed
        </h1>
        <div className="serif" style={{ fontSize: 22, color: 'var(--ink-2)', marginTop: 6, fontStyle: 'italic' }}>
          18 correct on first try
        </div>

        {/* Mastery callouts */}
        <div style={{ marginTop: 32, display: 'flex', flexDirection: 'column', gap: 10 }}>
          {[
            { lemma: 'gato', from: 'learning', to: 'mastered' },
            { lemma: 'temprano', from: 'learning', to: 'mastered' },
          ].map((m) => (
            <div key={m.lemma} style={{
              padding: '14px 18px', background: 'var(--accent-tint)',
              borderRadius: 'var(--r-md)', borderLeft: '2px solid var(--accent)',
              display: 'flex', alignItems: 'center', gap: 10,
            }}>
              <IconSpark size={16} style={{ color: 'var(--accent)' }} />
              <span style={{ color: 'var(--accent-2)', fontSize: 14 }}>
                You moved <em className="serif" style={{ fontStyle: 'italic', fontSize: 16 }}>{m.lemma}</em> to mastered.
              </span>
            </div>
          ))}
        </div>

        {/* Breakdown */}
        <div style={{ marginTop: 36, padding: '20px 22px', background: '#FBF9F3', border: '1px solid var(--rule-soft)', borderRadius: 'var(--r-md)' }}>
          <div className="eyebrow" style={{ marginBottom: 14 }}>Pipeline movement</div>
          <div style={{ display: 'grid', gridTemplateColumns: 'repeat(4, 1fr)', gap: 24 }}>
            <Stat n="2" label="To mastered" />
            <Stat n="3" label="Stayed learning" />
            <Stat n="1" label="Slipped back" />
            <Stat n="17 → 15" label="Active count" />
          </div>
        </div>

        <div style={{ marginTop: 40, display: 'flex', gap: 14, justifyContent: 'flex-end' }}>
          <button className="btn btn-secondary" onClick={() => go('home')}>Done</button>
          <button className="btn btn-primary" onClick={() => go('flashcards')}>
            Practice again <IconArrowRight size={16} />
          </button>
        </div>
      </div>
    </div>
  );
}

Object.assign(window, {
  DeliberateScreen, VocabBankScreen, WordDetailScreen,
  LearnScreen, FlashcardScreen, VocabSummaryScreen,
});
