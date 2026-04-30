# Session Schema
*Pre-dev artifact #6 — Tier 2 State & Schemas*

Defines active queue and interleave window state. Sessions are in-memory constructs derived
from attempt_log at startup — no persisted session schema exists.

---

## Session Types

### Regular Session
Works through the current unit with interleave review.

```typescript
interface RegularSession {
  sessionType: 'regular';
  activeUnitTag: string;         // Derived: most recent unit from attempt_log
  queue: ExerciseItem[];         // Pre-assembled at session start, served in order
}
```

**Queue assembly (40/40/20 split):**
- 40% — items from `activeUnitTag` not recently seen in attempt_log
- 40% — items from the last 5 units by recency in attempt_log
- 20% — items sampled from any mastered tag further back in attempt_log

### Deliberate Practice Session
Focused remediation on weak tags. Separate mode — not mixed into regular sessions.

```typescript
interface DeliberatePracticeSession {
  sessionType: 'deliberate';
  targetTags: string[];          // Tags with recent error patterns, derived from attempt_log
  queue: ExerciseItem[];         // Pre-assembled at session start, targeting weak tags
}
```

---

## Key Design Decisions

### No persisted session state
All session state is derivable from attempt_log:
- **Current unit** — most recent unit's primaryTag in attempt_log
- **Interleave window** — last 5 units by recency in attempt_log
- **Deliberate practice targets** — tags with recent error patterns in attempt_log
- **Recently seen items** — itemIds with recent timestamps in attempt_log

Sessions are assembled fresh at each app open. An interrupted sitting is not resumed — the
app assembles a new queue from the same correct starting point. No state is lost because
attempt_log is the single source of truth.

### Pre-assembled fixed queue, not dynamic item-by-item selection
The full queue is assembled upfront at session start. Items are served in order.
- No deduplication logic needed — each item appears in the list exactly once
- 40/40/20 split is applied cleanly at assembly time
- Predictable session length
- Simpler than dynamic per-item selection

### Deliberate practice is a separate session mode
The spec's original "spaced into next 2 sessions" retry model is superseded by this decision.
Retries are not mixed into regular sessions. Instead:
- Error patterns accumulate in attempt_log
- The app surfaces "weak spots to drill" prominently on the main screen when error patterns exist
- The learner explicitly enters deliberate practice mode when ready
- This serves the deliberate practice philosophy better: focused attention, intentional effort,
  conscious awareness of what is being drilled and why

### Deliberate practice is always available
Not triggered by an error count threshold (arbitrary) or a post-session prompt (easy to dismiss).
Always accessible from the main screen, surfaced prominently when weak tags exist. The learner
stays in control of when to drill weak spots.
