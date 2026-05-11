# ADR 0002 — SRS Algorithm: SM-2

**Status:** Accepted

## Context

Issue #18 requires choosing an SRS algorithm for the vocabulary track. The two main candidates were SM-2 (Anki's classic algorithm) and FSRS (Free Spaced Repetition Scheduler, the modern replacement).

## Decision

**SM-2.**

## Rationale

| Criterion                 | SM-2                                   | FSRS                                                            |
| ------------------------- | -------------------------------------- | --------------------------------------------------------------- |
| Implementation complexity | Low — ~10 lines of math                | High — neural-network-derived formula, requires decay functions |
| Parameters                | 3 per card (repetitions, interval, EF) | 4 per card + global model weights that improve with usage data  |
| Correctness auditability  | Easy — every interval is deterministic | Harder — depends on calibrated parameters                       |
| Retention quality         | Good enough for v1                     | Measurably better at ~10k+ reviews                              |
| Migration path            | Simple — add columns                   | Simple — same columns, different formula                        |

FSRS's retention advantage only becomes meaningful with substantial review history. At v1 scale (one user, hundreds of words, weeks of data), SM-2 and FSRS produce similar outcomes. SM-2 is implementable correctly in one sitting; FSRS requires tuning global model parameters that have no data to tune from at launch.

The slot-in replacement path is clear: both algorithms use the same per-card state shape (`repetitions`, `interval_days`, `ease_factor`). Switching to FSRS later is a formula swap, not a schema migration.

## Maturity Threshold

A word is considered **mastered** when `interval_days >= 21`. This matches the SM-2 community convention for "mature" cards and means a word must have been reviewed successfully at least 4 times (intervals: 1 → 6 → 15 → 37 days) before it graduates from active tracking.

## Quality Mapping

User recall is mapped to a binary signal for v1 simplicity:

- Correct (multiple choice hit or self-rated recall) → quality 4
- Incorrect → quality 0

This means EF can only decrease or hold steady — it never increases beyond the default 2.5. This is intentional: it keeps scheduling conservative and avoids over-extending intervals before the learner has truly consolidated a word. Quality-5 ("easy") support can be added later if a user-facing "easy" rating is introduced.
