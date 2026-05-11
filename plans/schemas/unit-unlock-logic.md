# Unit Unlock Logic
*Pre-dev artifact #10 — Tier 3 Logic Specs*

Given tag mastery state, determines which units are accessible and what guidance to show.
Prerequisite graph is informational — no hard gates.

---

## Core Rule

**All units are accessible at all times.** There are no locked units.

Prerequisites are surfaced as soft warnings when a learner selects a unit whose prereq
tags haven't been mastered yet. The learner can proceed regardless.

---

## Unlock Check (runs on app open)

For each unit, derive its visual state from `attempt_log`:

```
function getUnitState(unit):
  attempts = attempt_log.filter(tag == unit.skillTag)

  if attempts.count == 0:
    return 'not_started'

  last20 = attempts.orderBy(timestamp DESC).limit(20)
  if last20.correctRate >= 0.80 AND last20.count >= 20:
    return 'complete'

  return 'in_progress'
```

---

## Unit List Visual States

| State | Condition | Display |
|-------|-----------|---------|
| Not started | No attempts for `primaryTag` | Neutral |
| In progress | Attempts exist, below mastery threshold | Progress indicator |
| Complete | ≥80% correct over last 20 attempts | Checkmark |

No lock icons. All units are selectable regardless of state.

---

## Prerequisite Warning

Shown only when the learner selects a unit with unmastered prerequisites:

```
function getUnmetPrereqs(unit):
  return unit.prerequisites.filter(tag => !isMastered(tag))

function isMastered(tag):
  last20 = attempt_log.filter(tag == tag).orderBy(timestamp DESC).limit(20)
  return last20.count >= 20 AND last20.correctRate >= 0.80
```

If `getUnmetPrereqs` returns any tags, show:
> "Some prerequisites for this unit aren't mastered yet. Continue anyway?"

One tap to proceed. The warning is informational — it does not block access.

---

## Recommended Next Unit

- **Default**: unit whose `primaryTag` has the most recent timestamp in `attempt_log`
- **No history**: default to Unit 1
- Surfaced as a prominent "Continue" button on the main screen
- The learner can browse and select any other unit from the full list

---

## Design Decisions

### No hard gate — fluid progression
Originally settled as a hard binary gate in artifact #4. Revised here.

The session mechanics (40% interleave review + deliberate practice sessions) continuously
reinforce prior skills regardless of unit order. A hard gate is redundant with these mechanics
and creates frustrating bottlenecks. Fluid progression with soft warnings respects learner
autonomy while the session engine handles remediation.

### Soft warning only on unit selection
Showing prerequisite warnings on every unit in the list creates visual noise. A clean unit
list with completion states is sufficient. The warning appears only when the learner actively
chooses a unit, giving them the information at the moment it's relevant.

### Three visual states, no lock state
Locked/unlocked is replaced with not-started/in-progress/complete. These are more informative
(they tell the learner where they stand, not just what they can or can't do) and consistent
with the fluid progression model.

### No explicit linear mode
A learner who wants to go in order simply always taps "Continue." The recommended next unit
and the prerequisite graph together encode the linear path without requiring a separate mode.
