# Batch 01 — Taxonomy Decisions

Design log for the rule catalog, transfer-patterns catalog, and their conventions in batch 1.

## Naming conventions adopted

- **Lowercase-hyphenated IDs** throughout.
- **Category-prefixed IDs** where it aids readability: `verb-*`, `pronoun-*`, `noun-*`, `article-*`, `preposition-*`, `modal-*`, `interrogative-*`, `conjunction-*`, `phonology-*`, `suffix-*` (transfer only), `stem-*` (transfer only).
- **Tense/feature suffixes** on verb rules: `verb-present-yo-regular`, `verb-present-third-singular-ar`, `verb-present-nosotros-ir`. Gives future tenses a clear extension pattern (`verb-preterite-*`, `verb-imperfect-*`, etc.).
- **Irregular-verb rules** carry the verb lemma in the ID: `verb-irregular-ir`, `verb-irregular-ver`, `verb-irregular-saber-yo-se`. Yo-go verbs form a single cluster (`verb-irregular-yo-go`) since they share a pattern; individual yo-go verbs (tener, venir, poner, salir) are in its `vocab_refs` list rather than separate rules.
- **Modal rules** include the canonical yo form: `modal-quiero-plus-infinitive`, `modal-debo-plus-infinitive-no-que`, `modal-puedo-plus-infinitive`, `modal-tengo-que-plus-infinitive`. The presence-or-absence of `que` is encoded in the ID to flag the one quirky case (`tengo que` vs. `debo`).

## Granularity — split vs. merge decisions

### Split: regular conjugation skeleton (L12)

§11 introduces yo, third-person-singular (-ar), and third-person-plural forms together. I split this into three separate rules (`verb-present-yo-regular`, `verb-present-third-singular-ar`, `verb-present-third-plural-add-n`) rather than one `verb-present-regular-ar`.

**Reason:** each form composes independently with other rules (e.g., pronoun placement, stem changes, negation). Exercise generation will need to drill a specific form (e.g., "conjugate only the yo form for these -er verbs") and atomic rules make that constraint natural.

### Split: stem changes into e→ie and o→ue (L17 and L18)

The teacher introduces both in §15–16 as one phenomenon ("splits to add stress"). I split into two rules because the vocabulary sets are disjoint and learners don't derive the `ie` vowel from the `ue` vowel. Splitting also makes the `vocab_refs` lists cleaner.

### Split: nosotros -ar, -er, -ir (L19, L20)

§17 and §18 together cover all three. Since -ar comes before -er/-ir in teacher order and the "we-form doesn't split" rule pairs naturally with -ar verbs (the stem-change examples in §17 are all -ar), I split: L19 = -amos + no-split-in-we rule. L20 = -emos / -imos.

### Merge avoided: 'a' for motion before infinitive vs. before destination noun

`verb-motion-plus-a-before-infinitive` (§7) and `preposition-a-motion-destination` (§8, a la casa) are related but not the same rule — they apply to different syntactic slots. Kept separate. Learner can produce "Voy a casa" with the second rule only, or "Voy a comer" with the first only. Mixing them ("Voy a la casa a comer") combines both.

### Merge: yo-go cluster into one exception-cluster rule

`tengo`, `vengo`, `pongo`, `salgo` all share a pattern: yo form ends in -go with no other irregularity. One rule with `vocab_refs: [tener, venir, poner, salir, ...]` rather than four separate rules. When more -go verbs appear (e.g., `digo`, `hago`), they get added to the same rule's vocab_refs.

## Transfer vs. rule routing

Every suffix/stem transformation went to `transfer-patterns.yaml`. No borderline calls:
- `-tion → -cion` with the derived verb: transfer (one-shot mapping).
- `stem-pose-to-poner`: transfer (verb stem swap; the resulting verb's conjugation is governed by separate rules).
- `-mente = -ly`: transfer (even though it feels morphological).

The morphological rules that went to `rules.yaml` were those that compose (third-person plural adds -n to the third-singular stem) or are closed-form (irregulars) or are phonological. Nothing in between.

## Categories

All three category values got used:
- **grammar**: 33 rules. Vast majority.
- **phonology**: 4 rules. `phonology-ja-to-kh`, `phonology-rr-rolled`, `phonology-h-silent`, `phonology-stress-accent-default`. These are reference-only — not drilled as primary structure.
- **exception-cluster**: 4 rules. `verb-irregular-yo-go`, `verb-irregular-saber-yo-se`, `verb-irregular-ir`, `verb-irregular-ver`. Each carries `vocab_refs` naming the verbs it governs.

No rule straddled categories.

## Prerequisites — minimality

Each rule lists only its immediate logical prerequisites, not the transitive closure. Example: `pronoun-compound-indirect-before-direct` lists `do-pronoun-basic` and `pronoun-nos-us`, not every pronoun rule that came before. The spec says lessons inherit the transitive closure, so the DAG handles the rest.

## Noun-gender: what got introduced in batch 1 vs. batch 2

The teacher introduces `-ion feminine` explicitly at §12 and `-e context-dependent` at §19. The general `-a feminine` rule appears at §23 (outside batch 1). Rather than introducing `-a feminine` early (which would violate sequencing), I added a minimal `article-el-la-basic` filler at L9 so learners can parse `la casa` — but the full `-a feminine` rule is left for batch 2. See `batch-01-gaps.md` for the gap-fill reasoning.

## Vocab policy

Strictly flat lemma lists. No metadata fields added anywhere. Pronouns (`yo`, `lo`, `me`, `te`, `nos`) are included in `vocab_introduced` as lemmas in their own right (they are lexical items the learner encounters) even though they aren't verbs.

Irregular verb forms (`voy`, `quiero`, `quiere`, `tengo`, `vengo`, `se`, `pongo`, `supongo`, `salgo`, `veo`) are NOT separate vocab entries — the lemma is what matters. Exercise generation will produce the irregular form from the lemma + the relevant rule.
