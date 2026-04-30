# User Progress Schema
*Pre-dev artifact #5 — Tier 2 State & Schemas*

Per-tag attempt and error state. The mastery threshold spec (artifact #4) runs against this data.

---

## Schema

Single table: `attempt_log`

```typescript
interface AttemptRecord {
  id: string;            // UUID — primary key
  tag: string;           // Tag this attempt is attributed to
  itemId: string;        // UUID of the ExerciseItem that generated this attempt
  correct: boolean;      // Whether the attempt was correct
  learnerAnswer: string; // What the learner actually submitted (added in artifact #7)
  timestamp: Date;       // When the attempt was recorded
}
```

---

## How Attempts Are Written

Every evaluated answer produces one or more attempt records:

- **Correct answer**: one record written for `primaryTag` with `correct: true`
- **Wrong answer**: one record written for `errorTag` (from `EvaluationResult`) with `correct: false`

Both stacked and primary tag appearances accumulate attempts — whichever tag the evaluator attributes the result to gets the record.

---

## Derived State (never stored explicitly)

All higher-level state is computed at runtime from the attempt log:

**Tag mastery** — last 20 records for a given `tag`, compute `correct` rate. Mastered if ≥80%.

```
SELECT correct FROM attempt_log
WHERE tag = ? ORDER BY timestamp DESC LIMIT 20
```

**Unit completion** — unit's `primaryTag` is mastered (≥80% over last 20 attempts).

**Unit unlock** — all tags in `unit.prerequisites` are individually mastered.

**Recently seen items** — query `itemId` against attempt log to avoid serving the same item twice across sessions.

---

## Design Decisions

### Single-user, no userId
This is a local desktop app with on-device storage. All progress belongs implicitly to the device owner. Multi-user scoping (userId foreign key, row-level security) adds complexity with no payoff. Revisit if the app moves to the web or adds accounts.

### Raw attempt log, not running summary
A fixed-size summary (e.g. `recentAttempts: boolean[]` updated in place) is a premature optimization. SQLite handles unbounded attempt logs trivially for a single user — even 10,000 attempts is negligible. Raw logs support debugging, future SRS, and retrospective analysis without a schema migration.

### itemId on attempt record
Not used for mastery calculation, but needed for session assembly: knowing which items a learner has recently seen prevents repetition within and across sessions. One UUID field per record with no meaningful storage cost.

### No per-tag summary cache
"Last 20 attempts for a tag" is a trivial indexed query on a local SQLite database. A cached summary adds a write path that must stay in sync with the attempt log. Add a materialized view only if query performance becomes a measurable problem.

### Unit completion and unlock state are derived
Storing explicit completion or unlock flags risks drift — if attempt records change, the flags become stale. Since both are cheap queries against the attempt log, there is no reason to maintain separate state.
