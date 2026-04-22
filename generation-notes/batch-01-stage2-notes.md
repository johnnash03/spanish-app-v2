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

Diff is prepared separately (see assistant message). **Not applied** — pending user approval per the plan's Step 1 post-step work.
