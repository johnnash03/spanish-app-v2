# Spanish App v0

## Problem Statement

**How might we turn 76+ hand-written Spanish lesson notes into a personal practice app that drives structures to automaticity — while planting the data seams for a later system that constrains exercise and conversation generation to exactly what the user has learned?**

## Recommended Direction

A Tauri desktop app with a linear list of lessons. Each lesson has a pre-written note (adapted from `ReferenceNotes.md`) and ~15 AI-generated EN→ES translation exercises. Claude generates and grades. Mistakes are logged with per-exercise metadata (lesson, structures exercised, vocab used). Progress stored locally in SQLite + markdown lesson files on disk.

The app trusts Claude for generation and grading in v0 — if quality falls short, the prompts tighten; only if that fails does a curated bank enter the picture.

The differentiator isn't visible in v0. It's the tagged-mistake history that compounds: layer 2 (deliberate practice on weaknesses, introduced vocab bank) consumes the mistake log; layer 3 (constrained chat) consumes the learned-vocab set. v0's job is to produce that data cleanly while being a usable practice tool.

## Key Assumptions to Validate

- [ ] **Claude generates on-target exercises.** Produce 20 EN→ES exercises from lesson 5's note in isolation; eyeball: do they exercise the target structure, and do they stay within introduced vocab?
- [ ] **Claude grades with the right strictness.** Submit 20 canned answers (10 correct, 10 with specific error types — wrong word order, wrong pronoun position, missing accent, wrong verb form). Does the grader flag the right things and accept valid alternatives like `Quiero verlo` vs `Lo quiero ver`?
- [ ] **Lesson notes can be rewritten without losing their flavor.** Adapt 3 lessons from the raw notes. Read as a student. Still feels like the heuristic, pattern-based teaching that makes these notes work?
- [ ] **Typed EN→ES sustains a daily habit.** Use a paper version for 3 days (read English, write Spanish). If that feels good, the app will.
- [ ] **Structure tags don't drift.** Hand-tag 3 lessons with their structures. Crisp and non-overlapping, or fuzzy and you keep re-splitting?

## MVP Scope

**In:**

- Tauri app, local SQLite (via `tauri-plugin-sql`) for state, markdown files on disk for lesson notes.
- Lesson list, linear order. Pick a starting slice: **first 20 lessons** to validate the loop before authoring the rest.
- Lesson detail view: render note (markdown) → "Start practice" → 15 exercises.
- Exercise flow: show English prompt → user types Spanish → submit → AI grades (correct / partial / wrong with 1-sentence feedback) → log result → next.
- Exercise set = 12 from current lesson + 3 interleaved from random prior lessons (trivial interleaving; real algorithm later).
- Per-lesson metadata file: `{ id, title, note_path, primary_structures[], introduced_vocab[] }`. Authored once (with Claude's help).
- Each exercise generation call returns its own tags (`structures_used`, `vocab_used`) alongside the sentence. Grading result + tags stored.
- Resume: open app → continue from last incomplete lesson.
- Anthropic API key stored in OS keychain (`tauri-plugin-stronghold`) or env.

**Explicitly out of v0:**

- Deliberate practice (targeted retest of logged mistakes) — layer 2.
- Real interleaving algorithm / spaced repetition — layer 2.
- Vocab bank + "I don't know this word" flow — layer 2.
- Free-form conversation with known-vocab constraint — layer 3.
- Strict generation constraint (AI only using known vocab). v0 trusts Claude.
- Automaticity measurement (response timing). Useful later, skip now.
- Multi-user accounts, cloud sync, auth. Local-only; keep seams so data is user-scoped in the schema (`user_id = 'local'` for now).
- Mobile, PWA, web version.
- TTS / STT / spoken practice.
- Dependency graph between lessons. Linear order; store `prerequisite_lesson_ids int[]` field but only populate as `[n-1]` for now — leaves the door open.
- Lesson authoring UI. Notes are markdown files edited in your editor.

## Not Doing (and Why)

- **Curated exercise banks** — Claude is good enough; curation is huge work. Add only if the first assumption fails.
- **Strict vocab constraint in generation** — premature. Needs a reliable vocab schema first, which v0 doesn't yet have. Defer to layer 2.
- **Lesson-authoring UI** — markdown on disk is faster than building a CMS.
- **Accounts & cloud** — local SQLite beats the alternative for a solo user by weeks. Schema already user-keyed → migration path stays open.
- **Mobile** — practice habits live on mobile, but Tauri desktop ships faster and you're the only user. Port later if the habit sticks.
- **Graph dependencies** — linear is enough for a beginner and 50 lessons of runway. Add `prerequisite_lesson_ids[]` column pre-emptively; populate later.

## Open Questions

1. **Lesson count for first cut:** ship first 20 lessons to validate the loop, or go all-in on 76? Recommend: 20, validate, then batch-author the rest.
2. **Note adaptation pass:** do you rewrite each note into lesson form, or does Claude do a first pass that you edit? Recommend: Claude drafts, you edit.
3. **Structure taxonomy:** is there a canonical list of structures (tagged across lessons), or do tags emerge per-lesson and dedupe later? Recommend: emerge, dedupe in a pass before layer 2.
4. **Grading output format:** binary (right/wrong), ternary (right/partial/wrong), or rich (per-token diff)? Recommend: ternary + one sentence of feedback. Log the raw user answer so richer analysis is possible later.
5. **Exercise count per lesson:** 15 is a number I made up. Is 10 enough? Is 20 too many in one sitting? Recommend: 15 with a "pause for now" escape.

## Architecture Notes for Later Layers

### Runtime vocab bank (layer 2)

A learner-facing vocab bank is planned as a runtime artifact, not a curriculum-authoring artifact. It unlocks the "words I know" screen, tap-to-look-up, "I don't know this word" flagging, and deliberate practice on weak words.

Populated at runtime from three sources:

- **Static** — lesson frontmatter (`vocab_introduced: [lemma, ...]` in each `lesson-NN.md`). Provides `lemma` + `first_seen_lesson`.
- **Dynamic** — SQLite learner history. Per-lemma encounter/miss counts, last-seen timestamp, unknown flag.
- **Lazy** — Claude-enriched metadata (gloss, gender, pos, conjugation), cached in SQLite on first display. Never re-called.

Schema sketch:

```sql
CREATE TABLE vocab_bank (
  lemma TEXT PRIMARY KEY,
  first_seen_lesson INTEGER NOT NULL,
  cached_gloss TEXT,
  cached_gender TEXT,
  cached_pos TEXT,
  cached_conjugation TEXT,
  times_encountered INTEGER DEFAULT 0,
  times_missed INTEGER DEFAULT 0,
  last_seen_at TIMESTAMP,
  flagged_unknown BOOLEAN DEFAULT FALSE
);
```

Population:

- **Curriculum ingest** (on curriculum update): crawl lesson frontmatter, upsert `(lemma, first_seen_lesson)` rows.
- **First display** of a lemma: single Claude call fills the `cached_*` columns. Never called again for that lemma.
- **Exercise runtime**: increment encounter/miss counters, update `last_seen_at`.
- **Transfer-pattern-derived vocab** (e.g. arbitrary `-cion` words unlocked via `suffix-tion-to-cion`): inserted on first actual appearance in an exercise, with `first_seen_lesson` = the transfer pattern's introduction lesson.

Why defer to layer 2: v0 trusts Claude for generation + grading; the bank becomes valuable once deliberate-practice and unknown-word flows land. Architecture is already compatible — lesson frontmatter carries the authored vocab; SQLite carries the rest.

## Success Metric

Primary: **"I can translate the structures I have learned without much thinking."**

Instrumentation for v0: log per-exercise `{answered_at, first_keystroke_at, submitted_at, correct?}`. Response time on structures you've drilled is the quantitative proxy for automaticity; the qualitative signal is your own. Don't build a dashboard yet — just log so you can query later.

Secondary (leading indicators): opened the app ≥10 of 14 days; completed lessons are revisited (mistakes re-drilled) rather than abandoned.
