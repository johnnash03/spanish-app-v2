# Lesson Exercise Generation Prompt
*Pre-dev artifact #14 — Tier 4 Prompts*

Generates exercise items for a unit given skill tag, stack ratio, and prereqs.
Called once per unit when building or replenishing the exercise bank — not on every
learner interaction.

---

## Model Configuration

| Setting | Value |
|---------|-------|
| Model | GPT-4o |
| Temperature | 0.7 |
| Output format | Structured outputs (JSON Schema enforced) |
| Prompt caching | Yes — system prompt is static, user message is dynamic |

---

## Output Schema

Array of `ExerciseItem` objects without `id` — UUID is assigned server-side after generation:

```typescript
interface GeneratedItem {
  source: string;        // English cue shown to the learner
  canonical: string;     // Correct Spanish answer
  primaryTag: string;    // The skill being drilled (must match unit's skillTag)
  stackedTags: string[]; // Prior skills mixed in; empty for minimum-pair items
}
```

---

## System Prompt Structure

### Part 1 — Role and task
```
You are a Spanish language exercise author for a translation practice app.
Your job is to generate a set of English → Spanish translation exercises for a single
drill unit. Each exercise has an English cue (shown to the learner) and a canonical
Spanish answer (used server-side for evaluation).

The learner translates English sentences into Spanish. Exercises target one primary skill
and optionally combine it with prior skills (stacking).
```

### Part 2 — Difficulty curve rules
```
Generate items in the following order:

Items 1–3: MINIMUM-PAIR
- stackedTags must be empty []
- Only the primary skill varies
- Sentences should be simple and isolate the target construction cleanly

Items 4–10: LIGHT STACKING
- stackedTags must contain exactly one tag from the available stacking tags
- Introduce one prior skill alongside the primary skill

Items 11+: FULL STACKING
- stackedTags must contain 2–3 tags from the available stacking tags
- Combine the primary skill with multiple prior skills simultaneously

BACKGROUND VOCABULARY RULE:
The learner has mastered everything up to this unit. Background vocabulary (any
construction listed under "Background vocabulary" in the user message) may appear
freely in any item — including minimum-pair items — without being added to stackedTags.
Vary background vocabulary naturally across items. Do not repeat the same opener,
verb, or construction in every sentence just because it appears in the stacking tags.
```

### Part 3 — Style guide
*(Full content from artifact #18 — inject verbatim into system prompt)*

```
STYLE RULES:
1. Tone: neutral everyday English — conversational, not formal or slangy.
2. Vocabulary: simple A2-B1 level. Vocabulary should not be an additional challenge.
3. Length: natural, not artificially stripped or padded. Length follows from stacking complexity.
4. Person: vary grammatical person naturally across items. Don't default to first person only.
5. Canonical format: omit subject pronouns by default ("Quiero comer", not "Yo quiero comer").
6. Ambiguity: prefer clear, contextually grounded cues. Add context when a sentence could
   translate two valid ways that test different skills.
7. Dialect: neutral Latin American Spanish. Use 'ustedes' not 'vosotros', 'tú' not 'vos'.
   Avoid regionally marked vocabulary.
```

---

## User Message Format (per call)

```
Unit skill: {primaryTag} — {primaryTagDescription}
Phase: {phase}
Stack ratio: {stackRatio}% of items should be stacked
Items to generate: {count}

Available stacking tags (deliberately test these — include in stackedTags):
- {prereqTag1} — {description1}
- {prereqTag2} — {description2}
...

Background vocabulary (use freely in sentences, do NOT include in stackedTags):
- {bgTag1} — {description1}
- {bgTag2} — {description2}
...

Existing English cues to avoid:
- "{existingSource1}"
- "{existingSource2}"
...
```

If no existing cues (first generation): omit the "Existing English cues to avoid" section.
If no stacking tags available (Phase 1 units): omit "Available stacking tags" section and
generate all items as minimum-pair.
If no background vocabulary (unit has no learned skills beyond its direct prereqs): omit
the "Background vocabulary" section.

**How to populate background vocabulary:** include all tags the learner has mastered that
are not direct prereqs of this unit and not the primary tag — i.e., the full set of
learned tags minus stacking tags minus primaryTag. In practice, pass the tags from all
units whose unitNumber is less than the current unit, excluding those already listed as
stacking tags.

---

## Design Decisions

### GPT-4o over GPT-4o-mini
Generation requires linguistic expertise and creativity — natural English cues, correct
Spanish grammar, appropriate sentence variety. Generation is called once per unit (not on
every learner interaction), so cost is not a primary concern. Quality of the exercise bank
matters more than generation speed.

### All items in one call
Generating all items together gives the model context about the full set — it ensures
variety, avoids repetition, and distributes the difficulty curve correctly. One-at-a-time
generation is context-blind about what's already been produced.

### Temperature 0.7
High enough to produce varied, natural-sounding sentences across 20 items. Low enough
to stay grounded and grammatically correct. Tune up if items are repetitive; tune down
if quality degrades.

### Explicit numbered difficulty curve instructions
More reliable than prose description. The model is told exactly how many stackedTags to
include per item group. The structured output schema enforces the field format.

### Inline prereq descriptions in user message
Only the unit's direct prerequisites are passed — not all 195 tags. Keeps the system
prompt cacheable and the user message focused.

### Stacking tags vs background vocabulary
These are two distinct concepts that must be kept separate in the user message.
Stacking tags are the direct prereqs being deliberately exercised — they go in
`stackedTags` and feed the error cascade and deliberate practice engine. Background
vocabulary is everything else the learner knows; it can appear freely in sentences
without being tracked. Conflating them (passing only stacking tags and nothing else)
causes the model to over-repeat whichever constructions are listed, because it has
no signal that other learned patterns exist and should be varied.

### Deduplication via existing source list
Passing existing English cues prevents regeneration from producing duplicate sentences.
UUID assignment happens server-side — the model only produces content fields.

### Style guide as placeholder
The system prompt structure is fully settled. Style guide content (artifact #18) slots
in as a discrete section. Finalize this prompt after artifact #18 is complete.
