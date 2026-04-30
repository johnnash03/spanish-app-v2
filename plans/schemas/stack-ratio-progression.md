# Stack Ratio Progression Spec
*Pre-dev artifact #9 — Tier 3 Logic Specs*

Defines how the % of stacked items in a unit is determined. Used by the exercise generation
prompt to know how many items should combine the primary skill with prior tags.

---

## What Stack Ratio Means

The stack ratio for a unit is the percentage of its exercise items that combine the primary
skill (`primaryTag`) with one or more prior skills (`stackedTags`).

- A **minimum-pair item** (not stacked) isolates only the primary skill
- A **stacked item** requires the learner to apply the primary skill alongside prior skills simultaneously

All items in a unit practice the primary skill regardless of stacking — the ratio controls
how much prior-skill pressure is applied on top.

---

## Rules

### Rule 1 — `.mixed` units: always 100%
Any unit whose `skillTag` ends in `.mixed` gets 100% stack ratio, regardless of phase.

These are consolidation units where mixing prior skills IS the exercise by definition
(e.g. `opener.mixed`, `stem.pres.mixed`, `conj.pres.regular.mixed`). Minimum-pair items
are not meaningful for these units.

### Rule 2 — All other units: two-segment linear formula

**Anchor points:**
| Phase | Stack ratio |
|-------|-------------|
| 1 | 30% |
| 16 | 60% |
| 42 | 100% |

**Segment 1 — Foundation arc (Phases 1–16):**
```
ratio = 30% + (phase - 1) × 2%
```

**Segment 2 — Integration arc (Phases 17–42):**
```
ratio = 60% + (phase - 16) × 1.54%
```

**Rounding:** snap result to nearest 5%. Precision beyond 5% is false accuracy on a
15–25 item drill set (the difference between 44% and 45% on 20 items is zero items).

**Sample values:**

| Phase | Raw | Snapped |
|-------|-----|---------|
| 1 | 30% | 30% |
| 5 | 38% | 40% |
| 8 | 44% | 45% |
| 16 | 60% | 60% |
| 20 | 66% | 65% |
| 24 | 72% | 70% |
| 30 | 82% | 80% |
| 35 | 90% | 90% |
| 42 | 100% | 100% |

---

## No Per-Unit Override

There is no `stackRatioOverride` field on the unit schema. The two rules above cover all
cases — `.mixed` units always get 100%, all others follow the formula. If a unit's ratio
needs to deviate, the right fix is to reconsider its `skillTag` or phase assignment, not
to add an override.

---

## Design Decisions

### Two-segment linear, not single linear
The spec explicitly marks Phase 16 as a milestone ("climbing to 60% by Phase 16"). Treating
it as a breakpoint respects the pedagogical arc: Phases 1–16 are the foundation-building arc
(new skills introduced one at a time), Phases 17–42 are the integration arc (learner combines
all skills toward B2 fluency).

### .mixed rule replaces override
Every interleaved consolidation unit in the spec has a `.mixed` tag — this is a consistent,
rule-derivable signal. Applying 100% to all `.mixed` units automatically handles the cases
where the formula would produce an inappropriately low ratio, without requiring any author
judgment or manual override per unit.

### Snap to nearest 5%
The generation prompt uses the ratio to determine how many stacked items to author out of
15–25 total. A clean percentage (45%, 60%, 70%) is easier to reason about than 44.3% or
69.2%. Rounding at the percentage level rather than the item level keeps the spec human-readable.
