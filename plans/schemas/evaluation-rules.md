# Evaluation Rules
*Pre-dev artifact #3 — Tier 1 Foundation*

Defines what "correct" means. Must be settled before any prompt or logic references correctness.
Evaluation is LLM-based — these rules are injected into the evaluator prompt (artifact #13).

---

## Evaluator Input Contract

The evaluator LLM receives:
- `source` — the English cue shown to the learner
- `canonical` — the correct Spanish answer
- `primaryTag` — the skill being drilled, with a brief description of what it tests
- `stackedTags` — prior skills mixed into the item, each with a brief description
- `learnerAnswer` — what the learner submitted

## Evaluator Output Shape

```typescript
interface EvaluationResult {
  correct: boolean;
  errorTag: string | null;  // one of primaryTag or stackedTags; null if correct
  remarks: string[];        // informational notes surfaced to the learner post-answer
}
```

---

## Correctness Rules

### Accents
Always lenient — an answer is never marked wrong solely due to a missing or incorrect accent.
Always remark when an accent is wrong or missing, explaining the difference (e.g. "Note: `si` means 'if'; `sí` means 'yes' — worth getting right.").

### Punctuation
Ignore `¿` and `¡` entirely. No remark. Keyboard limitations make these unreasonable to require.

### Capitalization
Ignore entirely. Capitalization errors signal nothing about language acquisition.

### Grammatically valid Spanish that avoids the tested construction
Mark as correct with a remark noting what construction was expected and why it's worth practicing.
*(Future consideration: tighten this to incorrect once the learner experience is validated — avoiding a construction is not the same as demonstrating mastery of it.)*

### Partial credit
None. Evaluation is binary correct/incorrect. The `remarks` field handles "you were close" communication. The scheduler treats any wrong answer uniformly.

---

## Error Attribution

- `errorTag` is a single tag — one of `primaryTag` or one of `stackedTags`
- When multiple tags are wrong simultaneously, attribute to `primaryTag`
- `errorTag` is `null` when `correct` is `true`

The deliberate practice scheduler (artifact #11) consumes `errorTag` to determine which skill to resurface.

---

## Design Decisions

### LLM-based evaluation, not string matching
Variant acceptance (clitic placement, optional pronouns, lexical synonyms) is handled natively by the evaluator LLM anchored to the canonical answer. An explicit variants list would add authoring overhead without improving reliability at this stage. Revisit if LLM evaluation proves inconsistent on specific variant classes.

### Always lenient on accents
Even meaning-changing accents (`que` vs `qué`, `si` vs `sí`) are not penalized. The remark surfaces the distinction pedagogically without blocking progress. This prioritizes learner motivation over strict orthographic enforcement in v1.

### Single error tag attribution
The scheduler acts on one tag per error event. Splitting errors across multiple tags adds scheduling complexity with marginal benefit — if multiple tags are weak, primary tag retries will naturally resurface stacked context anyway.

### Tags passed explicitly to evaluator
The LLM cannot reliably infer which skill is under test from the canonical answer alone. Passing `primaryTag` and `stackedTags` with descriptions gives the evaluator the context it needs to attribute errors correctly.
