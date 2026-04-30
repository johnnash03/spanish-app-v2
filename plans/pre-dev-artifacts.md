# Pre-Development Artifacts

Ordered by dependency. Each item is a discrete deliverable to be completed before development starts.
Pick one at a time, complete it, then move to the next.

---

## Tier 1 — Foundations
*Everything else references these. Do these first.*

| # | Artifact | Why first |
|---|----------|-----------|
| 1 | **Exercise item schema** | The atomic unit of the app. Every prompt, every algorithm, every DB table is shaped around this. |
| 2 | **Unit metadata schema** | Defines the container exercises live in; prereq graph, stack ratio, interleave settings. |
| 3 | **Evaluation rules** | Defines what "correct" means — accent strictness, variant acceptance, partial credit tiers. Must be settled before any prompt or logic references correctness. |
| 4 | **Mastery threshold spec** | Defines what "done with a tag" means. Unlocks depend on this; deliberate practice depends on this. |

---

## Tier 2 — State & Schemas
*The runtime data structures. Depend on Tier 1 schemas and mastery rules.*

| # | Artifact | Why here |
|---|----------|----------|
| 5 | **User progress schema** | Per-tag attempt/error/interval state. Needs mastery threshold spec to know what fields to track. |
| 6 | **Session schema** | Active queue + interleave window state. Needs item + unit schemas to define queue shape. |
| 7 | **Error event schema** | What gets logged on a wrong answer. Feeds deliberate practice scheduler. |

---

## Tier 3 — Logic Specs
*Behavioral algorithms. Depend on Tier 1 + 2 schemas.*

| # | Artifact | Why here |
|---|----------|----------|
| 8 | **Session queue assembly algorithm** | The 40/40/20 interleave split, sliding window of last 5 units, long-tail sampling. Core session loop. |
| 9 | **Stack ratio progression spec** | How the % of stacked items climbs from 30% (Phase 1) → 60% (Phase 16) → 100% (capstone). Needed by exercise generation. |
| 10 | **Unit unlock logic** | Given tag mastery state, which units are open. Prereq threshold rules, partial prereq handling. |
| 11 | **Deliberate practice scheduling algorithm** | 3 retries per wrong item, spaced across next 2 sessions, dedup when same tag errors multiple times in one session. |
| 12 | **Error cascade logic** | 3 errors on same micro-skill in a window → resample prereq tags. Define window size and cascade depth. |

---

## Tier 4 — Prompts
*LLM-facing. Depend on schemas (what fields to reference) and evaluation rules (what counts as correct).*

| # | Artifact | Why here |
|---|----------|----------|
| 13 | **Answer evaluation prompt** | First prompt to build — validates the evaluation rules spec against real model output before generation prompts are written. |
| 14 | **Lesson exercise generation prompt** | Generates items for a unit given skill tag, stack ratio, prereqs. Depends on item schema + stack spec. |
| 15 | **Deliberate practice generation prompt** | Generates retry items targeting a specific failing micro-skill. Depends on error event schema + deliberate practice algorithm. |
| 16 | **Hint generation prompt** | Triggered on wrong answer mid-attempt. Nudges without revealing. Depends on item schema + evaluation rules. |
| 17 | **Feedback/explanation prompt** | Post-reveal explanation of why the correct answer is correct, tied to the skill tag. |

---

## Tier 5 — Content
*Authored artifacts. Depend on all schemas and prompts being stable.*

| # | Artifact | Why last |
|---|----------|----------|
| 18 | **Exercise generation style guide** | Rules for authoring items (minimum-pair structure, variant format, tagging stacked items, English cue tone). Must be written before the pilot bank. |
| 19 | **Pilot exercise bank (Phases 1–3, Units 1–15)** | Fully authored set to validate generation prompts and evaluation rubric end-to-end before scaling to all 195 units. |
