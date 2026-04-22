# Batch 01 — Stage 2 Notes

Log of anchor edits, AI-generated additions, clitic normalizations, catalog discrepancies, and deliberate self-check exceptions during Stage 2 generation. Appended per step.

---

## Step 1 — Pilot (L1–L3)

### Anchor edits
None. Every teacher anchor in the L1–L3 slice of `batch-01-lesson-plan.yaml` passed through verbatim.

### AI-generated anchors added
None. All rules/patterns with an anchorable surface (`suffix-al-stays`, `negation-no-before-verb`, `suffix-ant-ent-to-e`, `suffix-ible-able-stays`, `suffix-mente-equals-ly`, `suffix-tion-to-cion`) already had ≥1 teacher anchor covering them.

### Clitic normalizations
None applicable. L1–L3 anchors contain no object or reflexive pronouns — nothing to normalize. First clitic appears in L4 (`Quiero preparar me` → `Quiero prepararme`), so the attach-no-space convention will first bite in step 2.

### Discrepancies with frozen catalogs
None. Every rule ID in `rules_introduced` / `rules_reinforced` and every pattern ID in `transfer_patterns_introduced` across L1–L3 exists in `rules.yaml` / `transfer-patterns.yaml`.

### Deliberate self-check exception — phonology rule without anchor

**L2 — `phonology-ja-to-kh` is in `rules_introduced` but no anchor sentence lists it in `drills`.**

Reason: at L2 the cumulative vocab universe (ser, no, plus any -al / -ant / -ent / -ible / -able / -mente derivative) contains no natural `j`, `ge`, or `gi` example that the teacher himself volunteered in §3. The teacher's §3 mention is a bare one-liner — "ja sound becomes kh" — without an accompanying drillable Spanish word.

Two options considered:

1. **Force-add an AI-generated anchor** using `general` (an -al cognate whose `ge-` exercises the rule as cataloged). Rejected because the catalog extension to `ge`/`gi` goes beyond what the teacher stated in §3; drilling it here would outrun the teacher's pedagogy.
2. **Body-only coverage.** Taken. The stage-2 prompt's "Phonology — body only unless first-class" guidance directly permits this; the rule is stated in the L2 body in the teacher's register. The "≥1 anchor per introduced rule" self-check is relaxed in this specific case for phonology rules with no drillable vocab in scope.

A cleaner long-term fix would be a small catalog/plan change: either (a) defer `phonology-ja-to-kh`'s `introduced_in_lesson` to the first lesson where a `j`/`ge`/`gi` word enters vocab naturally, or (b) split the rule so the `ja` half introduces in L2 (body-only is fine) and the `ge`/`gi` half introduces later. **Not doing that now** — the catalog is frozen for this batch. Flagging for the batch-01 retro.

### New sequencing concerns
None.

### Word-count check

| Lesson | Body words (approx.) | Target | Status |
|-------:|---------------------:|:------:|:------:|
| L1     | ~170                 | 150–400 (grammar/mixed) | ok |
| L2     | ~140                 | 80–200 (transfer-leaning) | ok |
| L3     | ~90                  | 80–200 (transfer) | ok |

### Proposed exemplars for `docs/prompts/stage-2.md`

- `<exemplar-grammar>`: **L1** — smallest possible grammar lesson; introduces `negation-no-before-verb` as a gap-fill riding alongside a transfer pattern. Shows the body shape when a grammar rule enters light.
- `<exemplar-transfer>`: **L3** — cleanest possible transfer-pattern lesson; no grammar, one pattern, arrow-form derivations. Shows the body shape for pure transfer.

Exemplars were subsequently applied to `docs/prompts/stage-2.md` prior to step 2 execution.

---

## Step 2 — Early grammar (L4–L8)

### Anchor edits

**L8 — "I want to show you something"**: changed `drills` from `reflexive-pronoun-attach-infinitive` to `do-pronoun-postverbal-infinitive` (this sentence uses `te` as an object pronoun, not reflexive). Marked `source: ai-edited`.

### AI-generated anchors added

- **L5**: added `horrible` to drill `phonology-rr-rolled` using an in-universe example (derivable via the earlier `-ible` transfer pattern).
- **L7**: added `horrible` to drill `phonology-h-silent` (teacher-mentioned example; missing from the lesson plan anchors).

### Clitic normalizations

Normalized space-clitic spellings to attached form (kept `source: teacher`):

- **L4**: `Quiero preparar me` → `Quiero prepararme`
- **L5**: `Quiero cancelar lo` → `Quiero cancelarlo`; `No quiero obligar te` → `No quiero obligarte`; `Quiero informar me` → `Quiero informarme`
- **L7**: `Intento publicar lo` → `Intento publicarlo`; `Intento continuar lo` → `Intento continuarlo`; `Quiero visitar lo` → `Quiero visitarlo`
- **L8**: `Quiero ver los` → `Quiero verlos`; `Voy a ver los` → `Voy a verlos`; `Voy a intentar ver los` → `Voy a intentar verlos`; `Voy a pasar a visitar te` → `Voy a pasar a visitarte`; `Quiero mostrar te algo` → `Quiero mostrarte algo`

### Discrepancies with frozen catalogs

None. All rule IDs and transfer-pattern IDs referenced in L4–L8 exist in `rules.yaml` and `transfer-patterns.yaml`.

### Deliberate self-check exception — `a` not listed in `vocab_introduced`

The lesson plan introduces `verb-motion-plus-a-before-infinitive` at L8, and every anchor necessarily contains the token `a`, but `a` is not present anywhere in `vocab_introduced` through this slice. Treated `a` as a structural marker required by the introduced rule (not a content-vocab choice) and proceeded without retroactively adding it to the vocab lists (schema forbids that during Stage 2).

### New sequencing concerns

None.

### Word-count check

| Lesson | Body words | Target | Status |
|-------:|----------:|:------:|:------:|
| L4     | 166       | 150–400 (grammar) | ok |
| L5     | 176       | 150–400 (grammar) | ok |
| L6     | 94        | 80–200 (transfer) | ok |
| L7     | 169       | 150–400 (grammar) | ok |
| L8     | 184       | 150–400 (grammar) | ok |

---

## Step 3 — Mid grammar (L9–L14)

### Anchor edits

**L13**: added anchors to cover two introduced rules that had zero teacher anchors:

- `verb-present-third-singular-er`: added `Lo vende` (`source: ai-generated`)
- `noun-gender-ion-feminine`: added `la administracion` (`source: ai-generated`)

**L11**: added anchors so every introduced rule has at least one anchor:

- `verb-infinitive-ending-r`: `comer` (`source: ai-generated`)
- `verb-irregular-yo-go`: `Tengo` (`source: ai-generated`)
- `phonology-stress-accent-default`: `intentar / intento` (`source: ai-generated`)

### AI-generated anchors added

All AI-generated anchors for this step are listed under "Anchor edits" (above).

### Clitic normalizations

Normalized space-clitic spellings to attached form (kept `source: teacher`):

- **L9**: `Quiere visitar me` → `Quiere visitarme`; `Por que no quiere visitar me` → `Por que no quiere visitarme`; `Debo identificar lo` → `Debo identificarlo`
- **L10**: `... a visitar me` → `... a visitarme`; `No quiero imaginar lo` → `No quiero imaginarlo`
- **L12**: `Quiero ver lo` → `Quiero verlo`; `Debo invitar lo` → `Debo invitarlo`

### Discrepancies with frozen catalogs

None. All rule IDs and transfer-pattern IDs referenced in L9–L14 exist in `rules.yaml` and `transfer-patterns.yaml`.

### Deliberate self-check exception — closed-class words not tracked in vocab lists

Across this slice, multiple anchors necessarily include high-frequency closed-class words that are not present in the cumulative `vocab_introduced` lists (e.g., `a`, `la/las`, `por`, `que`, `si`, `y`). These are treated as structural/functional tokens required by introduced grammar rules and by the teacher's anchors, and are not retroactively added to the vocab lists during Stage 2.

Proper nouns (e.g., `Pablo`) are treated the same way.

### New sequencing concerns

None.

### Word-count check

| Lesson | Body words | Target | Status |
|-------:|----------:|:------:|:------:|
| L9     | 176       | 150–400 (grammar) | ok |
| L10    | 166       | 150–400 (grammar) | ok |
| L11    | 227       | 150–400 (grammar) | ok |
| L12    | 163       | 150–400 (grammar) | ok |
| L13    | 187       | 150–400 (grammar/mixed) | ok |
| L14    | 157       | 150–400 (grammar/mixed) | ok |

---

## Step 4 — Stem changes + modals + pose-to-poner (L15–L18)

### Anchor edits

**L15 — "I don't want to invite him but she does"**: added explicit `Yo` for the intended emphasis drill and normalized the clitic (`invitarlo`). Marked `source: ai-edited`.

**L18 — "Me duermo"**: changed `drills` from `reflexive-pronoun-attach-infinitive` to `pronoun-placement-flexible` (this is a one-verb sentence; the pronoun sits before the conjugated verb, not attached to an infinitive). Marked `source: ai-edited`.

### AI-generated anchors added

None.

### Clitic normalizations

Normalized space-clitic spellings to attached form (kept `source: teacher` unless otherwise noted):

- **L15**: `No quiero invitar lo ...` → `... invitarlo ...` (also added `Yo` as an explicit emphasis cue; see Anchor edits); `Tengo que hacer lo` → `Tengo que hacerlo`; `Debo ver la / Tengo que ver la` → `Debo verla / Tengo que verla`

### Discrepancies with frozen catalogs

None. All rule IDs and transfer-pattern IDs referenced in L15–L18 exist in `rules.yaml` and `transfer-patterns.yaml`.

### Deliberate self-check exception — closed-class words not tracked in vocab lists

Same as Step 3: teacher anchors contain closed-class tokens (e.g., `que`, `a`, `la/las`, `ella`) that are not consistently present in `vocab_introduced` lists. These were not retroactively added during Stage 2.

### New sequencing concerns

None.

### Word-count check

| Lesson | Body words | Target | Status |
|-------:|----------:|:------:|:------:|
| L15    | 171       | 150–400 (grammar) | ok |
| L16    | 95        | 80–200 (transfer) | ok |
| L17    | 193       | 150–400 (grammar) | ok |
| L18    | 153       | 150–400 (grammar) | ok |

---

## Step 5 — Nosotros + irregulars + future (L19–L22)

### Anchor edits

None beyond clitic normalization (see below).

### AI-generated anchors added

**L21**: added two anchors to ensure `noun-gender-e-context-dependent` has explicit anchor coverage with an article:

- `la noche` (`source: ai-generated`)
- `la carne` (`source: ai-generated`)

### Clitic normalizations

Normalized space-clitic spellings to attached form (kept `source: teacher`):

- **L22**: `Vamos a ver la` → `Vamos a verla`; `Vas a ver nos pronto` → `Vas a vernos pronto`

### Discrepancies with frozen catalogs

None. All rule IDs and transfer-pattern IDs referenced in L19–L22 exist in `rules.yaml` and `transfer-patterns.yaml`.

### Deliberate self-check exception — closed-class words not tracked in vocab lists

Same as Steps 3–4: teacher anchors contain closed-class tokens (e.g., `la`, `que`, `y`, `a`, `donde`) that are not consistently present in `vocab_introduced` lists. These were not retroactively added during Stage 2.

### New sequencing concerns

None.

### Word-count check

| Lesson | Body words | Target | Status |
|-------:|----------:|:------:|:------:|
| L19    | 178       | 150–400 (grammar) | ok |
| L20    | 158       | 150–400 (grammar) | ok |
| L21    | 166       | 150–400 (grammar) | ok |
| L22    | 172       | 150–400 (grammar/mixed) | ok |
