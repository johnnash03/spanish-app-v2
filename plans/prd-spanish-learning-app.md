# PRD: Spanish Learning App

> Current target: single user (personal tool). Architecture decisions reflect this. Multi-user expansion is a future consideration, not a current requirement.

---

## Problem Statement

Learning Spanish to fluency requires mastering two independent but interconnected pillars: grammar (how the language is structured) and vocabulary (what words to use within those structures). Most existing tools treat these as a single problem — Duolingo mixes them haphazardly, Anki handles only one, grammar textbooks ignore vocabulary acquisition entirely.

The learner needs a tool that:
- Teaches grammar structures systematically and progressively, with reinforcement built into every session
- Builds vocabulary rapidly through exposure breadth first, then consolidates through spaced repetition
- Brings both pillars together in context — exercises that use real grammar structures with words the learner is actively learning
- Respects learner autonomy — no enforced schedules, no daily streaks, no pace mandates

---

## Solution

A local-first desktop app (Tauri) with three distinct but interconnected learning tracks:

1. **Grammar Track** — Structured, unit-based grammar progression. AI-generated exercises drill one new grammar structure at a time, with deliberate interleaving of prior structures across every session. LLM-based answer evaluation handles natural variation in correct Spanish.

2. **Vocabulary Track** — SRS-based flashcard system over a frequency-ranked word bank (first 2000 most common Spanish words). User controls their intake pace. Words move through a pipeline from new → learning → mastered.

3. **Combined Exercise Track** — AI-generated exercises that combine all grammar structures the learner has unlocked so far with vocabulary words currently active in their pipeline. This is where grammar and vocabulary reinforce each other in context.

The app is a content server, not a schedule enforcer. Each track always knows what to show next. The learner decides when and how much to practice.

---

## User Stories

### Grammar Track

1. As a learner, I want to see a list of all grammar units so that I know what is available and where I stand.
2. As a learner, I want each unit to show its completion state (not started / in progress / complete) so that I can see my progress at a glance.
3. As a learner, I want a "Continue" button that takes me to my most recently active unit so that I can resume practice without navigating manually.
4. As a learner, I want to be able to select any unit regardless of whether I have mastered its prerequisites so that I am never blocked from exploring the curriculum.
5. As a learner, I want to see a soft warning when I select a unit whose prerequisites I haven't mastered so that I am informed without being blocked.
6. As a learner, I want to be shown an English sentence and type its Spanish translation so that I practice active production of the language.
7. As a learner, I want my answer to be evaluated by an AI that understands natural Spanish variation so that correct alternative phrasings are accepted, not just the exact canonical answer.
8. As a learner, I want to know immediately whether my answer was correct or incorrect so that I get instant feedback.
9. As a learner, I want to see a hint when I get an answer wrong so that I can understand what I missed without having the answer revealed outright.
10. As a learner, I want to see an explanation of why the correct answer is correct after each exercise so that I understand the grammar rule being drilled.
11. As a learner, I want each session to include review of recently practiced grammar structures, not just the current unit, so that prior skills stay fresh.
12. As a learner, I want prior grammar structures to be mixed into current unit exercises (stacked items) so that I practice applying new structures alongside ones I already know.
13. As a learner, I want the proportion of stacked items to increase as I progress through the curriculum so that later sessions feel more like real language use.
14. As a learner, I want a unit to be marked complete when I have demonstrated consistent accuracy on its primary skill so that completion reflects genuine mastery.
15. As a learner, I want a completed unit to never re-lock even if my accuracy temporarily dips so that my progress is never taken away from me.
16. As a learner, I want weak grammar skills to be automatically resurfaced in review so that I do not have to manually track what needs more practice.
17. As a learner, I want a dedicated deliberate practice mode that focuses on my weakest grammar skills so that I can do targeted remediation when I choose.
18. As a learner, I want the deliberate practice entry point to always be visible on the main screen when I have weak spots so that I am aware of them but not forced to address them.
19. As a learner, I want the app to automatically insert prerequisite review exercises when I am making repeated errors on a skill, so that gaps in foundational knowledge are addressed in the moment.
20. As a learner, I want session state to be reconstructed from my attempt history rather than persisted separately so that an interrupted session is never lost or corrupted.

### Vocabulary Track

21. As a learner, I want a vocabulary bank showing all words ranked by frequency so that I know I am learning the most useful words first.
22. As a learner, I want to see each word's current state (new / learning / mastered) in the vocabulary bank so that I know where each word stands in my pipeline.
23. As a learner, I want a single prominent number showing how many words I have mastered so that I have a satisfying measure of vocabulary progress.
24. As a learner, I want a "Learn new words" CTA that adds words from the frequency-ranked bank into my active pipeline so that I control my vocabulary intake pace.
25. As a learner, I want to see contextual feedback at the "Learn new words" CTA indicating whether my current pipeline load is healthy, too full, or ready for more so that I can make an informed decision about adding words.
26. As a learner, I want new and learning-phase words to be drilled with multiple choice so that I can build recognition without the pressure of full recall on unfamiliar words.
27. As a learner, I want mature words to be drilled with self-rated recall so that I practice genuine retrieval as words consolidate.
28. As a learner, I want the SRS system to show me each word at increasing intervals as I demonstrate consistent recall so that my review time is focused on words that need it.
29. As a learner, I want a word to be considered mastered once it has passed the SRS maturity threshold so that mastery reflects durable retention, not a single correct guess.
30. As a learner, I want mastered words to continue appearing naturally in combined track exercises so that they stay active in my memory through contextual use.

### Combined Exercise Track

31. As a learner, I want a combined exercise track that uses all the grammar structures I have unlocked alongside words I am currently learning so that I can see vocabulary in meaningful grammatical context.
32. As a learner, I want the combined track to be unlocked only once I have at least 10 words in my active vocabulary pipeline so that exercises have enough variety to be useful.
33. As a learner, I want combined track exercises to use the same free-text translation format as grammar track exercises so that I do not have to learn a new interaction pattern.
34. As a learner, I want each combined track exercise to focus on one or two new vocabulary words embedded in familiar grammatical context so that the unknown is always manageable.
35. As a learner, I want getting a combined track exercise correct to count as a successful review for the vocabulary words it contains so that exercises and flashcards reinforce each other.
36. As a learner, I want getting a combined track exercise wrong to have no penalty on my vocabulary SRS state so that grammar confusion does not incorrectly set back vocabulary progress.
37. As a learner, I want combined track exercises to be generated in advance and ready immediately so that I never wait for content to be produced during a session.
38. As a learner, I want the combined exercise pool to refresh automatically as my vocabulary window advances and as I unlock new grammar units so that exercises always reflect my current learning state.
39. As a learner, I want combined track exercises to mix multiple unlocked grammar structures in a single sentence so that I practice the language holistically, not in isolated drills.

### General

40. As a learner, I want all three tracks to be independently accessible from the main screen so that I can choose what to practice without the app deciding for me.
41. As a learner, I want my progress to be stored locally on my device so that it is always available without internet access.
42. As a learner, I want my data to persist across app restarts so that I never lose my progress.
43. As a learner, I want unit notes to explain any vocabulary appearing in grammar exercises that I may not know yet so that unfamiliar words do not block me from practicing grammar.

---

## Implementation Decisions

### Platform & Storage
- **Desktop app** built with Tauri
- **Local SQLite** database for all structured data (attempt log, vocabulary state, exercise bank)
- **Filesystem** for any larger assets (exercise batches, generated content)
- Single-user architecture — no userId, no authentication, no cloud sync

### Grammar Track

- **ExerciseItem shape:** `id` (UUID), `source` (English cue), `canonical` (Spanish answer — server-side only, never sent to frontend), `primaryTag`, `stackedTags[]`
- **Canonical answer** is passed only to the LLM evaluator and never exposed to the client
- **Answer evaluation** is LLM-based, not string matching — handles accent leniency, clitic placement variants, valid lexical synonyms. Anchored to the canonical answer.
- **Attempt log** is the single source of truth for all progress state. Session state, mastery, unit completion, and deliberate practice targets are all derived from it at runtime — nothing is stored explicitly.
- **AttemptRecord shape:** `id`, `tag`, `itemId`, `correct`, `learnerAnswer`, `timestamp`
- **Mastery threshold:** ≥80% correct over the last 20 attempts for a given tag (rolling window, no SRS)
- **Session queue assembly:** 40% current unit items not recently seen, 40% items from the last 5 units by recency, 20% long-tail items from mastered tags further back. Assembled upfront at session start — fixed queue, not dynamic.
- **No persisted session state** — sessions are reconstructed from attempt_log on every app open
- **Unit visual states:** not started / in progress / complete — derived from attempt_log
- **Unit unlock:** no hard gates. All units accessible at all times. Prerequisite warnings are soft and informational only.
- **Error cascade:** 3 errors on the same micro-skill within a session window → insert prerequisite tag items into the current queue. Transparent to the user.
- **Deliberate practice:** separate session mode, always accessible, not modal. Surfaces when weak tags exist. Attempt records write to attempt_log identically to regular attempts.
- **Stack ratio progression:** starts at ~30% stacked items in Phase 1, climbs toward 100% at capstone mixed units. Applied at exercise generation time, not at runtime.
- **Exercise generation** is an offline authoring pipeline (not a runtime call). Generated items are reviewed and committed to the exercise bank. The pilot bank (`plans/schemas/pilot-exercise-bank.json`) covers Phases 1–3, Units 1–15 and is the initial seed.
- **Hints** are LLM-generated on demand after a wrong answer — nudge without revealing.
- **Post-answer explanation** is LLM-generated — tied to the primaryTag skill, explains why the correct answer is correct.

### Vocabulary Track

- **Word bank source:** SUBTLEX-ESP corpus, first 2000 words by conversational frequency, curated to remove proper nouns, archaic forms, and words already covered in grammar unit notes
- **Word entry shape:** `lemma`, `translation`, `frequencyRank`, `partOfSpeech` — no example sentences at the word level
- **Word pipeline states:** new → learning → mastered
- **SRS algorithm:** to be chosen during implementation (SM-2 or FSRS)
- **Flashcard interaction:** multiple choice (4 options) for new/learning words; self-rated recall for mature words
- **Intake:** user-initiated via "Learn new words" CTA. No system-enforced daily target.
- **Feedback at CTA:** advisory signal based on pipeline health. Never blocks the user.

### Combined Exercise Track

- **Unlock condition:** minimum 10 words in new/learning state
- **Exercise format:** English → Spanish free text, identical to grammar track
- **Generation strategy:** pre-generated offline in batches. Background replenishment triggered when pool drops below a threshold or when a new grammar unit is unlocked.
- **Generation input:** unlocked grammar tags + active vocabulary words (new/learning state)
- **Exercise construction principle:** each exercise embeds 1 new-encounter vocabulary word and 2–3 consolidating words in a sentence using unlocked grammar structures (1T sentence principle)
- **SRS integration:** exercise success counts as a successful SRS review for active vocabulary words in the exercise. Exercise failure has no effect on SRS state.
- **Pool staleness:** exercises do not go stale as the vocabulary window advances — words used in earlier batches remain valid reinforcement content

---

## Out of Scope

- Multi-user support and authentication
- Cloud sync or remote storage
- Audio, listening, or speaking exercises
- Mobile platform (iOS/Android)
- Home screen UX design — deferred to a separate design session
- Vocabulary track and combined track (Phase 2 — see `plans/phase-2-vocab-combined-track.md`)
- Onboarding flow for new users
- Settings or customization UI beyond vocabulary intake pacing

---

## Further Notes

- All schema-level design decisions are documented in full in `plans/schemas/`. The PRD references those decisions but does not duplicate them — the schema files are the authoritative source.
- The Phase 2 design (vocabulary track + combined track) is documented in `plans/phase-2-vocab-combined-track.md`.
- The exercise generation style guide (`plans/schemas/exercise-generation-style-guide.md`) governs authoring conventions for all AI-generated exercises across both phases.
- Tech stack (Tauri version, frontend framework, LLM provider and model) must be finalized before implementation begins.
