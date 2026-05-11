# Feedback/Explanation Prompt
*Pre-dev artifact #17 — Tier 4 Prompts*

---

## Decision: Merged into Artifact #13

The feedback/explanation prompt is not a separate prompt. It is merged into the answer
evaluation prompt (artifact #13) as the `explanation` field on `EvaluationResult`.

---

## Rationale

The evaluator already has all the context needed to generate an explanation:
- `source` — what the learner was asked
- `canonical` — the correct answer
- `learnerAnswer` — what they wrote
- `primaryTag` + description — the skill being tested
- `errorTag` — which tag the error was attributed to

Adding `explanation: string | null` to `EvaluationResult` handles both jobs in one API
call, saving a round-trip after every wrong answer. The explanation is `null` on correct
answers and a 1–3 sentence pedagogical breakdown on wrong ones.

## Updated EvaluationResult

See artifact #13 for the full updated schema and few-shot examples.

```typescript
interface EvaluationResult {
  correct: boolean;
  errorTag: string | null;
  remarks: string[];
  explanation: string | null; // null when correct; pedagogical breakdown when wrong
}
```
