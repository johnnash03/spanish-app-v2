# Hint Generation Prompt
*Pre-dev artifact #16 — Tier 4 Prompts*

---

## Decision: Not Implemented

Hint generation is dropped for v1. It is redundant with existing feedback mechanisms
and risks becoming a crutch that undermines the practice philosophy.

---

## Why It's Redundant

The current post-answer feedback flow already provides two layers of guidance:

1. **Evaluator remarks** (artifact #13) — surfaces accent issues, wording observations,
   and construction notes immediately after a wrong answer
2. **Explanation prompt** (artifact #17) — explains why the correct answer is correct,
   tied to the skill tag

A hint prompt would add a third layer — a separate LLM call, a "Get a hint" UX element,
and additional complexity — for a scenario already handled by existing mechanisms.

## Why It Conflicts With the App's Philosophy

The app is a "practice-only companion to a separate video + notes channel — the video
carries instruction, the app does reps." The learner is assumed to already know the
material. If they don't know how to construct a sentence, the right response is to
revisit the video, not to receive an in-app hint. Hints risk becoming a crutch that
bypasses the actual learning.

---

## If Hints Are Needed Later

Add if user feedback shows learners are consistently stuck and frustrated in ways that
evaluator remarks and explanations don't address. The trigger, hint depth (nudge vs
reveal), and effect on mastery scoring would all need to be defined at that point.
