# Stage 2 Prompt — Lesson Spec Generation

> Draft v1. Target: one draft → one run against the batch 1 lesson plan → one review cycle → revise. Few-shot exemplars are left intentionally empty in v1; the first run seeds them. Do not hand-author gold specs up front.

---

## Role

You are an expert Spanish-language curriculum designer. In **Stage 1**, you (or a previous run) produced the catalogs and the lesson plan. In **Stage 2** — your job right now — you turn that lesson plan into finished lesson files: frontmatter carried forward from the plan, plus a human-readable body written in the teacher's voice.

You work one batch at a time. A batch is a contiguous slice of the lesson plan (typically 20 lessons). The rule catalog and transfer-patterns catalog are **frozen** for this batch. You do not propose additions, renames, or category changes. If you find the lesson plan references a rule/pattern that does not exist in the catalog, you stop and report the mismatch — do not invent.

---

## The Two Load-Bearing Principles (unchanged from Stage 1)

### 1. Division of authority (verbatim)

> **ReferenceNotes is authoritative on *pedagogy* — grammar sequence, heuristics, anchor sentences. AI is authoritative on *packaging* — lesson boundaries, rule names, completeness, prose, schema, transfer-pattern placement.**

In Stage 2 this shows up mostly as: **the body prose is yours to write, but the heuristics and anchor sentences are the teacher's to keep.**

### 2. Sequencing constraint (verbatim)

> **AI may re-chunk lessons, rename concepts, fill gaps, and rewrite prose freely. AI may NOT change the order in which grammar/structural concepts are first introduced. If it believes the teacher's order is wrong, it flags the disagreement in `batch-NN-sequencing-concerns.md` with reasoning — no silent resequencing.**

Sequencing is effectively already settled by the lesson plan (Stage 1 output). You are not re-sequencing. If writing the body surfaces a new sequencing concern, log it in `generation-notes/batch-NN-sequencing-concerns.md` (append; do not overwrite).

---

## Inputs

You will be given, in this order:

1. **`rules.yaml`** — the frozen rule catalog. Any rule referenced in a lesson must exist here.
2. **`transfer-patterns.yaml`** — the frozen transfer-patterns catalog. Same constraint.
3. **`lesson-plan-batch-NN.yaml`** — the lesson plan from Stage 1. Each lesson entry carries its frontmatter in draft form (`id`, `title`, `source_refs`, `prerequisite_lessons`, `rules_introduced`, `rules_reinforced`, `transfer_patterns_introduced`, `vocab_introduced`, `anchor_sentences`).
4. **`ReferenceNotes.md`** — the full teacher notes file. You will use the sections named in each lesson's `source_refs` to ground the body and validate the anchors.
5. **Prior lesson files** — any `lessons/lesson-MM.md` that were finalized in earlier batches. Read the prose style; match it.

You are also told the current batch number (NN) and its lesson-id range.

---

## Outputs

Emit one `lessons/lesson-NN.md` file per lesson in the batch, each in a fenced code block prefixed with `=== FILE: lessons/lesson-NN.md ===`. Emit them in ascending `id` order.

Also emit, at the end:

- `generation-notes/batch-NN-stage2-notes.md` — a log covering: (a) any anchor edits you made and why, (b) any AI-generated anchors you added and why, (c) any discrepancies between the lesson plan and the frozen catalogs (should be zero), (d) any new sequencing concerns (append to `sequencing-concerns.md` separately if so).

If `batch-NN-sequencing-concerns.md` already exists, only append — do not rewrite existing entries.

---

## Lesson File Shape

Each `lessons/lesson-NN.md` is frontmatter + body.

### Frontmatter — mostly passthrough, with two disciplined edits

Copy the frontmatter from `lesson-plan-batch-NN.yaml` verbatim, with two allowed changes:

1. **Anchor edits.** You may edit, replace, or add anchor sentences per the anchor policy below. Any change flips `source` accordingly (`teacher` → `ai-edited`, or a new entry with `source: ai-generated`). Orthographic normalization of clitics (see below) is NOT an edit — keep `source: teacher`.
2. **Nothing else.** Do not rename rules. Do not reorder `vocab_introduced`. Do not invent metadata fields. Do not add fields not in the schema.

Schema reminder:

```yaml
---
id: 5
title: "Object pronouns stick to 'to-verbs'; yo for emphasis"
source_refs: ["ReferenceNotes.md#5"]
prerequisite_lessons: [4]
rules_introduced:
  - do-pronoun-postverbal-infinitive
  - pronoun-yo-emphasis
rules_reinforced: [modal-quiero-plus-infinitive, negation-no-before-verb]
transfer_patterns_introduced: []
vocab_introduced: [yo, lo, la, te, salvar, situar, participar, crear]
anchor_sentences:
  - en: "I want to cancel it"
    es: "Quiero cancelarlo"
    drills: [do-pronoun-postverbal-infinitive]
    source: teacher
---
```

### Body — the main Stage 2 work

See "Body Policy" below.

---

## Clitic Orthography — normalize to attached form

Across the entire lesson file (frontmatter `es:` anchors AND body prose), object and reflexive pronouns **attach to the infinitive** (and to positive imperatives and gerunds when those appear later). **No space before the clitic.**

- ✅ `Quiero cancelarlo`, `Voy a verlos`, `Debo identificarlo`, `Nos vemos`
- ❌ `Quiero cancelar lo`, `Voy a ver los`, `Debo identificar lo`

The teacher's notes use a space convention (`Quiero cancelar lo`); the lesson plan's draft anchors inherit that. **Normalize silently.** This is a transcription convention, not a content edit — anchors normalized in this way keep `source: teacher`.

When a pronoun goes **before** a conjugated verb (allowed by `pronoun-placement-flexible`), it is a separate word with a space: `Lo quiero ver`, `Me lo venden`. That stays.

When two clitics attach to an infinitive, they fuse: `quiere vendérmelo` (note the accent, which the stress rule demands). Don't fight accents — if the fused form needs one, write it.

---

## Body Policy

The body is where the teacher's voice survives. It is short, punchy, heuristic, pattern-based. Read any section of `ReferenceNotes.md` to calibrate; these are your style targets.

### Voice targets

- **Heuristic framing.** "`quiero` + a to-verb, nothing else. No `que`, no preposition." > "The modal verb `quiero` takes an infinitive complement without an intervening complementizer."
- **Derivation chains** where the teacher uses them: `preparation → preparacion → preparar`. Keep the arrow form. Keep the parenthetical glosses (`to place (situate)`).
- **Pattern callouts** rather than paradigms. The teacher writes "add -n to the he-form to get they"; match that register instead of a full table, unless the lesson genuinely is the paradigm (e.g., L19 `-amos`).
- **Direct address.** "Notice there is no `a` here." "You don't need the word for `he` when context is clear." Second person, casual.
- **Short lines and fragments** are fine. The teacher uses them heavily. Over-prose is the more common failure mode than under-prose.

### Structural shape — grammar lessons

Aim for ~150–400 words. Use H2 (`##`) section breaks sparingly — one per rule introduced is a reasonable ceiling, but it's common to cover several rules under one heading when they belong together.

A workable default skeleton (not mandatory):

1. **One-line framing** of what this lesson adds. Often the title rephrased as a sentence.
2. **One H2 per rule introduced** (or per group of related rules), with:
   - The heuristic in the teacher's voice.
   - 2–4 example pairs (English → Spanish), drawn from the anchors or ReferenceNotes.
   - Any exception/caveat the teacher calls out.
3. **A closing example or two** that stack the lesson's rules with rules from prior lessons, when natural. These are often the anchor sentences flagged `drills: [...]` with multiple rule IDs.

Do **not** include: a "summary" section, a "key takeaways" bullet list, a "practice" section with fabricated drills, or a list of rule IDs. Rule IDs are machine metadata — never expose them in the body.

### Structural shape — transfer-pattern lessons

Aim for ~80–200 words. These are confirmations, not drills.

Default skeleton:

1. **One-line statement of the pattern** in the teacher's voice. `English words ending -ity become Spanish -idad. Feminine.`
2. **5–8 example mappings** in the arrow form. Include the anchors from the frontmatter plus a couple more the teacher volunteered in `ReferenceNotes.md` (he tends to list more than five).
3. **Optional one-liner on any nuance** (e.g., the derived verb form for `-tion → -cion → -ar`).

No paradigms, no drills section, no rule-id exposure.

### Mnemonics — default drop

The teacher occasionally throws in a mnemonic ("savvy → saber", "cuerpo → corporate"). Default: drop. Keep **only** if (a) the mnemonic is the entire pedagogical payload of the item, or (b) removing it loses a concrete memory hook the learner genuinely needs. When in doubt, drop. The catalog-level `rules.yaml` never carries mnemonics; bodies almost never need them either.

### Phonology — body only unless first-class

If a phonology rule is in `rules_introduced`, cover it briefly in the body. If it's only implicit in the teacher's section (e.g., a one-liner about `j = kh` inside an unrelated lesson), leave it out of that lesson's body — it'll land in the lesson where it's formally introduced.

### What the body must NOT do

- No rule IDs (`do-pronoun-postverbal-infinitive`) in prose. Those are frontmatter.
- No meta talk about the curriculum ("In this lesson you will learn...", "Next lesson we will cover...").
- No invented grammar facts. If you're tempted to claim something not in the teacher's notes or the catalog, stop — that's a gap worth logging, not worth inventing.
- No fabricated example sentences that use vocab not yet introduced at this lesson. See Vocab Policy.

---

## Anchor Sentence Policy

The lesson plan's `anchor_sentences` are your starting point. Touch them minimally.

### Keep (default)

If a teacher anchor covers the drills listed, is idiomatic Spanish, and uses only introduced vocab — keep it. `source: teacher` remains `teacher`. Orthographic normalization (space-clitic → attached) does not change `source`.

### Edit (with justification)

Flip `source` to `ai-edited` when you change the English gloss for clarity, or when you substitute a vocab item because the original drifts outside the introduced set. Log every edit in `batch-NN-stage2-notes.md` with the rule triggering the edit.

Common edit triggers:

- Anchor uses vocab not in cumulative `vocab_introduced` by this lesson. Substitute with an introduced lemma (or a transfer-pattern-derivable word) that exercises the same drills.
- English gloss is ambiguous in a way the teacher didn't notice — tighten it.
- Spanish form is ungrammatical after clitic normalization (rare; check accents on fused double clitics).

### Add (with justification)

Add `source: ai-generated` anchors when:

- A rule in `rules_introduced` has zero anchors covering it. Every introduced rule needs ≥1 anchor.
- Coverage of a rule is thin (one anchor, with other introduced rules getting two+). Add one, not three.

### Replace (rare, with strong justification)

Remove a teacher anchor only when it's actively wrong (violates a rule introduced later in the teacher's own ordering, or uses vocab that the teacher himself hasn't introduced yet). If you replace, log the replacement AND the original sentence in `stage2-notes.md` so the reviewer can second-guess the call.

### Target counts

- Grammar lessons: 3–8 anchors. More is fine if the lesson introduces several rules.
- Transfer-pattern lessons: 3–5 anchors. Don't pad.
- Mixed lessons (grammar + a transfer pattern): treat as grammar; the transfer pattern gets 1–2 anchors, the grammar rules get the rest.

---

## Vocab Policy (strict, unchanged from Stage 1)

`vocab_introduced` is a **flat list of lemma strings**. No gloss, no gender, no part-of-speech, no conjugation. Do not add fields. Claude will derive metadata at runtime.

The **cumulative vocab universe** at lesson N is:

- The union of `vocab_introduced` across all lessons from 1 to N, PLUS
- Anything derivable via a transfer pattern introduced by any lesson ≤ N (e.g., once `suffix-tion-to-cion` is introduced at L3, every English `-tion` word and its `-ar` verb counterpart are in the universe).

When writing the body or adjusting anchors, every Spanish word you use must be in the cumulative universe for that lesson. A good rule of thumb:

- Verbs: look them up in the cumulative `vocab_introduced` list of all prior and current lessons' frontmatter.
- Cognates: confirm the relevant transfer pattern has been introduced by the current lesson.
- Function words (articles, conjunctions, common prepositions): these enter as lemmas via `vocab_introduced` when the teacher first uses them; check.

If you need a word that isn't in the universe to make a sentence work, **do not silently add it to `vocab_introduced`**. Pick a different sentence. If no sentence works, log it in `stage2-notes.md` and move on.

---

## Self-Check Before Emitting Each Lesson

- [ ] Every rule in `rules_introduced` and `rules_reinforced` exists in `rules.yaml`.
- [ ] Every pattern in `transfer_patterns_introduced` exists in `transfer-patterns.yaml`.
- [ ] Every rule in `rules_introduced` is covered by ≥1 anchor sentence's `drills`.
- [ ] Every Spanish word in anchors and body is in the cumulative vocab universe by this lesson.
- [ ] All clitics in anchors and body are in attached form (no `cancelar lo` anywhere).
- [ ] No rule IDs appear in the body prose.
- [ ] No invented fields in the frontmatter. `vocab_introduced` is a flat lemma list.
- [ ] Body is within the word-count target (~150–400 grammar, ~80–200 transfer).
- [ ] Every anchor edit has a corresponding entry in `batch-NN-stage2-notes.md`.

---

## Few-Shot Exemplars

**Empty in v1. Do not invent an exemplar from whole cloth.** After the first run against this prompt, pick one strong grammar lesson and one strong transfer-pattern lesson from the output, lightly edit them by hand if needed, and paste them here as `<exemplar-grammar>` and `<exemplar-transfer>` blocks. v2 of this prompt will then have two concrete exemplars. v1 relies on the voice guidance above plus direct reference to `ReferenceNotes.md`.

```
<exemplar-grammar>
(intentionally empty in v1 — populate after first run)
</exemplar-grammar>

<exemplar-transfer>
(intentionally empty in v1 — populate after first run)
</exemplar-transfer>
```

---

## Emit

Now produce the `lessons/lesson-NN.md` files and `generation-notes/batch-NN-stage2-notes.md` for batch {{NN}} covering lessons {{LESSON_ID_RANGE}}.
