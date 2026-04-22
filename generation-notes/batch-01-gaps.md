# Batch 01 — Gaps

Rules used in teacher examples but not explicitly stated. For each: whether it was filled as a rule in batch 1, or deferred.

## Filled in batch 1

### `negation-no-before-verb` — filled at L1

**First used:** §2 (`No es normal`).
**Teacher statement:** none. The teacher uses `no` before verbs from the very first sentence and never mentions it.
**Decision:** added as a rule at L1. Placement of `no` directly before the conjugated verb is a structural constraint that many later rules reinforce (`no me lo venden`, `no quiero cancelar`, etc.); not having it in the catalog would force every downstream rule to hand-wave its examples.

### `article-el-la-basic` — filled at L9

**First used:** §8 (`a la casa`). `las casas` at §12; `el coche` at §27 (batch 2).
**Teacher statement in batch 1:** none. The teacher uses `la casa` without explaining articles or gender.
**Teacher's eventual statement:** §23 introduces `-a feminine` rule; §27 introduces adjective agreement.
**Decision:** added a minimal `article-el-la-basic` rule at L9, stating only the four forms (el / la / los / las) and that they agree in gender and number. This is the *minimum* needed for learners to parse `la casa`. The *full* `-a feminine` rule (including the `-ma` exceptions) is left for batch 2 at the teacher's §23. See `batch-01-sequencing-concerns.md`.

## Not filled (accepted as pedagogical style)

### Subject-verb inversion not needed for questions

**First used:** §7 (`Por que no quiere visitar me` as a question). §8, §13 have more.
**Teacher statement:** the teacher treats questions as unmarked — no "do" auxiliary, no inversion — but never explicitly says this. Examples like `Quiere venir a la casa` (statement or question based on context) demonstrate it.
**Decision:** not promoted to a rule. The absence-of-a-rule is the rule. Explicitly stating "Spanish has no subject-verb inversion or auxiliary in questions" would be overhead when all the teacher's examples just work. Flagging here in case exercise generation needs to disambiguate "¿?" intonation later.

### Subject pronouns are optional

**First used:** §2 (`Es ilegal` with no `el`/`ello`). §4 explicitly mentions: "You don't need the word for he if you know who you're talking about."
**Decision:** this IS mentioned by the teacher (§4) but casually. Covered under `pronoun-yo-emphasis` at L5 which describes `yo` as optional-for-emphasis; the general optionality extends to `el`, `ella`, etc. I did not add a standalone `subject-pronoun-optional` rule because it would duplicate what `pronoun-yo-emphasis` already encodes. Noting here in case a reviewer thinks it should be its own rule.

## Deferred (will appear in a later batch)

### `-a feminine` rule

**First used in batch 1:** §7 (`a la casa`), §12 (`las casas`).
**Teacher's formal statement:** §23 ("Words ending -a are feminine but words ending -ma are masculine"), with the Greek-origin exception cluster.
**Decision:** left for batch 2. The gap is partially filled by `article-el-la-basic` at L9 (just enough to parse the noun phrase); the full rule with its `-ma` exception list lands at the teacher's §23.

### Adjective agreement / adjective position

**First used:** §27 (outside batch 1).
**Decision:** not a gap — teacher introduces it in batch 2. Flagging for completeness so the next batch knows to expect this rule.

### Ser conjugation

**First used in batch 1:** §2 (`es`), recurring throughout.
**Teacher's formal statement:** §31 (four -oy verbs: `voy, doy, estoy, soy`).
**Decision:** `es` is used as a memorized chunk in batch 1 (no ser-conjugation rule active). The lemma `ser` is in `vocab_introduced` at L1. Full conjugation and ser-vs-estar contrast are batch 2 material.

### Estar conjugation and the ser-vs-estar contrast

**First used in batch 1:** not used directly in batch 1 (§29 introduces it).
**Decision:** batch 2.

## Summary

- **Two rules filled** (`negation-no-before-verb`, `article-el-la-basic`). Both minimal, both in the lesson where the concept first surfaces.
- **Two stylistic non-rules flagged** (no inversion, subject pronoun optionality). Not worth elevating to rules.
- **Three rules deferred to batch 2** (`-a feminine`, adjective agreement, ser/estar). All will arrive at the teacher's own sequencing point.

Short gap list = the teacher's notes are close to self-contained for batch 1, modulo the `la casa` article issue. That validates assumption G ("ReferenceNotes is complete enough") in the spec, at least for this batch.
