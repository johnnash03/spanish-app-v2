# Mastery Threshold Spec
*Pre-dev artifact #4 — Tier 1 Foundation*

Defines what "done with a tag" means. Unit unlocks and deliberate practice scheduling depend on this.

---

## Approach

**Rolling accuracy window** — not SRS.

Mastery is defined as ≥80% correct over the last 20 attempts at a given tag.

SRS is deferred — it requires interval tracking, decay functions, and scheduling machinery that is out of scope for v1. Can be layered on later if retention becomes a concern.

---

## Parameters

| Parameter | Value |
|-----------|-------|
| Window size | 20 attempts |
| Accuracy threshold | 80% (≥16/20 correct) |

---

## What Counts as an Attempt

Every item where a tag appears — whether as `primaryTag` or in `stackedTags` — contributes an attempt to that tag's mastery window. Sources:

- Working through a unit's items directly
- Interleave review (prior tags pulled into the 40% review bucket each session)
- Deliberate practice retries (wrong answers generate 3 retry items at the same tag)

Error attribution follows the evaluation rules (artifact #3): `errorTag` on the `EvaluationResult` determines which tag receives the failed attempt.

---

## Unit Completion

- A unit is **complete** when its `primaryTag` reaches ≥80% over the last 20 attempts
- Stacked tag mastery does not gate unit completion — stacked tags are prior skills under review, not the skill being taught
- Shown to the learner as a simple complete/incomplete indicator per unit
- Tag names, accuracy percentages, and window progress are **never exposed to the learner** — tags are internal engine vocabulary

---

## Mastery Degradation

- Mastery can degrade naturally as the rolling window updates with new wrong attempts
- Units **never re-lock** once unlocked — re-locking creates confusing UX and cascading unlock complexity
- Degraded tags are resurfaced automatically by the deliberate practice scheduler (artifact #11)

---

## Prerequisite Unlocking

- **No hard gate** — all units are accessible to the learner at any time
- Prerequisites are **informational**: the app shows which prereq tags aren't mastered yet with a soft warning ("Heads up: you haven't mastered X yet — this unit builds on it")
- Unmastered prior skills are handled automatically by interleave review and deliberate practice sessions
- The prerequisite graph guides the learner but does not block them

*(Revised from original hard binary gate in artifact #10 — fluid progression with soft warnings better serves learner autonomy while the session mechanics handle remediation.)*

---

## Design Decisions

### Rolling window over SRS
SRS is a system, not a threshold — it requires its own scheduling algorithm and per-tag interval state. Rolling accuracy is simple, auditable, and sufficient for v1.

### 20-attempt window
Aligns with unit drill set size (15–25 items). By the time a learner finishes a unit's items plus a few interleave appearances, they will naturally have ~20 attempts at the primary tag. Large enough to avoid lucky-streak mastery; small enough to feel achievable.

### Primary tag only gates unit completion
A unit drills one new skill. Declaring it complete when that skill is mastered is clean and predictable. Stacked tag weakness is the deliberate practice scheduler's concern, not the unit completion gate's.

### Tags never exposed to learner
Learners should not be managing skill graphs — that's the engine's job. Deliberate practice handles weak skills automatically. Exposing tag-level progress creates cognitive overhead and invites learners to optimize the wrong thing.

### Fluid progression with soft warnings (revised in artifact #10)
All units are accessible at any time. Prerequisites are surfaced as informational warnings, not hard gates. The interleave review and deliberate practice sessions continuously reinforce prior skills — the strict gate is redundant with these mechanics and creates frustrating bottlenecks for learners.

### Units never re-lock
Re-locking completed units is disorienting UX and creates cascading unlock state complexity. Degraded mastery is handled silently by the scheduler resurfacing the tag in review.
