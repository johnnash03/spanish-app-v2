# PRD: Spanish Practice App v2

> Current target: single user (personal tool). Architecture decisions reflect this.
> V2 is a rewrite of the curriculum data, content generation pipeline, and answer evaluation subsystems within the existing application (same repo, same shell). The app skeleton — desktop shell, local database, session/review screen patterns, attempt-log architecture — survives from v1.
>
> **Status: FROZEN (2026-06-11).** Scope is closed. New ideas are triaged by one test — does it reshape a foundation (schemas, licensing sets, error enum, word-state machine, attempt log)? If yes, it is a deliberate PRD amendment; if it rides on foundations, it goes to the post-launch candidates list in Further Notes without debate. The tracer bullet (first vertical slice) is the arbiter for promoting parked ideas.

---

## Problem Statement

V1 set out to provide deliberate, interleaved practice of Spanish grammar patterns with a controlled vocabulary, and failed in practice for three reasons, all confirmed by analysis of the v1 database (4,055 generated items, 147 attempts, 131 evaluations):

1. **Curriculum ordering was violated by generated content.** Exercises in early units demanded grammar taught many phases later (full conjugation paradigms, ser, verb-specific prepositions appearing in unit 1). Root cause: generation constraints were expressed as prompt prose with no enforcement layer, the style guide actively contradicted unit scope, and six cognate units were seeded into the curriculum against the original MOC design and used as stacking tags everywhere.

2. **Answer evaluation was untrustworthy.** Grammatically correct answers were marked wrong with confidently false explanations ("Los puedes ver" rejected as invalid Spanish). The evaluator violated its own written rules (accent leniency, punctuation leniency, synonym acceptance), hallucinated error tags that exist nowhere in the registry, and misattributed errors — polluting mastery data. Roughly 5% of verdicts were unjust, with no recourse for the learner. Root cause: a weak model performing a genuine linguistic-judgment task, four jobs entangled in one prompt, and no deterministic path for trivially correct answers.

3. **Exercises were boring and repetitive.** Twenty consecutive items of corporate-abstract English ("organize the event," "influence the decision") — a direct consequence of the cognate stacking tags and the absence of any content plan for generation.

The learner abandoned the app. The pedagogical thesis (pattern practice via deliberate and interleaved practice over a rolling vocabulary window) was never disproven — the execution layer failed it.

---

## Solution

V2 keeps the pedagogical thesis and rebuilds the three failed subsystems around one principle: **guarantees live in code, not in prompt prose. The LLM does only what it is good at — writing sentences, describing language, and judging meaning — while rules are enforced by deterministic checks it cannot disobey.**

The product simplifies to **two tracks**:

1. **Practice** — skill-based exercise sessions following the curriculum DAG. Exercises draw their content words from the learner's active vocabulary window, so every grammar rep doubles as a vocabulary rep (this absorbs v1's separate "combined track"). Three formats: free translation (the spine), forced-choice for selection skills, and paradigm micro-drills for verb forms.

2. **Words** — a vocabulary intake surface. The learner pulls new content words from a frequency-ranked bank into their active window at their own pace. All reviewing happens inside Practice; there are no SRS flashcards.

The lexicon is two-tier: **~45 power verbs** (one or two exemplars per conjugation class) are curriculum citizens drilled inside grammar units and visualized on a **conjugation map**; all other content words flow through the rolling window. Vocabulary has no thematic organization — frequency is the only ordering principle.

Three new structural mechanisms deliver the trust and quality v1 lacked:

- **Licensing sets** — each unit declares, as data, exactly which verb forms, constructions, and vocabulary are allowed in its exercises. Ordering becomes checkable.
- **Generate → validate → repair** — every generated item is analyzed by a second LLM call and judged by deterministic code against the licensing set before entering the bank. Violations are regenerated.
- **Three-tier evaluation** — deterministic variant matching first (instant, code-enforced leniency), decomposed LLM judgment second (grammaticality and meaning judged separately from target-structure use), and an appeal button third (re-evaluation by a reasoning-grade model with retroactive overturn).

---

## User Stories

### Practice Track — Sessions

1. As a learner, I want to practice skills as English → Spanish translation exercises, so that I actively produce the language rather than recognize it.
2. As a learner, I want selection skills (ser/estar, por/para, preterite/imperfect, becoming verbs, si-clauses) drilled as forced-choice exercises, so that the subtlest distinctions are practiced and judged without AI ambiguity.
3. As a learner, I want paradigm micro-drills ("nosotros + querer, present → ?"), so that I can directly practice the conjugation forms of the power verbs I care about.
4. As a learner, I want every exercise's content words drawn from my active vocabulary window plus words I already know, so that grammar practice simultaneously reinforces my vocabulary.
5. As a learner, I want sessions to interleave current-unit items with items from recent and older units, so that prior skills stay fresh without my managing review manually.
6. As a learner, I want exercises within a unit to never require grammar or vocabulary from later in the curriculum, so that I am never penalized for not knowing something I haven't been taught.
7. As a learner, I want exercise sentences to vary naturally in person, polarity, sentence type, and word sense, so that practice does not become repetitive or formulaic.
8. As a learner, I want to type through all items in a session without per-item interruptions and end the session whenever I choose, so that I maintain flow during production practice.
9. As a learner, I want my answers evaluated eagerly in the background while I continue practicing, so that my end-of-session review is ready the instant I finish.
10. As a learner, I want any unit to be startable at any time with at most a soft prerequisite warning, so that I am informed but never blocked.
11. As a learner, I want a "Continue" entry point that resumes my most recently active unit, so that starting a session requires no navigation.

### Practice Track — Review & Evaluation

12. As a learner, I want a correctly spelled answer that matches an accepted form to be marked correct instantly and deterministically, so that the most common case never depends on AI judgment.
13. As a learner, I want grammatically valid alternative phrasings (clitic placement variants, optional pronouns, reasonable synonyms) accepted as correct, so that I am never punished for producing real Spanish.
14. As a learner, I want answers differing only in accents, capitalization, or punctuation marks (¿ ¡) to be marked correct with an informational note, so that orthographic slips never block progress — enforced by code, not model discretion.
15. As a learner, I want a correct answer that avoids the unit's target structure to be marked correct with a nudge toward the target form, so that good Spanish is never called wrong.
16. As a learner, I want structure-avoiding correct answers to contribute nothing to my mastery of that skill and cause the skill to be re-served, so that my mastery data reflects demonstrated ability only.
17. As a learner, I want each wrong answer classified into a small, fixed set of error categories with an evidence span, so that my weak-spot data is reliable rather than hallucinated.
18. As a learner, I want wrong answers in review accompanied by a hint and a pedagogical explanation referencing my specific mistake, so that I learn from the review rather than just see a verdict.
19. As a learner, I want an "I think I was right" button on every wrong verdict that re-evaluates my answer with a stronger model, so that evaluation mistakes are recoverable rather than trust-destroying.
20. As a learner, I want a successful appeal to retroactively correct my attempt history and mastery state, so that the record reflects the truth.
21. As a learner, I want every appeal logged with its outcome, so that the system accumulates a regression suite of evaluation edge cases from my real usage.
22. As a learner, I want the review screen to show wrong answers in detail and correct answers compactly (expandable), so that my attention goes where learning happens.

### Curriculum & Ordering

23. As a learner, I want the curriculum organized as skill units with explicit prerequisites forming a DAG, so that both the app and I can reason about what depends on what.
24. As a learner, I want each unit to carry an explicit licensing set (allowed verb forms, constructions, and vocabulary), so that exercise content is mechanically checkable against what I've been taught.
25. As a learner, I want a small, explicit "ambient set" of day-0 licensed material (articles, gender basics, negation, core cognate patterns), so that natural sentences are possible from the first unit without hidden leaks.
26. As a learner, I want cognate transformation patterns available as reference notes rather than as drill units or stacking tags, so that they aid comprehension without flooding exercises with abstract vocabulary.
27. As a learner, I want conjugation and tense units to drill across the power-verb list one paradigm class at a time, so that learning one exemplar unlocks every verb in its class.
28. As a learner, I want to see unit completion states (not started / in progress / complete) derived from my actual attempt history, so that progress display is always truthful.

### Power Verbs & Conjugation Map

29. As a learner, I want a curated list of ~45 power verbs covering every conjugation class (regular families, spelling-change classes, stem-change families, the irregular core including decir), so that deep practice of exemplars generalizes to the whole verb system.
30. As a learner, I want a conjugation map — power verbs × tense/mood forms — with cells shaded by my recency-weighted accuracy, so that I can see at a glance which forms I own and which are weak or untouched.
31. As a learner, I want to tap a sparse or weak region of the conjugation map to start a paradigm drill on it, so that the map doubles as a deliberate-practice entry point.
32. As a learner, I want paradigm drill results and verb-form errors from translation exercises to both feed the map, so that it reflects all evidence about my conjugation ability.

### Words Track & Vocabulary Window

33. As a learner, I want a frequency-ranked bank of content words only (nouns, verbs beyond the power list, adjectives, adverbs — no function words), so that intake is spent on words that grammar practice doesn't already cover.
34. As a learner, I want to pull new words into my active window whenever I choose, with no system-enforced pace, so that intake speed is mine to control.
35. As a learner, I want word intake to be a first-encounter experience (word, translation, part of speech shown plainly), so that the first meeting carries no test pressure.
36. As a learner, I want my window to target ~20 active words with advisory health bands rather than a hard cap, so that I can deliberately go wider when I want speed, informed of the reps-per-word tradeoff.
37. As a learner, I want window entry balanced across parts of speech within frequency order, so that exercise generation always has nouns, verbs, and modifiers to work with.
38. As a learner, I want window words deliberately scheduled into my practice exercises and embedded only in grammar structures I have already consolidated, so that a new word is always the single unknown in its sentence.
39. As a learner, I want a word to graduate out of the window after sustained success spread across multiple distinct days, so that "learned" means durable, not crammed.
40. As a learner, I want words that repeatedly fail in exercises flagged as stuck and offered as a quick multiple-choice warm-up before sessions, so that a struggling word has a lighter-weight remediation path than full exercises.
41. As a learner, I want a single prominent count of graduated words, so that vocabulary progress has one honest number.
42. As a learner, I want graduated words to keep appearing incidentally in exercises, so that mastered vocabulary stays alive through use.

### Deliberate Practice & Mastery

43. As a learner, I want skill mastery computed as recency-weighted accuracy with a minimum-evidence floor, so that the system reacts to my current ability within a few attempts and old errors fade.
44. As a learner, I want weak skills surfaced on the home screen as an always-visible but never-forced deliberate practice entry point, so that I am aware of gaps without being coerced.
45. As a learner, I want deliberate practice sessions that name the skill being drilled (unlike anonymous interleaved sessions), so that blocked, conscious practice is available when I choose it.
46. As a learner, I want error categories from evaluation mapped onto curriculum skills in code (not by AI free-association), so that deliberate practice targets are trustworthy.
47. As a learner, I want a completed unit to never re-lock, so that progress is never taken away.

### Content Generation (system behavior the learner depends on)

48. As a learner, I want exercises generated per unit in the background and cached before I need them, so that I never wait for content during a session.
49. As a learner, I want every generated item validated against the unit's licensing set before it can ever be shown to me, so that ordering violations are caught at the source.
50. As a learner, I want generated items checked for near-duplication and slot variety, so that a unit's bank doesn't collapse into one sentence pattern.
51. As a learner, I want each item authored with a list of accepted answer variants, validated at generation time, so that deterministic evaluation can accept real alternatives instantly.
52. As a learner, I want items whose canonical answer fails to exercise the unit's target skill rejected at validation, so that I am never drilled on an item that contradicts its own unit.

### General

53. As a learner, I want all my data stored locally and persistent across restarts, so that the tool works offline and my history is never lost.
54. As a learner, I want to practice offline with evaluation deferred until connectivity returns, so that lack of network never blocks a session.
55. As a learner, I want an unsubmitted session surfaced as a banner on the home screen when I return, so that interrupted work is recoverable but never auto-resumed.
56. As a learner, I want the app's tone and visuals to remain calm and study-like (no streaks, no gamification, no celebrations), so that the tool respects my autonomy and attention.

---

## Implementation Decisions

### Foundation

- **Same repo, new core.** The desktop shell, local SQLite database, screen patterns (session, end-of-session review), and the attempt-log-as-single-source-of-truth architecture survive from v1. The curriculum seed data, generation pipeline, and evaluation subsystem are rewritten. The v1 SM-2/SRS machinery and the separate combined-exercise subsystem are deleted.
- **V1 data becomes test fixtures.** The 4,055 v1 items serve as validator test input; the 131 v1 evaluations (including the known unjust verdicts) seed the evaluation regression suite.

### Product Shape

- **Two tracks.** Practice (skill sessions, vocabulary embedded) and Words (intake only). The v1 combined track and its unlock condition are removed — integration of grammar and vocabulary is the default nature of all practice.
- **No flashcard SRS.** Vocabulary review happens exclusively through exercise scheduling. The only flashcard-like surface is the stuck-word multiple-choice warm-up.

### Curriculum

- **The v1 MOC DAG survives with surgical edits:** (1) the six cognate units are removed from the unit sequence — cognate patterns move to reference notes and the day-0 ambient set, and may never appear as stacking tags; (2) conjugation/tense phases are rewired to drill across the power-verb paradigm classes; (3) every unit gains a licensing set.
- **Licensing set** is first-class unit data: the enumerated allowed verb forms (specific conjugated forms, not tense names), allowed constructions, and allowed vocabulary (ambient set + window + previously licensed words). Computed from the DAG plus the unit's own grant; stored, versioned, inspectable.
- **Ambient set** is an explicit curated artifact: the small day-0 grammar and vocabulary base (articles, gender basics, plurals, negation with "no", core cognate noun/adjective patterns) licensed everywhere. Its contents are settled during curriculum authoring, not at runtime.
- **Power verb list (~45 verbs)** covering: the irregular core (ser, estar, tener, hacer, ir, ver, poder, dar, saber, poner, decir, haber, venir, traer, salir, oír, caer, pedir, seguir, servir, querer…), one or two exemplars per regular family, per spelling-change class (-car/-gar/-zar, -cer, -gir, -uir), and per stem-change family across all three conjugations. These verbs are curriculum citizens: tense/mood units drill their forms class by class.
- **Soft gating preserved from v1:** all units startable at all times; prerequisite warnings informational only; completed units never re-lock.

### Lexicon & Vocabulary Window

- **Two-tier lexicon.** Power verbs live in the grammar curriculum. All other content words flow through the window. Function words are excluded from the bank entirely (grammar practice covers them implicitly).
- **Bank:** frequency-ranked content words, curated (no proper nouns, no archaic forms, no function words) — the full content-word yield of the frequency corpus (~1,600 words), not a few-hundred-word subset. Throughput modeling shows a fast learner consumes ~150 words/month through mingled practice, exhausting a 500-word bank by month 3 of a ~10-month curriculum; the full list sustains the Words track through curriculum end (~1,300–1,500 words learned). Bank exhaustion, if reached, is framed as a designed graduation moment into self-directed vocabulary acquisition, not an error state.
- **Window mechanics:** soft target of ~20 active words; intake never hard-blocked; advisory health bands communicate the reps-per-word tradeoff as the window widens. Entry is frequency-ordered within part-of-speech lanes (approximately half nouns, a quarter verbs, a quarter adjectives/adverbs).
- **Graduation:** a word exits the window after approximately 4 successful exercise uses spread across at least 3 distinct days (both values config-tunable). The day-spacing floor is non-negotiable: speed comes from window width, never from shortened spacing.
- **Word pipeline states are reversible by design.** Graduation is not a terminal state: the state model must support a graduated word re-entering the window. No re-entry feature ships at launch, but the schema and state transitions must not assume one-way flow — this keeps post-launch candidates (demotion on repeated lexical failure, learner self-certification corrections, decay handling) features rather than migrations.
- **Stuck words:** approximately 3 failures flag a word; stuck words are offered as a brief multiple-choice warm-up before sessions and are deprioritized (not removed) in generation scheduling.
- **No thematic/scenario organization of vocabulary anywhere in the product.** Variety in generated content comes from slot specifications, not topical seeds.

### Content Generation Pipeline

- **Generate → validate → repair, per unit, in the background.** Generation is never invoked mid-session. Streaming persistence, adjacent-unit prefetch, and prompt-caching behaviors carry over from v1.
- **Generator input is a positive spec:** the unit's licensing set (enumerated allowed forms — never prose like "present tense only"), the active window words, per-item slot specifications (target skill, stacked skills, grammatical person, polarity, sentence type, sense variation for polysemous verbs), and existing items for dedup context.
- **One-unknown rule (load-bearing):** every item may carry difficulty on exactly one axis. Items drilling a not-yet-mastered structure draw content words only from consolidated vocabulary (graduated words, or window words with prior successful uses); window words in their early encounters appear only in items whose structures are all at mastery. The slot spec declares which axis an item's unknown lives on, and the validator rejects items that violate it, using learner state at generation time. This rule is what makes mingled vocabulary/grammar practice safe — without it, vocabulary failures masquerade as grammar failures.
- **Validator: LLM analyzes, code judges.** A separate LLM call produces a structured linguistic inventory per item (every verb form with lemma/tense/person, constructions used, content-word lemmas). Deterministic code then performs set-membership checks against the licensing set and window, verifies the target skill is actually exercised, and checks near-duplication and slot-spec conformance. Failed items are regenerated with the specific violation named. Mis-analysis fails safe: it causes needless regeneration, not leaked grammar.
- **Variants authored at generation time:** each item carries a validated list of accepted correct answers (clitic-placement alternates, optional subject pronouns, justified synonyms). Variants must preserve the target structure — structure-avoiding correct answers are deliberately excluded from the variant list so they route to LLM judgment and the nudge path.
- **Exercise formats at launch:** free translation (spine, majority of items), forced-choice (selection-skill units, deterministic evaluation), paradigm micro-drills (verb form production, deterministic evaluation, feeds the conjugation map). Cloze and transform formats are deferred.

### Evaluation

- **Tier 0 — deterministic match:** learner answer is normalized (accents stripped, case folded, ¿¡ and punctuation dropped) and compared against normalized canonical + variants. Match → correct instantly, offline-capable. Accent/orthography discrepancies in an otherwise-matching answer produce deterministic informational remarks. Leniency rules are enforced by this code path and are not model-discretionary.
- **Tier 1 — decomposed LLM judgment** (only for answers Tier 0 cannot match): three independent judgments with evidence — (a) is it grammatical Spanish? (b) does it convey the English cue's meaning? (c) does it use the item's target structure? Correct = a ∧ b. Correct-but-structure-avoiding (¬c) yields a nudge remark, zero mastery credit for the target skill, and re-serving of that skill. Wrong answers additionally receive a classification into a closed error enum (~12 categories: verb-form, clitic-placement, clitic-choice, agreement-gender, agreement-number, lexical-choice, mood-selection, tense-selection, word-order, omission, orthography) with an evidence span. Code — not the model — maps error categories onto curriculum skill tags, validated against the registry at write time.
- **Tier 2 — appeal:** every wrong verdict carries an "I think I was right" affordance. Appeal re-runs evaluation on a reasoning-grade model with a prompt that explicitly entertains learner correctness. Overturns retroactively correct the attempt log (mastery recomputes automatically since all state derives from the log). All appeals and outcomes are logged as regression cases.
- **Session flow: batched review with eager evaluation.** The learner types through items uninterrupted; Tier 0 resolves locally per item and Tier 1 calls fire in the background as answers are submitted. The end-of-session review screen is ready immediately on session end. Offline: attempts persist unevaluated; evaluation runs on reconnect; v1's pending-session banner behavior carries over.

### Mastery & Scheduling

- **Mastery per skill:** exponentially recency-weighted accuracy (half-life on the order of 15 attempts or 2 weeks, whichever is shorter; tunable), mastered at ≥80% weighted accuracy with a minimum of ~6 genuine demonstrations. Structure-avoided items and appeal-overturned verdicts are handled before mastery computation. Derived entirely from the attempt log at runtime; nothing stored.
- **Session queue assembly carries over from v1** (current unit / recent-units window / long-tail mix, ratios config-tunable) and additionally weaves in window-word scheduling and stuck-skill resurfacing.
- **Deliberate practice carries over from v1's design:** separate, named-tag (blocked) mode versus anonymous interleaved regular sessions; entry from the home screen weak-skills surface and from the conjugation map.

### Models

- **Capability-tier policy, not pinned models:** generator, validator-analyzer, and Tier 1 evaluator all run on the current frontier tier; Tier 2 appeals run on a reasoning-grade model. Model identifiers live in configuration and are never hardcoded. Rationale: content and evaluation quality are the product; single-user cost is trivial; v1 proved that economizing on judgment roles is fatal.

### UI

- **Launch screens:** home (two track cards + weak-skills pill + pending-session banner), practice session, end-of-session review (with appeal affordance and per-correct-item vocabulary annotations), unit list and unit detail (description, soft prerequisite warning, notes), words intake flow, and the conjugation map.
- **Conjugation map at launch is minimal and read-only:** power verbs × unlocked tense/mood columns, cells shaded by recency-weighted accuracy, tap a region to start a paradigm drill. Per-cell history, decay animation, and stale-cell resurfacing are deferred.
- **Visual design, microcopy tone, and keyboard interaction model carry over from v1's spec** (studious calm, serif content / sans chrome, muted palette, batched-flow keyboard bindings).

---

## Out of Scope

- Multi-user support, authentication, cloud sync
- Audio, listening, or speaking exercises
- Mobile platforms
- Cloze and transform exercise formats (deferred to post-launch)
- Full-featured conjugation map (per-cell drill-down, history, decay visuals)
- A dedicated polysemy / word-senses module — sense variety is handled inside generation slot specs and collocation errors are handled by evaluation remarks; telemetry may later justify individual late-curriculum contrast units
- Thematic or scenario-based vocabulary organization (explicitly rejected)
- SRS flashcard system (explicitly removed from v1)
- Placement / test-out quizzes for returning learners
- Onboarding flow
- Settings UI beyond config-file tunables

---

## Further Notes

- **The licensing set is the keystone artifact.** It serves generation (positive spec), validation (judgment criteria), and the conjugation map (column unlock order). Most of v2's ordering guarantee reduces to the quality of these sets; they should be authored and reviewed as carefully as the curriculum itself.
- **The closed error enum's final category list** is settled during implementation, with the constraint that it stays small (~12), mutually exclusive in the common case, and mappable onto curriculum tags in code.
- **Build order recommendation:** licensing sets + validator first (everything downstream trusts them), then tiered evaluation with the appeal flow, then the slot-spec generator over the vocabulary window, then mastery/deliberate-practice retargeting, then the conjugation map.
- **Regression suites from day one:** the v1 evaluation log (including every known unjust verdict) seeds the evaluator suite; every v2 appeal grows it. The v1 item bank exercises the validator.
- **Config-tunable defaults** (window target, graduation thresholds, stuck threshold, mastery half-life and floor, queue ratios, batch sizes) ship as constants in one place; no settings UI at launch.
- **Post-launch candidates** — deliberately parked, not forgotten. Each rides entirely on machinery this PRD already builds (attempt log, error enum, licensing sets, validator); none requires new foundations, and none ships at launch:
  - _Error-corpus drills:_ error-spotting exercises ("find the mistake") and verbatim resurrection of the learner's own past wrong sentences, generated from the attempt log and error classifications.
  - _Sentence expansion ladders:_ a kernel sentence grown one licensed skill per step (Quiero comer → Quiero comerlo → ¿Por qué no quieres comerlo ahora?), teaching grammar as accretive operations; near-deterministic per-step evaluation.
  - _Fluency sprints:_ timed rounds of exclusively mastered material to train automatization; adds response-time as a second mastery dimension ("accurate but slow") that the accuracy-only model cannot see.
  - _"Why?" micro-dialogues:_ an on-demand, two-turn Socratic exchange about a specific reviewed sentence — explanation as pull, not push.
  - _Free composition:_ open writing prompts constrained to licensed material, checked by the validator, judged by the evaluator with the appeal safety net — the only mode practicing retrieval without a cue.
  - _Mark-as-known + demotion:_ learner self-certifies words (cognates, prior knowledge) to skip the window entirely; curated false-friend flags interpose a one-screen warning on the dangerous cases; any graduated word accumulating repeated lexical-choice failures in incidental use re-enters the window. Together these let extreme learners spend deliberate scheduling only on words that need it (relies on the reversible state model above).
- Reference documents: v1 PRD and schema decisions (`plans/`), Lessons MOC v2, the power-verb list (this PRD's curriculum section), and the v1 post-mortem analysis conducted in-session prior to this PRD.
