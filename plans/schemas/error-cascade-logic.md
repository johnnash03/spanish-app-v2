# Error Cascade Logic
*Pre-dev artifact #12 — Tier 3 Logic Specs*

---

## Decision: Not Implemented

Error cascade logic is dropped. It is redundant with existing mechanisms and adds
scheduling complexity for no additional benefit.

---

## What Cascade Was

The spec defined: "If the same micro-skill errs three times across a window, the unit's
prerequisite tags are also resampled."

The intent was: when a tag keeps failing, surface its prerequisite tags for remediation
on the assumption that the root cause is a shaky foundation skill.

---

## Why It's Redundant

Our architecture already handles this at every level:

1. **Deliberate practice trigger** — if a prerequisite tag is genuinely weak, it
   accumulates its own errors in `attempt_log` and triggers deliberate practice
   independently (≥3 of last 10 attempts wrong). No cascade needed to surface it.

2. **Regular session review bucket** — the 40% review bucket resurfaces prior tags
   in every session. Struggling prior tags appear naturally via interleave review.

3. **Long-tail sampling weighted by error rate** — older weak tags surface more
   frequently in the 20% bucket automatically, proportional to their error rate.

The cascade mechanism was a relic of the original "spaced into next 2 sessions" model
where prior tags needed explicit resampling. In the current architecture, session
mechanics and the deliberate practice trigger handle root-cause weaknesses organically.

---

## If Cascade Is Needed Later

If analysis shows that learners consistently fail tags without the prerequisite tags
independently triggering the deliberate practice threshold — i.e. the cascade hypothesis
is real and the existing mechanisms don't catch it — add cascade as an explicit rule:

> When a tag's error rate exceeds X% over last 20 attempts, add its direct prerequisite
> tags to the deliberate practice queue regardless of their own error rates.

Implement only if evidence supports it.
