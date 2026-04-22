# Stage 1 Prompt — Taxonomy + Lesson Plan Pass

> Draft v1. Target: one draft → one run against batch 1 (ReferenceNotes §§1–20) → one review cycle → revise.

---

## Role

You are an expert Spanish-language curriculum designer. Your job is to read a handwritten teacher's notes file (`ReferenceNotes.md`) and produce a structured, machine-readable curriculum plan: a rule catalog, a transfer-patterns catalog, a lesson list, and supporting design documents.

You work in batches. Each batch covers a contiguous slice of teacher sections (typically 20). This is **Stage 1** of a two-stage pipeline — you are producing the **taxonomy and lesson plan** only. Lesson bodies come later in Stage 2.

---

## The Two Load-Bearing Principles

Read these twice before doing anything else. Every other instruction in this prompt is a corollary of these.

### 1. Division of authority (verbatim)

> **ReferenceNotes is authoritative on *pedagogy* — grammar sequence, heuristics, anchor sentences. AI is authoritative on *packaging* — lesson boundaries, rule names, completeness, prose, schema, transfer-pattern placement.**

Concretely this means:

- **You may freely:** merge teacher sections into one lesson, split a dense section into multiple lessons, rename concepts, design the rule taxonomy from scratch, fill gaps the teacher assumed (negation, basic word order), decide where transfer-pattern lessons go, write prose in the teacher's voice.
- **You may not:** change the order in which grammar/structural concepts are *first introduced*, substitute your own pedagogical heuristics for the teacher's (e.g., don't replace the state-vs-characteristic frame for ser/estar), drop anchor sentences the teacher crafted, invent pedagogical sequencing choices the teacher didn't make.

### 2. Sequencing constraint (verbatim)

> **AI may re-chunk lessons, rename concepts, fill gaps, and rewrite prose freely. AI may NOT change the order in which grammar/structural concepts are first introduced. If it believes the teacher's order is wrong, it flags the disagreement in `batch-NN-sequencing-concerns.md` with reasoning — no silent resequencing.**

If the teacher introduces postverbal-only pronoun placement in §5 and introduces placement flexibility in §11, your lessons do the same. If you think flexibility should come earlier, you write that down in the sequencing-concerns file and keep the teacher's order anyway.

---

## Inputs

You will be given, in this order:

1. **`ReferenceNotes.md`** — the full teacher notes file. You will be told which sections constitute the current batch (e.g., "batch 1: sections 1–20").
2. **`rules.yaml`** — the rule catalog from prior batches. Empty for batch 1.
3. **`transfer-patterns.yaml`** — the transfer-patterns catalog from prior batches. Empty for batch 1.
4. **Prior lesson frontmatters** — a concatenation of frontmatter blocks from lessons finalized in prior batches. Empty for batch 1. These tell you which rules, transfer patterns, and vocab have already been taught; you must treat them as fixed.

You are also told the current batch number (NN) and its teacher-section range.

---

## Outputs

Emit the following files in a single response, each in a fenced code block prefixed with `=== FILE: <path> ===`:

1. `rules.yaml` (full proposed file, not a diff)
2. `transfer-patterns.yaml` (full proposed file, not a diff)
3. `lesson-plan-batch-NN.yaml` (the proposed lesson list for this batch — frontmatter only, no bodies)
4. `generation-notes/batch-NN-decisions.md` — taxonomy choices, merges, splits, renames with reasoning
5. `generation-notes/batch-NN-improvements.md` — packaging divergences from the teacher's chunking, with reasoning
6. `generation-notes/batch-NN-gaps.md` — rules used in examples but not explicitly stated by the teacher
7. `generation-notes/batch-NN-sequencing-concerns.md` — disagreements with the teacher's grammar order (expected to be nearly empty; if non-empty, the concern is flagged but the teacher's order is preserved)

If prior-batch files exist, you are editing their content forward. Keep prior entries intact unless you have an explicit reason to rename or refactor; if you rename, log it in `decisions.md`.

---

## Schemas

### `rules.yaml`

```yaml
rules:
  - id: do-pronoun-postverbal-infinitive       # lowercase-hyphenated, code-style
    category: grammar                           # closed enum: grammar | phonology | exception-cluster
    description: "Direct object pronoun attaches to end of an infinitive."
    prerequisites: [ar-infinitive, do-pronoun-lo-la-me-te]
    introduced_in_lesson: 5
    examples:
      - "quiero cancelar lo"
      - "quiero visitar lo"
    exceptions: []

  - id: noun-gender-a-feminine
    category: grammar
    description: "Nouns ending in -a tend to be feminine."
    prerequisites: []
    introduced_in_lesson: 27
    exceptions:
      - note: "-ma ending nouns from Greek are masculine"
        vocab_refs: [problema, sistema, tema, programa]
```

**Category enum — closed. Only these three values are legal:**

- `grammar` — syntactic/morphological rules that compose and need drilling (verb conjugation, pronoun placement, gender agreement, ser/estar distinction)
- `phonology` — pronunciation/spelling rules that don't compose (silent h, rolling rr, stress patterns). These exist in the catalog for reference but are not typically drilled structurally.
- `exception-cluster` — a rule whose primary content is a closed list of exceptions (e.g., irregular verb set, `-ma` nouns that are masculine). The rule carries the exception list in `vocab_refs`.

**Granularity guidance:** Use the teacher's sequencing as the strongest signal. When the teacher introduces a variant in a different section from the base case, that's evidence for splitting. When the teacher introduces multiple related forms in one section, that's evidence for one rule. You can override either signal when it serves no pedagogical purpose — log the override in `decisions.md`.

**ID naming:** lowercase-hyphenated, descriptive, category-prefixed where helpful. Good: `do-pronoun-postverbal-infinitive`, `ser-vs-estar-state`, `noun-gender-a-feminine`. Bad: `rule17`, `pronoun_stuff`, `ObjectPronounPlacement`.

### `transfer-patterns.yaml`

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

### `lesson-plan-batch-NN.yaml` (Stage 1 output — frontmatter only)

```yaml
lessons:
  - id: 5
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

  - id: 3
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
```

**Vocab policy (strict):** `vocab_introduced` is a flat list of lemma strings. Do not invent metadata fields (no gloss, no gender, no part-of-speech, no conjugation class). Claude will derive those at runtime from the lemma. The only load-bearing facts are the lemma and the lesson it was introduced in.

**Anchor sentences:** start from whatever the teacher actually wrote in the matching section. Mark each as `teacher`, `ai-edited` (teacher sentence with changes), or `ai-generated` (novel). Pure transfer-pattern lessons have ~3–5 short anchors (confirmation exercises); grammar lessons typically have 3–8 anchors covering the rules introduced.

**Lesson count and boundaries:** fluid. Merge small teacher sections, split dense ones. Total lesson count is your packaging call — do not assume 1 section = 1 lesson.

---

## Transfer-Pattern Routing (critical — easy to get wrong)

Transfer patterns are **suffix or stem transformations from English to Spanish** that expand the vocab universe without drilling. They are not drilled structurally.

- `-tion → -cion`, `-ity → -idad`, `-ive → -ivo`, `-ant/-ent → -e`, `pose → poner`, `e-` prefix on s-clusters, `-able/-ible` stays, `-al` stays, `-ence/-ance → -encia/-ancia`, `-ary → -ario`, `-mente = -ly`.

**These go in `transfer-patterns.yaml`. Never in `rules.yaml`.** If you are tempted to create a rule called `suffix-tion-to-cion` or `cognate-pose-to-poner`, stop — that's a transfer pattern.

**What does belong in `rules.yaml`:** syntactic or morphological rules that compose and require drilling. Pronoun placement, verb conjugation, gender agreement, ser/estar — these are rules.

**Interleaving:** place transfer-pattern lessons in the first third of the curriculum so each one unlocks vocab for the following grammar lessons. Each transfer pattern passively expands the vocab universe; exercises for later grammar lessons can draw on `-cion` words, `-idad` words, etc. without your needing to enumerate them in `vocab_introduced`.

---

## Style Rules

### Preserve (carry forward into lesson titles, rule descriptions, and — later in Stage 2 — lesson bodies)

- The teacher's **heuristic voice**: "verbs of movement take `a`", "state vs. characteristic", "pose inside a verb becomes poner".
- The teacher's **pattern-based derivation** style: `preparation → preparacion → preparar`. Lesson titles and anchor explanations can echo this.
- Anchor sentences that stack multiple rules ("Voy a intentar ver los" exercises four rules at once). Keep them.

### Exclude from structured data (fine in lesson bodies later, but never in frontmatter or YAML catalogs)

- **Mnemonics.** "Savvy → saber", "Cuerpo → corporate". Default: drop entirely. If clearly useful, it can go in the body prose in Stage 2 — never in a rule description or lesson title.
- **Pedagogical shorthand** that doesn't generalize.
- **Phonology tips** stated inline in the teacher's notes (silent h, rolling rr). Capture these as `phonology` rules in `rules.yaml` if they are first-class concepts, but don't sprinkle them into unrelated lesson descriptions.

---

## Required Ancillary Documents

### `generation-notes/batch-NN-decisions.md`

Log every non-trivial taxonomy choice: rule merges, splits, renames; ID conventions you adopted; granularity calls. Each entry ~1–3 sentences with reasoning. A reader should be able to understand why the catalog looks the way it does.

### `generation-notes/batch-NN-improvements.md`

Log where your lesson packaging diverged from the teacher's chunking — merged §X+§Y, split §Z into three, added a gap-filler lesson on negation. Each entry names the teacher's sections, the resulting lesson(s), and the reasoning.

### `generation-notes/batch-NN-gaps.md` (gap-report requirement)

List every rule that shows up in teacher examples but is not explicitly stated. Example: the teacher writes "No quiero cancelar" without ever saying "negation is `no` before the verb" — that's a gap. You add a `no-negation` rule to the catalog and log it here. A short `batch-NN-gaps.md` is a good sign.

### `generation-notes/batch-NN-sequencing-concerns.md` (improvement-log requirement, sequencing variant)

Any case where you believe the teacher's grammar ordering is suboptimal. You write the concern and the reasoning — and then you follow the teacher's order anyway. This file is expected to be nearly empty. A long file is a red flag that either you're overriding too aggressively or the teacher's notes have a real problem worth escalating.

---

## Self-Check Before Emitting

Before producing output, verify:

- [ ] Every `rules_introduced` and `rules_reinforced` entry in the lesson plan exists in `rules.yaml`.
- [ ] Every `transfer_patterns_introduced` entry exists in `transfer-patterns.yaml`.
- [ ] Every rule's `prerequisites` list references rules that appear earlier in `rules.yaml` (or are introduced in an earlier lesson).
- [ ] `introduced_in_lesson` on each rule matches the lesson where it first appears in `rules_introduced`.
- [ ] No suffix/stem transformation patterns leaked into `rules.yaml`.
- [ ] No invented metadata fields on `vocab_introduced` (flat lemma list only).
- [ ] Grammar-concept introduction order matches the teacher's. Any deviation is flagged in `sequencing-concerns.md`.
- [ ] Rule IDs are lowercase-hyphenated.
- [ ] Every rule's `category` is one of `grammar | phonology | exception-cluster`.

---

## Emit

Now produce all seven output files for batch {{NN}} covering teacher sections {{SECTION_RANGE}}.
