# Design Handoff: léxico — Spanish Learning App

## Overview

This package contains a high-fidelity HTML prototype for a local-first Tauri desktop app for learning Spanish. It covers all major screens across three practice tracks: Grammar, Vocabulary, and Combined. The prototype is click-through (pre-canned answers, no live LLM evaluation) with reactive state where it aids comprehension (flashcard MC feedback, learn-new-words flow, etc.).

---

## About the Design Files

The files in this bundle are **design references built in HTML/React** — they are not production code to ship. The task is to **recreate these designs in the Tauri codebase** using its established patterns, router, state management, and component library. Do not copy the JSX directly; treat it as a detailed visual and behavioural spec.

Open `Spanish App.html` in a browser to navigate the prototype. A floating screen-jumper in the bottom-left lets you jump to any screen directly.

**Fidelity: High.** Colours, typography, spacing, component shapes, copy, and interaction patterns are all final or near-final. Implement pixel-faithfully.

---

## Design Tokens

### Colours

```
--paper:        #F7F4ED   /* main background */
--paper-2:      #F1EDE3   /* card backgrounds, inputs */
--paper-3:      #E8E2D4   /* hover tints */
--rule:         #DCD4C2   /* borders, dividers */
--rule-soft:    #E6DFCE   /* subtle borders */

--ink:          #2A2622   /* primary text */
--ink-2:        #4A433C   /* secondary text */
--ink-3:        #7A7068   /* tertiary / muted */
--ink-4:        #A39A8E   /* placeholder / disabled */

--accent:       #3F6B4E   /* primary accent — fountain-pen ink green */
--accent-2:     #355C42   /* accent hover */
--accent-soft:  #DCE5DD   /* accent border */
--accent-tint:  #EDF1EB   /* accent background tint */

--good:         #3F6B4E   /* correct feedback */
--bad:          #A8553E   /* wrong feedback */
--bad-soft:     #EFDDD4   /* wrong background tint */
```

### Typography

| Role | Family | Size | Weight | Notes |
|---|---|---|---|---|
| Wordmark | Lora | 20px | 500 | Lowercase, letter-spacing -0.02em |
| Screen headings | Lora | 30–56px | 400 | letter-spacing -0.015 to -0.02em |
| Spanish cues (practice) | Lora | 36px | 400 | letter-spacing -0.015em |
| Lemmas (vocab) | Lora | 18–80px | 400 | Scales by context |
| Body / explanations | Lora | 15–17px | 400 | line-height 1.6 |
| UI labels / buttons | Manrope | 13–15px | 500 | letter-spacing -0.005em |
| Eyebrows / chips | Manrope | 11–12px | 500–600 | UPPERCASE, letter-spacing 0.07–0.09em |
| Counters / ranks | JetBrains Mono | 12–13px | 400 | tabular-nums |

Load via Google Fonts:
```
Lora: ital,wght@0,400;0,500;0,600;1,400;1,500
Manrope: wght@400;500;600;700
JetBrains Mono: wght@400
```

### Spacing & Radius

```
Border radius — sm:  4px
Border radius — md:  6px
Border radius — lg:  10px
Border radius — pill: 999px

Card shadow: 0 1px 0 rgba(42,38,34,0.04), 0 2px 12px rgba(42,38,34,0.04)
Pop shadow:  0 8px 30px rgba(42,38,34,0.10)
```

### Icons
**Phosphor Icons, regular weight** — thin-stroke, linear, no fill or duotone variants. Size 18–20px in topbar/nav; 14–16px inline. The prototype uses custom inline SVGs that match Phosphor's style — in production use the `phosphor-react` or `@phosphor-icons/react` package.

---

## Screens

### 1. Home
**File:** `screens-1.jsx` → `HomeScreen`
**Purpose:** Hub. Shows all three tracks and surfaces the most important action for each.

**Layout:** Single-column, max-width 1120px, 28px side padding.
- Top: persistent topbar (wordmark left, shortcuts icon right)
- Below topbar: conditional **Continue strip** — full-width pill bar, `#FBF9F3` bg, 14px 18px padding. Shows when there is a session to resume.
- Greeting: Lora 30px `"Buenas tardes."` + 14px muted subtitle
- Cards: `display: grid; grid-template-columns: 1.1fr 1fr 0.9fr; gap: 20px`

**Grammar card:**
- Eyebrow "GRAMMAR" + Layers icon
- Unit name (Lora 22px) + unit title (Lora 16px muted)
- Progress: `"X of Y toward mastery"` 13px muted + 3px progress bar (accent fill)
- Deliberate practice pill (conditional): 999px border pill, bad-colour dot, 13px
- Primary CTA: `btn-primary` "Continue Unit 7"
- Secondary: text-link "Browse all units"

**Vocabulary card:**
- Eyebrow "VOCABULARY" + Cards icon
- Hero number: Lora 44px, `"words mastered"` 14px muted below
- Pipeline health line: 13px `"Pipeline healthy · room for N more"`
- Micro line: 12px muted `"N learning · N new"`
- Primary CTA: `btn-accent` "Review" + badge count
- Secondary: text-link "Learn new words"

**Combined card:**
- Eyebrow "COMBINED" + Spark icon
- Serif tagline 22px, ready indicator dot
- CTA: `btn-secondary` "Practice" full-width

**Day-zero variant:** Continue strip hidden. Grammar card shows Phase 1 · Unit 1 with "Start" CTA. Vocab hero shows 0, review button disabled style. Combined card shows locked state with copy `"Unlocks when you have 10 words in your pipeline. You have N."`

---

### 2. Unit List
**File:** `screens-1.jsx` → `UnitListScreen`
**Purpose:** Browse all grammar units grouped by phase.

- Topbar with Home icon
- Page heading: Lora 32px "All units" + description
- Phases as collapsible sections — chevron toggles, current phase open by default
- Each phase row: chevron + eyebrow "PHASE N" + Lora 18px name + pill or completion count
- Unit rows inside: `grid-template-columns: 32px 60px 1fr 100px` — status dot + unit code (mono) + Lora unit name + status text
- Status dots: empty circle = not started, half-fill = in progress, solid accent = complete
- Click any unit → Unit Detail

---

### 3. Unit Detail
**File:** `screens-1.jsx` → `UnitDetailScreen`
**Purpose:** Staging + reading surface before starting a practice session.

- Back link "← All units"
- Eyebrow phase/unit + Lora 36px unit name + Lora 17px italic description
- Stats row: 4× `Stat` blocks (toward mastery, recent accuracy, sessions, last practiced)
- **Notes section** — full reading material:
  - NotesTOC: row of pill-chips for each section, jump on click
  - Section: *When you reach for it* — prose with `*italic*` spans in Lora
  - Section: *Formation* — prose + conjugation table (`border: 1px solid rule-soft`, `background: paper-2`, thead uppercase labels, Lora cells)
  - Section: *In the wild* — example sentence pairs (Spanish Lora 17px / English muted 13px)
  - Section: *Watch out for* — bullet list in Lora
  - Section: *Glossary* — 2-col grid, lemma (Lora) / translation (muted)
- Primary CTA: btn-primary-lg "Start practice →" + muted hint text

**Prerequisite warning** (conditional): soft inline notice with two CTAs. Not shown in current mock (prereqs met).

---

### 4. Practice — Grammar / Combined
**File:** `screens-1.jsx` → `PracticeScreen`
**Purpose:** Core practice loop. Variable-length. No per-item feedback.

- Minimal topbar: Home icon left · counter "N attempted" + Notebook icon right
- **No wordmark** while practising (reduces distraction)
- Full-bleed canvas centred, max-width 680px:
  - Eyebrow "TRANSLATE TO SPANISH" in ink-4
  - Cue: Lora 36px `class="cue"`, generous line-height 1.3
  - Input: `class="input-bare"` — transparent bg, 1px bottom border, Lora 24px, auto-focused; Enter = submit
  - Helper text row: 12px muted (keyboard hint left, "Feedback at end" right)
- Fixed bottom-right: `btn-secondary` "End & review"
- **Notes drawer** slides from right (540px wide):
  - Header: eyebrow + unit name + X close button
  - Body: NotesTOC chips + NotesBody (compact mode: smaller font, 1-col glossary)
  - Scrim behind drawer, click to close

**Deliberate practice variant:** small banner strip at top `"Deliberate practice · [Tag name]"` — tag name is intentionally visible in this mode.

---

### 5. End-of-Session Review
**File:** `screens-1.jsx` → `ReviewScreen`
**Purpose:** Post-session feedback after batched LLM evaluation.

- Topbar with Home icon
- Hero: Lora `"N / N"` 56px + italic `"correct"` 24px
- **Mastery callout** (conditional): accent-tint bg, accent left border 2px, Spark icon + `"You mastered: [tag]"` in accent-2 colour
- **Needs review section:**
  - Each wrong: `background: #FBF9F3, border-left: 2px solid var(--bad)`
  - Grid inside: `"You" / struck-through user answer` + `"Correct" / correct in accent-2`
  - Hint line: 13px ink-2
  - Why/explanation line: 13px ink-2 with `"Why"` label
- **Corrects section:** collapsed by default, toggle row with chevron. When expanded: check icon + English cue + Lora user answer + "explain" text-link per row
- **Follow-up block** (conditional, 3+ errors on same tag): border card with follow-up session CTA
- Footer CTAs: "Done" (btn-secondary) + "Practice again →" (btn-primary)

**Loading variant:** `"Evaluating your answers…"` + calm spinner. After 10s append `"Still working…"` line.
**Error variant:** error message + "Retry evaluation" + "Back to home" CTAs.

---

### 6. Deliberate Practice List
**File:** `screens-2.jsx` → `DeliberateScreen`
**Purpose:** Entry point for drilling specific weak skills.

- Back link, heading "Deliberate practice", Lora italic description
- Per-tag row: `grid-template-columns: 1fr auto auto` — tag name (Lora 18px) + accuracy bar (120px wide, 4px height, bad-colour fill) + "Practice" btn-secondary-sm
- Top CTA: btn-primary "Practice all weak skills →" + muted note that tag names are visible

---

### 7. Vocabulary Bank
**File:** `screens-2.jsx` → `VocabBankScreen`
**Purpose:** Browse and filter all 2000 SUBTLEX-ESP words.

- Header row-between: heading + mastery fraction (`"247 / 2000"`)
- Search: `class="search-input"` with Search icon absolutely positioned at left
- Filter chips: "All · New · Learning · Mastered · Untouched" — active chip: `background: ink, color: paper`. Count badge per chip.
- Sort dropdown: right-aligned text-link + caret
- Table header: `grid-template-columns: 70px 1fr 1.4fr 100px 120px`
- Word rows: same grid. Rank mono, Lemma Lora 18px, Translation muted 14px, POS italic muted 12px, State badge right-aligned
- State badge dots: untouched = ink-4, new = `#C5A572`, learning = `#6B8AAB`, mastered = accent
- Row hover: `background: #FBF9F3`
- Click row → Word Detail

---

### 8. Word Detail
**File:** `screens-2.jsx` → `WordDetailScreen`
**Purpose:** Full word record with history and contextual sentences.

- Back link, rank mono 12px, Lora 56px lemma, Lora 22px italic translation
- Pills: part-of-speech pill + state badge
- Review history card: `background: #FBF9F3, border: 1px solid rule-soft` — 4-col stat grid (Reviews, Correct, Last seen, Next due)
- "Seen in" section: example sentences from Combined-track sessions — Spanish Lora 17px + English muted 13px + relative date right-aligned

---

### 9. Learn New Words Flow
**File:** `screens-2.jsx` → `LearnScreen`
**Purpose:** First-encounter exposure flow. 3 steps.

**Step 1 — Setup:**
- Lora 36px heading "How many words today?"
- 3 large stepper buttons: 3 / 5 / 10. Selected: `background: ink, color: paper`. Lora 28px numerals.
- Pipeline health card below
- "Begin →" btn-primary-lg

**Step 2 — First-encounter card (one per word):**
- Counter "N of N" top-left
- Rank mono, Lora 80px lemma, Lora 22px translation, muted 13px italic POS
- Pipeline health update at bottom (updates as cards advance)
- "Got it →" btn-primary advances; on last card goes to Step 3
- 2px progress bar at bottom

**Step 3 — Confirm:**
- "Add these N words to your pipeline?" heading
- List: lemma (Lora 19px) + translation (muted 14px), separated by rule-soft borders
- "Add to pipeline" btn-accent-lg + "Cancel" btn-ghost

---

### 10. Vocabulary Flashcard Session
**File:** `screens-2.jsx` → `FlashcardScreen`
**Purpose:** SRS review loop. Two card types.

**Multiple choice (new/learning words):**
- Minimal topbar: Home · counter · "End & summary" btn-secondary-sm
- Eyebrow centred "WHAT DOES THIS MEAN?"
- Lora 72px lemma centred
- 4 tappable option rows: `border: 1px solid rule, border-radius: r-md, padding: 18px 22px, Lora 18px`
- Tap: correct option → accent-tint bg + accent border + Check icon; wrong → bad-soft bg + bad border + X icon
- Auto-advance after ~600ms

**Self-rated recall (mature words):**
- Lora 80px lemma centred
- "Show answer" btn-secondary-lg (or Spacebar)
- After reveal: Lora 24px italic translation appears
- Rating buttons: "Again" btn-secondary-lg · "Good" btn-primary-lg · "Easy" btn-accent-lg
- Keyboard hints in mono shown on buttons

---

### 11. Vocab Summary
**File:** `screens-2.jsx` → `VocabSummaryScreen`
**Purpose:** End-of-flashcard-session summary. No LLM eval needed.

- Lora 56px "N reviewed" + Lora 22px italic "N correct on first try"
- Mastery callouts: same accent-tint/left-border pattern as grammar review
- Pipeline movement card: 4-col stat grid
- Footer CTAs: "Done" + "Practice again →"

---

## Interactions & Navigation

**Architecture:** Hub-and-spoke. Home is the only persistent hub. All other screens have a Home icon (top-left) that returns to Home directly. No persistent sidebar or tab bar.

**Transitions:** Subtle fade + 4px translateY on screen mount (`fadeIn` keyframe, 220ms ease). No springy bounces.

**Notes drawer:** slides in from right (`transform: translateX(100%)` → `translateX(0)`, 240ms `cubic-bezier(0.2, 0.7, 0.2, 1)`). Scrim behind at `rgba(42,38,34,0.18)`. Click scrim to close.

**Collapsibles:** chevron rotates -90deg → 0deg on open, 160ms ease.

**Practice submit:** Enter key or button click logs item, clears input, advances to next cue. No feedback mid-session.

**Flashcard MC feedback:** correct/wrong highlight on tap, 800ms delay then auto-advance.

**Phase sections on Unit List:** current phase expanded by default, others collapsed.

---

## State Notes (for implementation)

- `weakTags` on learner → deliberate practice pill visibility on Home
- `dueCount === 0` → vocab review button shows "No reviews due" in disabled style (still tappable)
- `activeWords < 10` → Combined card locked state
- `continueSession` presence → Continue strip visibility
- Practice counter increments on each submission, never decrements
- Corrects section on Review defaults collapsed; wrongs default expanded
- Notes drawer state is local to practice session (open/closed)

---

## Design System Notes

- **No emoji** anywhere in the UI. Mastery callouts use a Spark SVG icon in accent colour instead of 🎉.
- **No gradients** on backgrounds or cards.
- **No filled icons** — Phosphor regular weight only.
- **All button hover states** use either a darkened background tint or border-color shift. No scale transforms.
- **Cards** use `background: #FBF9F3` (slightly warmer than paper) with `border: 1px solid var(--rule-soft)` and `border-radius: 10px`.
- **Microcopy tone:** warm, plainspoken, second-person. "Not quite — here's why." "You moved `libro` to mastered." No formal stiffness, no gamification.

---

## Files in This Package

| File | Purpose |
|---|---|
| `Spanish App.html` | Main prototype — open in browser to navigate all screens |
| `styles.css` | Full CSS token + component system |
| `data.jsx` | Mock data (mid-journey learner state, sample content) |
| `icons.jsx` | Inline SVG Phosphor-style icon components |
| `screens-1.jsx` | TopBar, Home, Unit List, Unit Detail, Practice, Review |
| `screens-2.jsx` | Deliberate Practice, Vocab Bank, Word Detail, Learn flow, Flashcards, Vocab Summary |

---

## Quick Start for Claude Code

1. Open `Spanish App.html` in a browser and explore all screens via the bottom-left screen jumper.
2. Read `styles.css` for the full token system — copy relevant CSS variables into your design system.
3. Read the screen sections above one at a time and implement each screen in your target framework.
4. Reference `data.jsx` for the data shape each screen expects.
5. Use `screens-1.jsx` and `screens-2.jsx` as behavioural reference — the component logic shows exactly what state each screen needs and how interactions flow.

The prototype is intentionally over-documented in code comments — the JSX is meant to be read as a spec, not executed in production.
