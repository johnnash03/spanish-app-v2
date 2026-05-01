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
8. As a learner, I want to type through all items in a session without per-item evaluation interruptions so that I can maintain flow, and see all my results together at the end of the session.
9. As a learner, I want each wrong answer in the end-of-session review to be accompanied by a hint that explains what I missed without simply restating the answer, so that I can reason about my mistake.
10. As a learner, I want each wrong answer in the end-of-session review to include an explanation of why the correct answer is correct, so that I understand the grammar rule being drilled. Correct answers do not show explanations by default but can be expanded on demand.
11. As a learner, I want each session to include review of recently practiced grammar structures, not just the current unit, so that prior skills stay fresh.
12. As a learner, I want prior grammar structures to be mixed into current unit exercises (stacked items) so that I practice applying new structures alongside ones I already know.
13. As a learner, I want the proportion of stacked items to increase as I progress through the curriculum so that later sessions feel more like real language use.
14. As a learner, I want a unit to be marked complete when I have demonstrated consistent accuracy on its primary skill so that completion reflects genuine mastery.
15. As a learner, I want a completed unit to never re-lock even if my accuracy temporarily dips so that my progress is never taken away from me.
16. As a learner, I want weak grammar skills to be automatically resurfaced in review so that I do not have to manually track what needs more practice.
17. As a learner, I want a dedicated deliberate practice mode that focuses on my weakest grammar skills so that I can do targeted remediation when I choose.
18. As a learner, I want the deliberate practice entry point to always be visible on the main screen when I have weak spots so that I am aware of them but not forced to address them.
19. As a learner, when I make repeated errors on the same skill within a session, I want the end-of-session review screen to surface a follow-up practice CTA targeting that skill (and its prerequisites) so that I can choose to address gaps immediately after seeing my results.
20. As a learner, I want session state to be reconstructed from my attempt history rather than persisted separately so that an interrupted session is never lost or corrupted.
21. As a learner, when a session ends I want to land on a review screen that shows my wrong answers in detail (with hint and explanation per item) and lists my correct answers compactly (expandable on demand) so that my attention goes to what I need to learn from.
22. As a learner, before starting a unit I want a unit detail screen that shows the unit's description, prerequisite warnings (if any), and the notes glossary so that I am oriented before I begin practice.

### Vocabulary Track

23. As a learner, I want a vocabulary bank showing all words ranked by frequency so that I know I am learning the most useful words first.
24. As a learner, I want to see each word's current state (untouched / new / learning / mastered) in the vocabulary bank so that I know where each word stands in my pipeline. "Untouched" applies to words in the frequency-ranked bank that have not yet been added to my active pipeline.
25. As a learner, I want a single prominent number showing how many words I have mastered so that I have a satisfying measure of vocabulary progress.
26. As a learner, I want a "Learn new words" CTA that adds words from the frequency-ranked bank into my active pipeline so that I control my vocabulary intake pace.
27. As a learner, I want to see contextual feedback at the "Learn new words" CTA indicating whether my current pipeline load is healthy, too full, or ready for more so that I can make an informed decision about adding words.
28. As a learner, I want new and learning-phase words to be drilled with multiple choice so that I can build recognition without the pressure of full recall on unfamiliar words.
29. As a learner, I want mature words to be drilled with self-rated recall so that I practice genuine retrieval as words consolidate.
30. As a learner, I want the SRS system to show me each word at increasing intervals as I demonstrate consistent recall so that my review time is focused on words that need it.
31. As a learner, I want a word to be considered mastered once it has passed the SRS maturity threshold so that mastery reflects durable retention, not a single correct guess.
32. As a learner, I want mastered words to continue appearing naturally in combined track exercises so that they stay active in my memory through contextual use.

### Combined Exercise Track

33. As a learner, I want a combined exercise track that uses all the grammar structures I have unlocked alongside words I am currently learning so that I can see vocabulary in meaningful grammatical context.
34. As a learner, I want the combined track to be unlocked only once I have at least 10 words in my active vocabulary pipeline (new + learning) so that exercises have enough variety to be useful.
35. As a learner, I want combined track exercises to use the same free-text translation format as grammar track exercises so that I do not have to learn a new interaction pattern.
36. As a learner, I want each combined track exercise to focus on one or two new vocabulary words embedded in familiar grammatical context so that the unknown is always manageable.
37. As a learner, I want getting a combined track exercise correct to count as a successful review for the vocabulary words it contains so that exercises and flashcards reinforce each other.
38. As a learner, I want getting a combined track exercise wrong to have no penalty on my vocabulary SRS state so that grammar confusion does not incorrectly set back vocabulary progress.
39. As a learner, I want combined track exercises to be generated in advance and ready immediately so that I never wait for content to be produced during a session.
40. As a learner, I want the combined exercise pool to refresh automatically as my vocabulary window advances and as I unlock new grammar units so that exercises always reflect my current learning state.
41. As a learner, I want combined track exercises to mix multiple unlocked grammar structures in a single sentence so that I practice the language holistically, not in isolated drills.

### General

42. As a learner, I want all three tracks to be independently accessible from the main screen so that I can choose what to practice without the app deciding for me.
43. As a learner, I want my progress to be stored locally on my device so that it is always available without internet access.
44. As a learner, I want my data to persist across app restarts so that I never lose my progress.
45. As a learner, I want unit notes to explain any vocabulary appearing in grammar exercises that I may not know yet so that unfamiliar words do not block me from practicing grammar.

---

## Implementation Decisions

### Platform & Storage

- **Desktop app** built with Tauri
- **Local SQLite** database for all structured data (attempt log, vocabulary state, exercise bank)
- **Filesystem** for any larger assets (exercise batches, generated content)
- Single-user architecture — no userId, no authentication, no cloud sync

### Grammar Track

- **ExerciseItem shape:** `id` (UUID), `source` (English cue), `canonical` (Spanish answer — server-side only, never sent to frontend), `primaryTag`, `stackedTags[]`
- **Canonical answer** is not used for client-side string matching and is not shown to the user during attempt entry. It may be sent to the frontend for display **only after** a session has been submitted, as part of the end-of-session review payload (so the user can see one valid correct answer alongside the explanation for any item they got wrong).
- **Answer evaluation** is LLM-based, not string matching — handles accent leniency, clitic placement variants, valid lexical synonyms. Anchored to the canonical answer.
- **Submission flow:** batched at end of session, not per-item. A session is a single sitting of practice — user-controlled length, not a fixed item count. The user starts a session, practices as many items as they want (could be 3, could be 30), and taps "End & review" when done. At that point all attempts in the session are sent to the LLM evaluator in a single call, and the user lands on an end-of-session review screen showing wrong items in detail (hint, explanation, one valid canonical answer) and correct items as a compact expandable list. There is no per-item retry, no mid-session hint, and no mid-session error cascade — error cascade surfaces as a follow-up-session CTA on the review screen instead. The next session in the same unit picks up where the queue left off.
- **Attempt log** is the single source of truth for all progress state. Session state, mastery, unit completion, and deliberate practice targets are all derived from it at runtime — nothing is stored explicitly.
- **AttemptRecord shape:** `id`, `tag`, `itemId`, `correct`, `learnerAnswer`, `timestamp`
- **Mastery threshold:** ≥80% correct over the last 20 attempts for a given tag (rolling window, no SRS)
- **Session queue assembly:** 40% current unit items not recently seen, 40% items from the last 5 units by recency, 20% long-tail items from mastered tags further back. Queue is generated lazily as the user advances — there is no fixed upfront length since session length is user-controlled. The queue continues producing items until the user ends the session.
- **No persisted session state** — sessions are reconstructed from attempt_log on every app open
- **Unit visual states:** not started / in progress / complete — derived from attempt_log
- **Unit unlock:** no hard gates. All units accessible at all times. Prerequisite warnings are soft and informational only.
- **Error cascade:** 3+ errors on the same micro-skill within a session → surface a follow-up-session CTA on the end-of-session review screen targeting that skill (and its prerequisites). Items are not inserted mid-stream; the cascade is a post-session offer the user can accept or skip.
- **Deliberate practice:** separate session mode, always accessible, not modal. Surfaces when weak tags exist (rendered as an inline pill on the home grammar card). Attempt records write to attempt_log identically to regular attempts.
- **Deliberate practice entry screen:** lists each weak tag with an item count and a per-tag "Practice" CTA, plus a top-level "Practice all weak skills" CTA. The user chooses targeted vs. broad practice.
- **Tag visibility — regular vs. deliberate:** regular sessions hide the tag/skill name during practice (interleaved practice; honest recognition). Deliberate practice **reveals** the tag at all times (banner during practice, named in entry list, named in mastery callouts on review). The two modes implement different pedagogical contracts: interleaved (anonymous) vs. blocked (named). Tag display strings come from unit metadata.
- **Deliberate practice item composition:** fresh items pulled from the exercise bank tagged with the weak skill, not replays of past-missed items. Variable-length session, batched evaluation, end-of-session review screen — same pattern as regular sessions. If a weak tag crosses back above mastery during the session, the review screen surfaces a celebratory callout ("_Ser vs Estar_ is no longer flagged").
- **Stack ratio progression:** starts at ~30% stacked items in Phase 1, climbs toward 100% at capstone mixed units. Applied at exercise generation time, not at runtime.
- **Exercise generation** is an offline authoring pipeline (not a runtime call). Generated items are reviewed and committed to the exercise bank. The pilot bank (`plans/schemas/pilot-exercise-bank.json`) covers Phases 1–3, Units 1–15 and is the initial seed.
- **Hints** are LLM-generated and shown on the end-of-session review screen alongside each wrong item — nudge without revealing.
- **Post-answer explanation** is LLM-generated — tied to the primaryTag skill, explains why the correct answer is correct. Shown by default for wrong items in review; available on demand (expandable) for correct items.
- **Unit detail screen** is the screen between the unit list and a practice session. It shows the unit's name, short description, prerequisite warnings (if any), the unit notes glossary (US #45), and a "Start practice" CTA. Each unit has an authored short description (1–2 lines) stored on unit metadata.

### Vocabulary Track

- **Word bank source:** SUBTLEX-ESP corpus, first 2000 words by conversational frequency, curated to remove proper nouns, archaic forms, and words already covered in grammar unit notes
- **Word entry shape:** `lemma`, `translation`, `frequencyRank`, `partOfSpeech` — no example sentences at the word level
- **Word pipeline states:** untouched → new → learning → mastered. "Untouched" is the default state for any word in the frequency-ranked bank that has not been added to the active pipeline. The "Learn new words" CTA promotes untouched words to "new."
- **SRS algorithm:** to be chosen during implementation (SM-2 or FSRS)
- **Flashcard interaction:** multiple choice (4 options, Spanish → English) for new/learning words with instant feedback and auto-advance; self-rated recall for mature words with 3 rating buttons (Again / Good / Easy). Mixed sessions — the system selects the interaction per card based on the word's state; the user does not choose a mode. Per-card evaluation is local and immediate (no LLM, no batching). Sessions are variable-length with a persistent "End & review" affordance, mirroring the grammar track.
- **Flashcard UI is independent of SRS algorithm choice.** Both SM-2 and FSRS are compatible with the 3-rating self-rate interaction; the algorithm choice can be deferred without affecting UI.
- **Intake:** user-initiated via "Learn new words" CTA. No system-enforced daily target. Default batch size 5 (adjustable: 3 / 5 / 10).
- **Intake flow doubles as first encounter, not first review.** The flow is full-screen. Each added word is shown on a single card (lemma, translation, part of speech, frequency rank) with a "Got it" CTA to advance. No question, no input, no SRS event — purely exposure. After all cards, an "Add to pipeline" confirmation commits the batch.
- **Pipeline health** is a four-band advisory signal computed from active pipeline size (count of words in `new + learning` state). Bands: Light (0–10) / Healthy (11–25) / Full (26–40) / Overloaded (41+). Surfaced at the "Learn new words" CTA on the home vocab card and live-updated within the intake flow as each card is acknowledged. Never blocks the user.

### Failure Modes & Resilience

- **LLM evaluation failure at session end:** attempts persist locally (already in attempt*log as unevaluated). User sees a calm error state: *"We couldn't reach the evaluator. Your answers are saved — try again in a moment."\_ with **Retry evaluation** and **Back to home** CTAs.
- **Offline behavior:** the user can practice offline; only end-of-session evaluation requires network. Offline at submit produces the same "Couldn't reach" state. Matches local-first philosophy (US #43).
- **Partial batch failure:** if any item in the batched eval fails, the whole batch is retried. No per-item eval state — keeps the data model simple.
- **Per-item hint/explanation failure on review screen:** silent hide. If hint generation fails for an item, omit the hint for that item; the canonical answer is always present. Degrade gracefully, no per-item error UI.
- **Long-running eval:** loading screen with calm copy ("Evaluating your answers…"). After ~10s without response, append a reassurance line ("Still working — sometimes the evaluator takes a bit longer"). No client-side timeout; the user can bail with Esc or back.
- **Pending session resume:** if the app opens with an unevaluated session in attempt*log, the home shows a banner above the cards: *"You have an unsubmitted session — review now"\_ with a CTA. No auto-navigation; user opts in.

### Keyboard Interaction

> Implementation note: full keyboard support is deferred to later stages of development. Initial builds may rely on mouse/trackpad. The bindings below are the target end-state.

- **Grammar / Combined practice screen:**
  - Input auto-focused on item arrival
  - **Enter** — submit answer and auto-advance to next item
  - **Esc** — end & review (no confirmation; nothing to lose, attempts are batched at session end)
  - **Cmd/Ctrl+K** — open Notes drawer
- **End-of-session review screen:**
  - **Enter** or **Space** — return to home (primary CTA)
  - **R** — practice again
  - **F** — focus first follow-up CTA (if any)
- **Vocabulary multiple choice:**
  - **1 / 2 / 3 / 4** — select corresponding option
- **Vocabulary self-rated recall:**
  - **Space** — reveal answer
  - **1** — Again, **2** — Good, **3** — Easy
- **Global:**
  - **Cmd/Ctrl+H** — return to home from anywhere
- **Discoverability:** a `?` icon top-right opens a keyboard shortcuts modal. Subtle inline hints (e.g., "Press Enter to submit") appear only in the user's first session, then disappear.

### Visual Design

- **Overall feel:** "studious calm" — a thoughtful study companion, closer to a leather notebook and good fountain pen than to a gamified app.
- **Density:** spacious. Big type, generous whitespace, one primary action per screen. Single-user focused practice — sustained attention is the constraint, not screen real estate.
- **Typography:** serif for content (Spanish cues, English translations, explanations — e.g., a humanist serif like _Source Serif_ or _Lora_) and sans for UI chrome (buttons, labels, counters, navigation — e.g., _Inter_). Treats language as the subject, distinguishes content from tool.
- **Color palette:** warm-neutral, paper-like. Off-white background, dark warm grey text, a single muted accent color (fountain-pen-ink feel — muted green or terracotta, _not_ Duolingo green). Correct/wrong feedback uses muted greens and reds, not bright alarms. Mastery callouts use the accent color, not gold-confetti gamification.
- **Motion:** subtle. Gentle slides for item transitions. No springy bounces, no celebration animations. Motion is functional (signaling change), not entertaining.
- **Microcopy tone:** warm, plainspoken, second-person. "Not quite — here's the rule." "You moved _libro_ to mastered." Avoids both formal stiffness and chirpy gamification.

### Combined Exercise Track

- **Unlock condition:** minimum 10 words in new/learning state
- **Exercise format:** English → Spanish free text, identical to grammar track
- **Generation strategy:** pre-generated offline in batches. Background replenishment triggered when pool drops below a threshold or when a new grammar unit is unlocked.
- **Generation input:** unlocked grammar tags + active vocabulary words (new/learning state)
- **Exercise construction principle:** each exercise embeds 1 new-encounter vocabulary word and 2–3 consolidating words in a sentence using unlocked grammar structures (1T sentence principle)
- **SRS integration:** exercise success counts as a successful SRS review for active vocabulary words in the exercise. Exercise failure has no effect on SRS state.
- **Pool staleness:** exercises do not go stale as the vocabulary window advances — words used in earlier batches remain valid reinforcement content
- **No notes drawer / vocab help during combined exercises.** By design, each combined exercise embeds at most 1–2 unfamiliar words inside familiar grammar; surfacing help would undercut the 1T sentence principle. Vocabulary impact is revealed only on the end-of-session review screen (per-correct-item annotations: "✓ _libro_ advanced in pipeline").
- **No vocabulary highlighting in the English cue.** Cues read as natural English; the new word is by construction the only unfamiliar element.

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
- LessonsMOC - `/plans/LessonsMOC.v2.md`
- Design handoff - `/design_handoff`
- The Phase 2 design (vocabulary track + combined track) is documented in `plans/phase-2-vocab-combined-track.md`.
- The exercise generation style guide (`plans/schemas/exercise-generation-style-guide.md`) governs authoring conventions for all AI-generated exercises across both phases.
- Tech stack (Tauri version, frontend framework, LLM provider and model) must be finalized before implementation begins.
