# Phase 2: Vocabulary Track + Combined Exercise Track

> Status: Design complete. Implementation follows grammar track (Phase 1).
> UX / home screen design: deferred to separate session.

---

## Core Philosophy

The app is a content server, not a schedule enforcer. Each track always knows what to show next. The user decides when and how much to practice. No daily goals, no streaks, no enforced pace.

Vocabulary and grammar are equally important pillars. Neither is scaffolding for the other.

---

## The Three Tracks

### 1. Grammar Track (Phase 1 — existing)
AI-generated exercises introducing one new grammar structure at a time. Unit notes cover any vocabulary the learner may not know yet. Format: English → Spanish translation.

### 2. Vocabulary Track (Phase 2)
SRS-based flashcard system over a frequency-ranked word bank. Primary mechanism for word introduction. User-controlled intake pace.

### 3. Combined Exercise Track (Phase 2)
AI-generated exercises using all grammar structures unlocked so far × words currently active in the vocabulary pipeline. The primary surface where vocabulary and grammar reinforce each other in context.

---

## Vocabulary Bank

**Source:** SUBTLEX-ESP corpus, first 2000 words by conversational frequency.

**Curation:** Filter out proper nouns, archaic forms, and words already introduced through grammar track unit notes.

**Word entry shape:**
```json
{
  "lemma": "comer",
  "translation": "to eat",
  "frequencyRank": 142,
  "partOfSpeech": "verb"
}
```

Example sentences are not stored at the word level — they belong to the exercise layer.

---

## Word Pipeline

Each word moves through three states:

| State | Description |
|-------|-------------|
| **New** | Just added to SRS. First flashcard not yet completed. Not yet available in combined track. |
| **Learning** | Has had at least one SRS exposure. Available in combined track exercises. |
| **Mastered** | Has passed SRS maturity threshold. Graduates from active tracking. Still appears naturally in exercises. |

Words always enter through the SRS flashcard first. A word cannot appear in combined track exercises until it has had its first SRS exposure.

---

## Vocabulary Track — SRS Flashcard

**New / Learning words:** Multiple choice (word shown, user picks correct translation from 4 options). Recognition-based, appropriate for early exposure.

**Mature words:** Self-rated recall (user mentally recalls meaning, flips card, rates themselves). Faster and more efficient once a word is consolidating.

**User intake:** User visits the vocab bank and taps "Learn new words" whenever they choose. There is no system-enforced daily word target.

**Contextual feedback at the CTA:** The system observes pipeline health and surfaces advisory feedback at the "Learn new words" touchpoint:
- "You have 14 words still consolidating — adding more may thin your focus."
- "Your pipeline looks healthy — good time to add more words."

Feedback is advisory only. User can always proceed regardless.

---

## Combined Exercise Track

### Unlock Condition
Requires a minimum of **10 words** in new/learning state. If below threshold, the track surface prompts: "Add more words to your vocab bank to unlock exercises."

### Exercise Format
English → Spanish translation. Same format as grammar track — no new interaction pattern to learn.

### Generation Strategy
Exercises are pre-generated in batches and stored in a pool. Generation is triggered in the background when the pool drops below a threshold. The user never waits for generation.

**Generation triggers:**
1. Pool drops below threshold (primary trigger)
2. A new grammar unit is unlocked (worth generating a fresh batch with updated structural variety)

**Generation input contract:**
```json
{
  "unlockedGrammarTags": ["opener.quiero", "opener.puedo", "..."],
  "activeVocabWords": ["comer", "salir", "..."],
  "batchSize": 30,
  "existingExerciseIds": ["..."]
}
```

**Exercise construction principle:** Each generated exercise should contain 1 "new encounter" word (first or second time in context) and 2–3 "consolidating" words (familiar but not yet mastered), embedded in a sentence using unlocked grammar structures. This is the 1T sentence principle — one unknown in a field of known context.

**Pool staleness:** Exercises do not go stale as the vocab window advances. Words used in earlier batches were active at generation time — seeing them in exercises remains valuable reinforcement.

### SRS Integration — Asymmetric Reward
- **Exercise success** → counts as a successful SRS review for the active vocab words in that exercise. Accelerates pipeline progress.
- **Exercise failure** → no penalty to any word's SRS state. Attribution is ambiguous (grammar vs vocabulary confusion), so failure is treated as neutral.

---

## Progress Indicator

A single number: **words mastered.** Shown prominently in the vocab bank. No XP, no streaks, no gamification overhead.

---

## Open Decisions

- Home screen UX and how the three tracks are surfaced to the user — deferred to separate design session.
- Exact SRS algorithm (SM-2, FSRS, or custom) — to be decided during implementation.
- Precise pool threshold values (batch size, low-watermark count) — to be decided during implementation.
- Exact multiple choice distractor selection logic — to be decided during implementation.
