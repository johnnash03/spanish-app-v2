# Deliberate Practice Scheduling Algorithm
*Pre-dev artifact #11 — Tier 3 Logic Specs*

Defines when weak tags are surfaced, how items are selected, and how the practice queue
is prioritized. UX organization (session vs persistent queue) is deferred to implementation.

---

## Trigger

A tag qualifies as weak when **≥3 of its last 10 attempts are wrong** in `attempt_log`:

```
function isWeak(tag):
  last10 = attempt_log
             .filter(tag == tag)
             .orderBy(timestamp DESC)
             .limit(10)
  wrongCount = last10.filter(correct == false).count
  return wrongCount >= 3
```

Evaluated on app open. All weak tags are surfaced simultaneously.

---

## Prioritization

Weak tags are sorted by **error rate descending** — higher error rate means more items
contributed to the practice queue:

```
errorRate(tag) = wrongCount / last10Attempts
```

A tag with 4/10 wrong (40%) gets more items than a tag with 3/10 wrong (30%).

---

## Queue Assembly

For each weak tag, collect:
1. **Original failed items** — `ExerciseItem` records where `correct = false` in `attempt_log` for this tag
2. **All unseen items** — items for this tag with no record in `attempt_log`

Shuffle both sets together per tag. Distribute items across weak tags **proportionally to
error rate**:

```
itemShare(tag) = errorRate(tag) / sum(errorRate for all weak tags)
```

All weak tags are included in one practice queue — the learner drills everything in one
go rather than entering separate sessions per tag.

---

## Resolution

No explicit "resolved" state. A tag drops off the weak list automatically when its error
rate falls below threshold — derived fresh from `attempt_log` on every app open.

If the learner drills a tag successfully, new correct attempts push the error rate down.
If they keep failing, the tag stays on the list.

---

## Design Decisions

### Trigger: attempt-count-based, not time-based
Originally "3 errors in last 7 days" (artifact #8), then "3 errors in last 5 sessions."
Both approaches are fragile: time-based windows wipe history when the learner takes a break;
session-count windows are inconsistent because sessions vary wildly in size (1 item vs 25).

"3 of last 10 attempts wrong" is schedule-agnostic, tag-specific, quantity-consistent,
and aligned with how mastery threshold is measured (also uses attempt counts per tag).

### No sessionId on attempt_log
The session-count-based trigger required stamping a sessionId on every attempt record.
Switching to attempt-count-based trigger eliminates this need. attempt_log schema is unchanged.

### No dedup within a session
Each error counts independently. Getting a tag wrong 4 times in one sitting is a stronger
signal, not a weaker one. The original spec mentioned dedup but that was in the context of
the "spaced into next 2 sessions" model, which has been replaced by the separate deliberate
practice mode.

### Prioritization by error rate, not raw count
A tag with 4/10 wrong (40% error rate) is weaker than a tag with 3/10 wrong (30%),
even though both clear the threshold. Error rate gives a more accurate picture of relative
weakness and drives proportional item distribution.

### Original failed items included in queue
Re-attempting the exact item that triggered the error tests "can you fix the specific mistake."
If the learner gets it right this time, that's meaningful signal. If wrong again, the tag
clearly needs more work. Original items are scored alongside generated items, not shown as
unscored review cards.

### UX organization deferred
The algorithm is the same whether deliberate practice is presented as a discrete session
(clear start and end) or a persistent queue (ongoing backlog). The presentation model is
a UI concern easier to decide when building the actual screens.
