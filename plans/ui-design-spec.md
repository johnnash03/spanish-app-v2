# UI Design Spec — Spanish Learning App

> Self-contained design brief intended to be fed to a high-fidelity design tool (e.g., Claude design). Synthesized from `plans/prd-spanish-learning-app.md` and the `/grill-me` design session held against it. Where this doc and the PRD disagree, the PRD has been amended to match — they should be in sync.

---

## 0. Brand & platform basics

- **Product name:** Working name TBD. Designer may use a placeholder wordmark — single word preferred, set in the content serif, lowercase.
- **Platform:** Tauri desktop (macOS / Windows / Linux), single-user.
- **Default window:** 1280 × 800, resizable. Min: 960 × 600. Optimize the primary practice and review layouts for ~1200px width.
- **Tonal references** (feels like): Readwise Reader, iA Writer, Things 3, Stripe documentation pages.
- **Anti-references** (does *not* feel like): Duolingo (gamified cartoon), Memrise (saturated, busy), Anki (spreadsheet austerity), Babbel (corporate teal).
- **Density example:** on a 1200px window, the practice screen shows only the cue (centered, ~28pt serif), the input (single line, generous height), and minimal top/bottom chrome (counter, Home, Notes, End & review). Nothing else competes for attention.
- **Iconography:** Phosphor icons, regular weight. Linear, thin-stroke, paper-feeling. No filled or duotone variants.

---

## 1. Product context (one paragraph)

A local-first Tauri desktop app for a single learner studying Spanish to fluency. Three independent but interconnected practice tracks: **Grammar** (unit-based progression with interleaved drilling), **Vocabulary** (SRS-based flashcards over the SUBTLEX-ESP top 2000 words), **Combined** (AI-generated exercises that fuse unlocked grammar with active vocab). The app is a content server, not a schedule enforcer — the learner decides when and how much to practice. No streaks, no daily targets, no enforced order. LLM-based answer evaluation handles natural Spanish variation.

---

## 2. Visual design language

| Aspect | Direction |
|---|---|
| **Overall feel** | "Studious calm" — leather notebook + fountain pen, not gamified app |
| **Density** | Spacious. Big type, generous whitespace, one primary action per screen |
| **Typography — content** | Humanist serif (e.g., *Source Serif*, *Lora*) for Spanish cues, English translations, explanations, word lemmas |
| **Typography — UI** | Sans-serif (e.g., *Inter*) for buttons, labels, counters, navigation |
| **Background** | Off-white, paper-like |
| **Text** | Dark warm grey (not pure black) |
| **Accent color** | Single muted accent — fountain-pen-ink feel (muted green or terracotta). *Not* Duolingo green |
| **Correct/wrong feedback** | Muted greens and reds, never bright alarm colors |
| **Mastery callouts** | Use the accent color. No gold confetti, no celebratory animations |
| **Motion** | Subtle. Gentle slides for transitions. No springy bounces, no decorative animation |
| **Microcopy tone** | Warm, plainspoken, second-person. *"Not quite — here's the rule."* *"You moved* libro *to mastered."* No formal stiffness, no chirpy gamification |
| **Dark mode** | Designer's discretion (paper-warm should still feel paper-warm, not pure dark slate) |

---

## 3. Information architecture

**Hub-and-spoke.** Home is the only navigation hub. Inside any track, a small persistent **Home icon** (top-left) returns to home. No persistent sidebar or top tabs.

```
Home
├─ Grammar Track
│   ├─ Unit list (browse all)
│   │   └─ Unit detail
│   │       └─ Practice session → End-of-session review
│   └─ Deliberate practice entry (when weak tags exist)
│       └─ Practice session → End-of-session review
├─ Vocabulary Track
│   ├─ Vocab bank (browse all words)
│   │   └─ Word detail
│   ├─ Learn new words (full-screen intake flow)
│   └─ Flashcard session → End-of-session summary
└─ Combined Track (locked until 10+ active words)
    └─ Practice session → End-of-session review
```

---

## 4. Screen-by-screen specs

### 4.1 Home

A hybrid hub: three track cards as the core layout, with a **Continue** strip above when there's a recent session to resume.

**Continue strip (top, conditional):**
- Hidden on day-zero or when nothing to resume
- Otherwise: small horizontal bar — *"Continue: Phase 2 · Unit 7"* + arrow CTA → resumes the most recent track

**Three track cards (main layout):**

**Grammar card:**
- Current unit name + progress hint: *"Unit 7 · 12/20 toward mastery"*
- Primary CTA: **Continue Unit 7**
- Secondary text-link: **Browse all units**
- Conditional inline pill (only if weak tags exist): *"3 skills need review →"* — entry to deliberate practice
- Day-zero variant: shows *"Phase 1 · Unit 1: [name]"* with **Start** as primary CTA, no deliberate practice pill

**Vocabulary card:**
- Hero number — large, prominent: *"247 words mastered"*
- Primary CTA with badge: **Review (12 due)**. When 0 due, button reads "No reviews due" in disabled style (still tappable, routes to bank)
- Secondary CTA: **Learn new words**, with pipeline-health microcopy directly below: *"Pipeline healthy · room for 14 more"* (or *"Pipeline filling up"*, *"Pipeline is light · add some words to begin"*, etc.)
- Small tertiary line: *"6 learning · 3 new"* for quick pipeline visibility
- Day-zero variant: hero shows "0", "No reviews due" disabled, primary action effectively becomes "Learn new words", health line: *"Pipeline is light · add some words to begin"*

**Combined card (always visible):**
- **Locked state** (under 10 active words): dimmed/greyed card, message: *"Unlocks when you have 10 words in your pipeline. You have 4."* Tap routes to vocab "Learn new words" flow.
- **Unlocked state**: minimal — primary CTA **Practice**, small *"Ready"* indicator. Do not expose pool size.

**Pending-session banner** (above the cards, when applicable):
- *"You have an unsubmitted session — review now"* with a **Review** CTA. User opt-in; no auto-navigation.

---

### 4.2 Unit list (Grammar → Browse all units)

- Grouped by **Phase** with collapsible section headers (current phase expanded by default; others collapsed)
- Each unit row: unit number + name + status icon (○ not started · ◐ in progress · ● complete)
- No accuracy %, no last-practiced date, no stats — keeps the list scannable
- Tapping a row → unit detail screen
- No hard locks (any unit accessible) — prerequisite warnings live on the unit detail screen, not on the row

---

### 4.3 Unit detail

A staging screen between the unit list and a practice session.

- Unit name + phase
- Short description (1–2 lines, authored on unit metadata)
- **Prerequisite warning** (only if user hasn't mastered prerequisites): a soft inline notice — *"You haven't completed [prereq unit]. You can practice anyway, or finish that unit first."* with both CTAs
- **Notes glossary** — vocabulary used in this unit's exercises that the user may not know yet (US #45). Displayed as a simple two-column glossary
- Primary CTA: **Start practice**

---

### 4.4 Grammar / Combined practice screen (the core loop)

The most-used screen. Variable-length session — user practices as many items as they want, then ends. Batched evaluation at end. Per-item feedback during practice is *deferred to the end-of-session review*.

**Layout:**
- Top-left: small Home icon (exits to home, attempts persist)
- Top-center: small running counter — *"7 attempted"*. No timer, no "remaining" count, no progress bar (no fixed length to progress against)
- Top-right: small Notes icon → opens Notes drawer (Grammar track only; combined track has no notes drawer by design)
- Center: the English cue, large serif type
- Below cue: text input field, auto-focused
- Bottom-right: persistent secondary-styled **End & review** button (always visible, never primary)

**Item interaction:**
- User reads cue, types Spanish translation, presses Enter (or clicks Submit)
- Item is logged to attempt_log (no eval yet)
- Next item appears immediately. No correct/wrong indication mid-session.
- Tag/skill name is **hidden** during regular practice (interleaving honest)

**Notes drawer (Grammar only):**
- Slides in from right. Shows the current unit's notes glossary
- Closeable; doesn't pause the session
- Available pre-answer or any time

**Ending:**
- **End & review** button → batched eval → review screen
- **End & review** with 0 attempts → bounces to home (no review screen)

---

### 4.5 End-of-session review screen

The single most important post-practice surface. After batched eval completes.

**Top:**
- Result hero: *"12 / 15 correct"* (or whatever the count was)
- Mastery callouts (only if events occurred): *"🎉 You mastered: Preterite -ar verbs"* (use accent color, not gold/sparkle treatments)

**Wrongs section (expanded by default):**
- Heading: *"Needs review"* with count
- Per item:
  - English cue
  - User's answer, struck through with red underline
  - One valid Spanish answer ("Correct: …")
  - Hint (LLM-generated — nudges without restating answer)
  - Explanation (LLM-generated — explains the grammar rule)
  - For combined-track items only: vocab impact line (*"`libro` advanced in pipeline"*)

**Corrects section (collapsed by default):**
- Heading: *"✓ 12 correct — tap to expand"*
- When expanded: each correct shows English cue · user's answer · ✓ · "explain" link to expand explanation per-item
- For combined-track items: vocab impact lines on corrects when expanded

**Follow-up CTAs section (bottom):**
- Error-cascade follow-ups (only if 3+ errors on the same tag): *"Follow-up session: 5 items on Ser vs Estar →"*
- Generic CTAs: **Practice again** (starts a new session in the same unit) and **Done** (back to home)

**Eval failure variant:**
- *"We couldn't reach the evaluator. Your answers are saved — try again in a moment."*
- CTAs: **Retry evaluation** and **Back to home**

**Loading variant:**
- *"Evaluating your answers…"* with calm spinner
- After ~10s: append *"Still working — sometimes the evaluator takes a bit longer."*

---

### 4.6 Deliberate practice — entry list

Reached from the inline pill on the home grammar card.

- Heading: *"Deliberate practice"*
- For each weak tag, a row showing:
  - Tag name (named explicitly — "Ser vs Estar", "Indirect object pronouns", etc.)
  - Brief weakness signal: *"6 wrong of last 20"*
  - Per-tag **Practice** CTA
- Top-level CTA: **Practice all weak skills**

---

### 4.7 Deliberate practice — practice screen

Same layout as regular practice screen with one key difference: a small banner at the top — *"Deliberate practice · Ser vs Estar"* (or *"Deliberate practice · 3 weak skills"*). Tag visibility is intentional here (blocked practice contract).

Otherwise: variable-length, batched eval, end-of-session review — identical to regular sessions. Mastery callouts on review may include *"Ser vs Estar is no longer flagged"* if the tag crosses back above mastery.

---

### 4.8 Vocabulary bank

Reached from tapping the vocab card or from the vocab session "Browse" affordance.

- **Search bar** at top — filter by lemma or English translation
- **Filter chips** below search: All · New · Learning · Mastered · Untouched
- **Sort dropdown** — default: frequency rank. Other options: state, recently learned, next due
- **Word list** — each row:
  - Frequency rank (small, e.g., *#142*)
  - Lemma (serif, prominent)
  - English translation (smaller)
  - State badge (untouched / new / learning / mastered) — minimal pill in muted color
- Tap row → word detail (modal or screen)

**Word detail:**
- Lemma · translation · part-of-speech · frequency rank
- State + brief history (review count, last review, next-due if applicable)
- Example sentences pulled from combined-track exercises that have used this word — bonus contextual reinforcement

---

### 4.9 Learn new words flow (vocabulary intake)

Full-screen flow reached from the "Learn new words" CTA on the vocab card.

**Setup screen:**
- Heading: *"How many words today?"*
- Stepper: 3 / 5 / 10 (default 5)
- Live pipeline-health line beneath: *"You have 14 active words (Healthy)"*
- CTA: **Begin**

**First-encounter cards (one per word):**
- Lemma (large serif)
- Translation (smaller)
- Part of speech label (e.g., *noun, masculine*)
- Frequency rank (e.g., *#142 most common*)
- Live pipeline-health line at bottom — updates as cards are acknowledged: *"After this word: 16 active (Healthy)"*
- CTA: **Got it →** (advances)
- No question, no input, no scoring — pure exposure

**Confirmation:**
- Summary: *"Add these 5 words to your pipeline?"* with the lemmas listed
- CTAs: **Add to pipeline** and **Cancel**

**Cancel anywhere:** no commit until confirmation.

---

### 4.10 Vocabulary flashcard session

Variable-length session. Mixed cards — system selects the interaction per card based on word state. The user does not pick a "mode."

**Multiple choice (new / learning words):**
- Spanish lemma at top, large serif
- 4 English translation options as tappable rows
- Tap an option → instant correct/wrong feedback (muted green/red flash), auto-advance after ~600ms
- Top counter: *"7 reviewed"* (no LLM, no batching, no review screen needed at end for correctness — just an end-of-session summary)
- Top-left Home icon, bottom-right End & review

**Self-rated recall (mature words):**
- Spanish lemma at top
- **Show answer** button (or Spacebar)
- After reveal: lemma + translation + 3 rating buttons: **Again** · **Good** · **Easy**
- User taps a rating → next card

**End-of-session summary (vocab):**
- Simpler than grammar's review screen — no LLM eval to display
- Hero: *"23 reviewed · 18 correct on first try"*
- Mastery callouts: *"🎉 `gato` moved to mastered"*
- CTAs: **Practice again** · **Done**

---

### 4.11 Empty states

**Day zero (entire app empty):** as covered in home spec — Continue strip hidden, each card shows its day-zero variant. No tutorial overlay; cards self-teach.

**Vocab bank with no learned words:** filter chips still present; word list shows the full untouched bank by frequency rank. CTA bar at top: *"Add your first words →"*.

**Deliberate practice when no weak tags:** entry pill on home grammar card simply doesn't appear. If user navigates to deliberate practice via some other path, screen shows: *"No weak skills right now. Keep practicing."* with **Back to home** CTA.

---

## 5. States cheat-sheet (for designer)

For each screen, designers should produce variants for:

- **Default / populated state**
- **Day-zero / empty state**
- **Loading state** (where applicable — eval in progress, fetching words, etc.)
- **Error state** (LLM eval failure, network issues)
- **Hover / focus states** for interactive elements (it's a desktop app)

---

## 6. Keyboard interaction (target end-state, deferred to later development)

Full bindings live in the PRD's "Keyboard Interaction" section. Designer should reserve space and design states for the inline first-session keyboard hints under input fields, and for the `?` shortcuts modal accessible from a top-right icon on every screen.

---

## 7. Microcopy reference

A starter set of strings — designer/copywriter to expand with the same tone:

- Day-zero vocab: *"Pipeline is light — add some words to begin."*
- Healthy pipeline: *"Pipeline healthy · room for 14 more."*
- Full pipeline: *"Pipeline filling up — consider consolidating."*
- Overloaded pipeline: *"Pipeline is full. Finish what you have first."*
- Mastery callout: *"You moved `libro` to mastered."*
- Unit complete: *"Unit complete: Preterite -ar verbs."*
- Wrong answer (review): *"Not quite — here's why."*
- Eval failure: *"We couldn't reach the evaluator. Your answers are saved — try again in a moment."*
- Pending session banner: *"You have an unsubmitted session — review now."*
- Deliberate practice surfacing: *"3 skills need review →"*

---

## 8. Open items / explicit non-decisions

These are design choices intentionally left to the designer:

- Specific accent color hex (within "muted green or terracotta, fountain-pen-ink feel")
- Specific serif and sans typeface choices (within the families described)
- Default Tauri window dimensions and resize behavior
- Dark-mode color tokens
- Exact spacing scale and grid
- Loading spinner style (something calm — not a candy-color throbber)

---

## 9. PRD discrepancies resolved during design session

For audit purposes — these were identified in the grilling and have been amended in the PRD:

1. US #8 — "immediately" feedback changed to end-of-session batched feedback
2. US #9 — hint moved from inline to review screen; per-item retry removed
3. US #10 — explanation moved to review screen; default-expanded for wrongs, collapsed for corrects
4. US #19 — error cascade moved from mid-session insertion to post-session follow-up CTA
5. New US #21 — end-of-session review screen as a named surface
6. New US #22 — unit detail screen as a named surface
7. US #24 — "untouched" added as a fourth vocabulary state
8. US #34 — "active pipeline" clarified as new + learning states
9. Implementation Decisions amended for: canonical answer display rules, batched submission flow, variable-length sessions (no fixed item count), pipeline health bands, intake flow as first-encounter exposure, flashcard interaction details, deliberate practice tag-visibility contract, combined-track no-notes design, Visual Design section added, Keyboard Interaction section added, Failure Modes & Resilience section added.
