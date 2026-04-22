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
