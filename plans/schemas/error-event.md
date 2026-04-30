# Error Event Schema
*Pre-dev artifact #7 — Tier 2 State & Schemas*

Defines what gets logged on a wrong answer. Feeds the deliberate practice scheduler and
error cascade logic. Resolved as an extension of attempt_log — no separate table.

---

## Resolution

There is no separate error event table. An error event is an `AttemptRecord` in `attempt_log`
where `correct = false`.

---

## Updated attempt_log Schema

*(Supersedes the schema defined in artifact #5 — adds `learnerAnswer` field)*

```typescript
interface AttemptRecord {
  id: string;            // UUID — primary key
  tag: string;           // Tag this attempt is attributed to (errorTag from EvaluationResult, or primaryTag on correct answers)
  itemId: string;        // UUID of the ExerciseItem that generated this attempt
  correct: boolean;      // Whether the attempt was correct
  learnerAnswer: string; // What the learner actually submitted
  timestamp: Date;       // When the attempt was recorded
}
```

---

## Derived Queries

**Error events for a tag:**
```sql
SELECT * FROM attempt_log
WHERE tag = ? AND correct = false
ORDER BY timestamp DESC
```

**Error cascade detection** (3 errors on same tag in a window):
```sql
SELECT COUNT(*) FROM attempt_log
WHERE tag = ? AND correct = false AND timestamp > ?
```

**Deliberate practice targets** (tags with recent errors):
```sql
SELECT tag, COUNT(*) as error_count FROM attempt_log
WHERE correct = false AND timestamp > ?
GROUP BY tag
ORDER BY error_count DESC
```

---

## Design Decisions

### No separate error event table
attempt_log already captures tag, itemId, correct, and timestamp for every attempt.
An error event is just an attempt where correct = false — no additional storage structure needed.
All downstream consumers (deliberate practice scheduler, error cascade logic) query attempt_log directly.

### learnerAnswer stored on all attempts
Stored on correct and wrong answers alike. Useful for post-answer review ("You said X,
correct answer was Y") and for the deliberate practice generation prompt, which can use
the specific wrong answer to generate more targeted retry items.

### Remarks are ephemeral
Evaluator remarks (accent notes, wording observations) are shown immediately post-answer
and discarded. The deliberate practice scheduler does not need them. Post-session review
uses learnerAnswer vs canonical — not the evaluator's remarks.

### Error cascade computed from attempt_log
The cascade condition (3 errors on same micro-skill in a window → resample prerequisite tags)
is a count query on attempt_log. No separate cascade tracking record needed. The cascade
logic (artifact #12) reads attempt_log directly.
