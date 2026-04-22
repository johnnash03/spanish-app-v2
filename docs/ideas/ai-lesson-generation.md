# AI-Driven Lesson Spec Generation

## Problem Statement

**How might we define a lesson-spec format, rule catalog, and dependency model — drafted by AI from `ReferenceNotes.md`, reviewed and edited by hand — that lets the exercise generator reliably produce exercises which (a) primarily drill the target lesson's structures and (b) never use grammar the learner hasn't been taught?**

## Recommended Direction

A **hybrid rule-DAG + AI-drafted curriculum** approach, delivered in **batches of 20 teacher sections** from `ReferenceNotes.md`.

Each lesson has a machine-readable spec (rule IDs, vocab lemmas, transfer patterns, anchor sentences, prerequisites) plus a human-readable body. Rules are named, atomic, and live in a versioned catalog. Rule dependencies form a DAG; lessons inherit the transitive closure of rules from their prerequisite lessons.

The real engineering effort is in the **Stage 1 prompt**, which extracts the rule taxonomy and proposes a lesson plan. Stage 2 (generating lesson specs against a frozen catalog) is mostly templating once taxonomy is stable.

Batch 1 (teacher sections 1–20) is the pilot. If its output is good after edit, batch 2 reuses the same prompts with the frozen catalog fed back as input.

## Division of Authority: ReferenceNotes vs. AI

The ReferenceNotes are battle-tested in some dimensions and incidental in others. The reconciliation is to split authority cleanly — not to compromise.

### ReferenceNotes is authoritative on **pedagogy**

- **Sequence of grammar concept introduction.** Postverbal-only pronouns (§5) before placement flexibility (§11) before multi-verb placement (§22). Ser/estar introduced together with the state-vs-characteristic contrast. If the teacher introduces X before Y, AI must too.
- **Pedagogical heuristics that aren't transfer patterns.** Genuine insights like the ser/estar state-vs-characteristic frame, or "verbs of motion take `a`". (Transfer patterns handled separately — see below.)
- **Anchor sentences that stack rules.** "Voy a intentar ver los" exercises four rules in one sentence. Crafted, not generated. Start from these.
- **Pattern-based derivation voice.** The whole teaching flavor survives into lesson bodies.

### AI is authoritative on **packaging**

- **Lesson boundaries.** Merge small teacher-sections, split dense ones. Total count is fluid.
- **Rule naming.** Teacher named nothing formally. AI designs the taxonomy.
- **Completeness and gaps.** The notes assume things they never state (negation, basic word order). AI fills these in.
- **Explanatory prose.** The body is AI-written in the teacher's heuristic voice.
- **Schema structure.** Frontmatter, IDs, dependency graph — mechanical.
- **Placement of transfer-pattern lessons.** Interleaved early for vocab density — not bound to teacher section order.

### The principle, one line

> **ReferenceNotes is authoritative on *pedagogy* — grammar sequence, heuristics, anchor sentences. AI is authoritative on *packaging* — lesson boundaries, rule names, completeness, prose, schema, transfer-pattern placement.**

### The sequencing constraint that falls out of this

> **AI may re-chunk lessons, rename concepts, fill gaps, and rewrite prose freely. AI may NOT change the order in which grammar/structural concepts are first introduced. If it believes the teacher's order is wrong, it flags the disagreement in `batch-NN-sequencing-concerns.md` with reasoning — no silent resequencing.**

## Transfer Patterns — Handled Separately

The teacher's notes introduce ~10–12 English-to-Spanish transfer patterns (`-tion → -cion`, `-ity → -idad`, `-ive → -ivo`, etc.). These are qualitatively different from grammar rules — one-shot-understandable, non-composing, no spaced practice needed — and routing them through the rule DAG would pollute it. They get their own artifact.

### Inventory from the notes

1. `-al` stays — normal, legal, natural
2. `-ant/-ent → -e` — importante, diferente
3. `-mente = -ly` — realmente, constantemente
4. `-ible/-able` stays — posible, durable
5. `-tion → -cion` (+ `-ar` verb form) — preparacion, preparar
6. `-ence/-ance → -encia/-ancia` — diferencia, importancia
7. `-ity → -idad` — universidad, posibilidad
8. `-ary → -ario` — necesario, contrario
9. `-ive → -ivo` — creativo, intuitivo
10. `e-` prefix on s-clusters — escuela, estudiante, especial
11. `pose → poner` verb-stem cognate — componer, suponer, imponer

### How they differ from rules

- **Not in `rules.yaml`.** Live in a separate `transfer-patterns.yaml`.
- **No prerequisites, no DAG.** Flat list.
- **Not drilled.** Exercise generator never selects them as a "primary structure to drill."
- **Light confirmation only.** When introduced in a lesson, 3–5 quick exercises.
- **Expand the vocab universe passively.** Once `-tion → -cion` is introduced, every Spanish `-cion` word becomes available without enumerating them.

### Placement

Interleaved in the first ~third of the curriculum, roughly tracking the teacher's order but free to cluster for vocab density. Each transfer pattern unlocks vocab for the next few grammar lessons to exploit. Placement is AI's packaging call.

### Mnemonics

Items like "savvy → saber" or "Cuerpo → corporate" are not transfer patterns, not rules, and not structured data. Optional prose in the lesson body — default to omitted unless clearly useful.

## Vocab: Lesson Frontmatter Is the Source of Truth

**No separate `vocab.yaml` at authoring time.** Lesson frontmatter (`vocab_introduced: [lemma, lemma, ...]`) is the single authored source for which words have been taught and when. Only two pieces of information are load-bearing for exercise generation: the **lemma** and the **lesson it was introduced in** — and both are already in the frontmatter.

Metadata Claude already knows (gloss, gender, part-of-speech, conjugation class) is not pre-authored. It gets derived at runtime when the app needs it. See **Runtime Vocab Bank** below.

**Vocab universe at lesson N** = union of `vocab_introduced` across all lessons ≤ N ∪ anything derivable via a transfer pattern introduced by lesson N.

### Exception words are owned by rules, not by per-word metadata

Exceptions like `el problema`, `el sistema`, `la mano` are attached to the rule they violate — the `noun-gender-a-feminine` rule in `rules.yaml` carries its exception list. Not duplicated as per-word flags.

### Irregular verbs

Claude knows `querer`, `saber`, `ir`, `tener`, `dar` are irregular. If an exercise class needs to constrain on regularity (e.g. drilling regular `-er` present tense must avoid `querer`), add an `irregular-verbs` rule to `rules.yaml` with a `vocab_refs` list — same exception-cluster pattern.

## Runtime Vocab Bank (Layer-2 Concern)

A learner-facing vocab bank **is** useful — for a "words I know" screen, tap-to-look-up, the "I don't know this word" flow, and deliberate practice on weak words. But it's a **runtime artifact**, not an authoring artifact.

Populated at runtime from three sources:
- **Static**: lesson frontmatter (`lemma`, `first_seen_lesson`)
- **Dynamic**: SQLite learner history (encounter/miss counts, last-seen, unknown-flag)
- **Lazy**: Claude-enriched metadata (gloss, gender, pos, conjugation), cached in SQLite on first display

Full SQLite design lives in `docs/ideas/spanish-app-v0.md` as layer-2 architecture. Not part of curriculum authoring.

## Core Artifacts the Pipeline Produces

```
rules.yaml                             — versioned rule catalog (append-mostly)
transfer-patterns.yaml                 — flat list of English→Spanish transfer patterns
lessons/lesson-01.md … lesson-NN.md    — frontmatter + human body (carries vocab_introduced)
generation-notes/
  batch-01-decisions.md                — taxonomy design log: merges, splits, renames
  batch-01-improvements.md             — where AI diverged from teacher packaging and why
  batch-01-gaps.md                     — rules used in examples but not stated
  batch-01-sequencing-concerns.md      — disagreements with teacher's order (read carefully)
```

No `vocab.yaml`. Vocab is carried in lesson frontmatter; exceptions live in their owning rule.

## Rule Catalog Schema (`rules.yaml`)

```yaml
rules:
  - id: do-pronoun-postverbal-infinitive
    category: grammar
    description: "Direct object pronoun attaches to end of an infinitive."
    prerequisites: [ar-infinitive, do-pronoun-lo-la-me-te]
    introduced_in_lesson: 5
    examples:
      - "quiero cancelar lo"
      - "quiero visitar lo"
    exceptions: []

  - id: ser-vs-estar-state
    category: grammar
    description: "Use estar for states (tired, here, bored-as-state); ser for characteristics."
    prerequisites: [ser-present, estar-present]
    introduced_in_lesson: 29
    examples:
      - "Estoy cansado"
      - "Es bueno"
    exceptions: []

  - id: noun-gender-a-feminine
    category: grammar
    description: "Nouns ending in -a tend to be feminine."
    prerequisites: []
    introduced_in_lesson: 27
    exceptions:
      - note: "-ma ending nouns from Greek are masculine"
        vocab_refs: [problema, sistema, tema, paradigma, diagrama, esquema, programa, planeta]
```

**Categories:** `grammar | phonology | exception-cluster`

No `morphology-derivation` category — that's transfer patterns, handled separately.

**Granularity:** AI's judgment (packaging). When the teacher introduces a variant at a different section, that's strong evidence for splitting — but AI can override when the split serves no pedagogical purpose.

## Transfer Patterns Schema (`transfer-patterns.yaml`)

```yaml
transfer_patterns:
  - id: suffix-tion-to-cion
    english_cue: "word ends in -tion"
    spanish_form: "replace -tion with -cion (feminine noun); drop -cion and add -r for -ar verb"
    introduced_in_lesson: 4
    examples:
      - "nation → nacion"
      - "preparation → preparacion → preparar"

  - id: suffix-al-stays
    english_cue: "word ends in -al"
    spanish_form: "same -al form"
    introduced_in_lesson: 2
    examples: [normal, legal, metal, natural]
```

## Lesson Spec Schema (`lesson-NN.md`)

Pure-grammar lesson:

```markdown
---
id: 5
title: "Object pronouns stick to 'to-verbs'; yo for emphasis"
source_refs: ["ReferenceNotes.md#5", "ReferenceNotes.md#6"]
prerequisite_lessons: [4]
rules_introduced:
  - do-pronoun-postverbal-infinitive
  - yo-pronoun-emphasis
rules_reinforced: [ar-present-yo, no-negation]
transfer_patterns_introduced: []
vocab_introduced: [salvar, situar, participar, crear, experimentar]
anchor_sentences:
  - en: "I don't want to cancel it"
    es: "No quiero cancelar lo"
    drills: [do-pronoun-postverbal-infinitive]
    source: teacher         # teacher | ai-generated | ai-edited
  - en: "I want to inform myself"
    es: "Quiero informar me"
    drills: [do-pronoun-postverbal-infinitive, reflexive-me]
    source: teacher
---

# Lesson 5: Object pronouns stick to "to-verbs"

[Body in the teacher's heuristic voice. Mnemonics dropped unless clearly useful.]
```

Pure transfer-pattern lesson:

```markdown
---
id: 3
title: "Power-up: any English -tion word is free Spanish vocab"
source_refs: ["ReferenceNotes.md#4"]
prerequisite_lessons: [2]
rules_introduced: []
rules_reinforced: []
transfer_patterns_introduced: [suffix-tion-to-cion]
vocab_introduced: []
anchor_sentences:
  - en: "preparation"
    es: "preparacion"
    drills: [suffix-tion-to-cion]
    source: teacher
  - en: "to prepare"
    es: "preparar"
    drills: [suffix-tion-to-cion]
    source: teacher
---

# Lesson 3: -tion words in Spanish end in -cion

[Body explaining the pattern in teacher's voice, with 5–8 example words.]
```

`vocab_introduced` is a flat list of lemmas — no nested metadata. Transfer-pattern lessons have ~3–5 confirmation exercises instead of the standard ~15. Mixed lessons (grammar rules + a transfer pattern) are allowed when material warrants.

## Two-Stage Pipeline

### Stage 1 — Taxonomy + Lesson Plan Pass (per batch)

**Input**
- `ReferenceNotes.md` sections for the batch
- Prior `rules.yaml`, `transfer-patterns.yaml` (empty for batch 1)
- Prior lesson frontmatters (for cumulative vocab and rule coverage)

**Output**
- Proposed additions/edits to `rules.yaml` and `transfer-patterns.yaml`
- Proposed lesson list with `rules_introduced`, `rules_reinforced`, `transfer_patterns_introduced`, `vocab_introduced` per lesson
- `batch-NN-decisions.md` — taxonomy choices, merges, splits, renames with reasoning
- `batch-NN-improvements.md` — packaging divergences with reasoning
- `batch-NN-gaps.md` — rules visible in examples but not stated
- `batch-NN-sequencing-concerns.md` — disagreements with teacher's grammar order (should be rare)

**The Stage 1 prompt must contain**
1. The full schemas above, inlined.
2. **The division-of-authority principle stated verbatim.**
3. **The sequencing constraint stated verbatim** — no silent resequencing of grammar concepts.
4. **Transfer-pattern routing**: suffix/stem transformations go to `transfer-patterns.yaml`, never to `rules.yaml`.
5. **Transfer-pattern interleaving instruction**: interleave transfer-pattern lessons in the first third of the curriculum so each one unlocks vocab for the next few grammar lessons.
6. **Vocab policy**: `vocab_introduced` is a flat list of lemmas — do not invent metadata fields.
7. Instruction to name rules with code-style IDs (lowercase-hyphenated).
8. Category taxonomy (`grammar | phonology | exception-cluster`) as a closed enum.
9. **Style to preserve**: the teacher's heuristic voice survives into lesson bodies.
10. **Style to exclude from structured data**: mnemonics, pedagogical shorthand, phonology tips — body only. Default to dropping mnemonics.
11. **Gap-report requirement.**
12. **Improvement-log requirement.**

**Manual gate:** Read the rule catalog, the transfer-patterns file, the improvements log, and especially the sequencing-concerns file before Stage 2.

### Stage 2 — Lesson Spec Generation (per batch)

**Input**
- Finalized `rules.yaml`, `transfer-patterns.yaml` (frozen for this batch)
- Few-shot lesson spec exemplars (iterated on during Stage 2 prompt development, not hand-authored upfront)
- `ReferenceNotes.md` sections for the batch
- Target lesson list from Stage 1

**Output**
- `lessons/lesson-NN.md` for every lesson in the batch

**The Stage 2 prompt must contain**
1. The lesson spec schema including `source:` on anchors and `transfer_patterns_introduced`.
2. Few-shot exemplars (one grammar, one transfer-pattern), iterated on during prompt development.
3. **Anchor sentence policy**: start from teacher's sentences. Keep unless clearly weak. Replace with justification. Add new AI-generated anchors when coverage is thin.
4. **Body policy**: teacher's heuristic pattern-based voice. Mnemonics default to omitted.
5. **Transfer-pattern lessons get ~3–5 confirmation exercises.**
6. **Vocab policy**: `vocab_introduced` is a flat lemma list — no gloss, no gender, no conjugation. Claude enriches at runtime.
7. Self-check: every rule/transfer-pattern referenced must exist in the corresponding catalog; every prerequisite chain respected.

**Manual gate:** Read the full batch before drafting batch 2's prompts.

## Batch Workflow

```
Batch 1 (teacher sections 1–20)  — PILOT
  Stage 1 → review catalog + transfer-patterns + improvements + sequencing-concerns → edit → freeze
  Draft Stage 2 prompt (few-shot exemplars emerge from prompt iteration, not hand-authored upfront)
  Stage 2 → review all lessons end-to-end → edit
  Retro: what prompt edits does batch 2 need?

Batch 2 (next slice)
  Stage 1 with prior rules.yaml + transfer-patterns.yaml + lesson frontmatters fed back
  …

Continue until curriculum is complete. Total lesson count is AI's call.
```

**Taxonomy stability gate:** if batch 2 requires >20% rule renames or splits, pause and redesign before batch 3.

## Key Assumptions to Validate

- [ ] **A — Clean rule extraction.** Atomic, non-overlapping rules, no hallucinations. *Test: spot-read every rule in batch 1.*
- [ ] **B — Taxonomy stability.** <20% rename/split rate when batch 2 lands.
- [ ] **C — AI respects the sequencing constraint.** `batch-NN-sequencing-concerns.md` is nearly empty.
- [ ] **D — Packaging decisions improve readability without losing voice.** *Test: read batch 1 end-to-end.*
- [ ] **E — Transfer-pattern routing is clean.** AI doesn't misroute between `rules.yaml` and `transfer-patterns.yaml`.
- [ ] **F — Transfer-pattern interleaving actually helps.** Grammar lessons mid-batch have noticeably richer vocab options.
- [ ] **G — ReferenceNotes is complete enough.** `batch-01-gaps.md` is short.
- [ ] **H — Rule constraint is load-bearing at exercise-gen time** (validated later).
- [ ] **I — Compositional bounds hold at exercise-gen time** (validated later).

## MVP Scope (this thread)

**In**
- Schemas defined: rules, transfer patterns, lesson spec.
- Division of authority stated.
- Transfer patterns carved out as a separate packaging category.
- Vocab authored only in lesson frontmatter; runtime bank deferred to layer 2.
- Stage 1 + Stage 2 pipeline at I/O level.
- Batch-of-20 workflow with review gates.
- Next-session handoff: draft the Stage 1 prompt.

**Out of this thread**
- Writing the actual Stage 1 prompt text (next session).
- Writing the Stage 2 prompt text (after batch 1 Stage 1 is reviewed).
- Exercise-generation prompt (separate concern).
- Runtime vocab bank design (lives in `spanish-app-v0.md`).
- Actual generation of any artifact.

## Not Doing (and Why)

- **Silent resequencing of grammar concepts.** Teacher's order is battle-tested.
- **Faithful transcription of the notes.** Packaging is AI's call.
- **Transfer patterns in the rule-constraint graph.** Vocab-expansion machinery, not drill targets.
- **Drilling transfer patterns with full 15-exercise sets.** Light confirmation only.
- **Authoring-time `vocab.yaml`.** Lesson frontmatter is the source of truth; duplicated state drifts.
- **Pre-stored vocab metadata** (gloss, gender, pos, conjugation). Claude derives at runtime; SQLite caches at app layer. No reason to author what Claude already knows.
- **Mnemonics in structured data.** Default dropped; kept only as body prose when clearly useful.
- **Phonology as constrained rules** (h silent, rr rolled, stress). Lives in the body.
- **Hand-writing lesson specs.** Stage 2 generates all of them; few-shot exemplars come out of prompt iteration, not upfront hand-authoring.
- **Full rule catalog upfront.**
- **Fixed lesson count at 60–80.** Fluid — AI's packaging call.
- **Regional variation tracking** (tú/vos, ustedes plurality edge cases). Follow `ReferenceNotes.md` defaults.

## Open Questions

1. **Stage 1 + Stage 2 — separate or merged?** Separate for batch 1. Later batches with frozen catalog may collapse.
2. **Catalog versioning.** Git log probably enough.
3. **Rename policy post-batch.** Free before batch N+1; expensive after. Needs an explicit freeze point.
4. **Exercise-generator coupling.** Schema may need one revision once we try to generate exercises.
5. **What counts as "the teacher's voice"?** Pinned down by the few-shot exemplars that emerge during Stage 2 prompt iteration.
6. **Runtime vocab bank — future unlock.** The architecture already accommodates it (lesson frontmatter + SQLite). Design lives in `spanish-app-v0.md`; built alongside layer-2 "I don't know this word" flow.

## Next Steps

1. **This artifact saved.**
2. **Stage 1 prompt drafted and run for batch 1** (`docs/prompts/stage-1.md`). Catalog frozen.
3. **Next — draft the Stage 2 prompt.** Iterate few-shot exemplars as part of prompt development; do not hand-author gold specs upfront.
4. **After batch 1 completes end-to-end**, retro before touching batch 2.
