# Session Queue Assembly Algorithm
*Pre-dev artifact #8 — Tier 3 Logic Specs*

The 40/40/20 interleave split, sliding window of last 5 units, and long-tail sampling.
Core session loop for both regular and deliberate practice sessions.

---

## Regular Session Queue

### Session Size
Concept-driven — no fixed item count. The session serves all unseen items for the current
unit, interleaved with review items in 40/40/20 proportion. The session ends naturally when
the current unit's unseen items are exhausted. The learner can regenerate for more practice.

### Current Unit
The unit whose `primaryTag` has the most recent timestamp in `attempt_log`.

### "Unseen" Definition
An item is unseen if it has no record in `attempt_log`. Items the learner has already
answered belong in the review bucket, not the current unit bucket.

### 40/40/20 Split

| Bucket | % | Source |
|--------|---|--------|
| Current unit | 40% | Unseen items where `primaryTag = activeUnitTag` |
| Review window | 40% | Items from the last 5 units by recent activity in `attempt_log` |
| Long-tail | 20% | Items from tags older than last 5 units, weighted by error rate |

**Proportions are soft targets, not hard requirements.** Early in the learner's journey,
prior unit buckets may not have enough items to fill their proportion. Accept a shorter
session with whatever is available — do not pad from the current unit bucket.

### Last 5 Units
The 5 units with the most recent timestamps in `attempt_log`, regardless of completion
status. Recent activity is the right signal — waiting for unit completion delays reinforcement.

### Long-tail Sampling
Tags older than the last 5 units, sampled with probability weighted by error rate in
`attempt_log`. Tags with more errors surface more frequently. This turns the long-tail
bucket into passive remediation — weak older skills reappear more often without requiring
the learner to enter deliberate practice mode explicitly.

### Queue Order
Fully shuffled — all three buckets are interleaved throughout the session. Serving in
blocks (all current unit first, then review) defeats the interleaving effect.

---

## Deliberate Practice Queue

### Trigger
A tag qualifies as weak if **≥3 of its last 10 attempts are wrong** in `attempt_log`.
Schedule-agnostic — no time window, no session counting. See artifact #11 for full details.

### Queue Contents
For each weak tag:
1. Original failed items (items where `correct = false` in `attempt_log` for this tag)
2. All unseen items for the weak tag

Both sets are shuffled together. The learner re-attempts their original mistakes alongside
new items — re-attempting the original item tests "can you fix the specific mistake" while
new items test "can you generalize the skill."

### Queue Size
Concept-driven — all available items for weak tags, no fixed cap.

---

## Design Decisions

### Concept-driven session length
No fixed or user-configured item count. Harder concepts with more items produce longer
sessions naturally. The learner can regenerate for more practice. Artificial caps distort
the concept-driven philosophy and create inconsistent coverage.

### Unseen = no attempt record
Items already answered by the learner belong in the review bucket. The current unit bucket
drains naturally as the learner works through it, signaling unit progression.

### Last 5 units by activity, not completion
A learner may work on a unit for multiple sessions without completing it. Waiting for
completion before including a unit in the review window delays reinforcement unnecessarily.

### Long-tail weighted by error rate
Pure random long-tail sampling treats a consistently-failed tag the same as an easy one.
Weighting by error rate provides passive remediation — weak older skills resurface more
often without explicit learner action.

### Deliberate practice includes original failed items
Re-attempting the exact item that triggered the error is pedagogically valuable. A correct
re-attempt signals the learner caught their mistake; another wrong attempt signals deeper
weakness. Original items are scored, not shown as unscored review cards.

### Deliberate practice trigger: 3 of last 10 attempts wrong
Revised from "3 errors in 7 days" in artifact #11. Attempt-count-based trigger is
schedule-agnostic and consistent regardless of practice frequency or break length.
