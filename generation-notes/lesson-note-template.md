# Lesson Note Template

This template is for the learner-facing note that appears before the exercise.

Its job is not to explain everything. Its job is to make the upcoming exercise feel fair, focused, and learnable.

## Design Rules

- Teach one main structure per lesson.
- Keep the note short enough to read in one sitting.
- Introduce only the vocabulary needed to understand and practice the structure.
- If a derivation rule appears, restrict it to a safe whitelist of examples.
- Every lesson should prepare both the first attempt and the later deliberate-practice loop.
- Every example should stay inside already-taught grammar unless the lesson explicitly introduces the new grammar.
- Make clear what this lesson can already combine with from earlier lessons.

## Recommended Size

- 1 core pattern
- 1 supporting pattern at most
- 4 to 8 new lexical items
- 6 to 12 worked examples
- 2 to 4 warning points

## Template

```md
# Lesson {{lesson_id}}: {{title}}

## What You Will Be Able To Say

- {{short payoff sentence 1}}
- {{short payoff sentence 2}}
- {{short payoff sentence 3}}

## Core Pattern

{{one short explanation in plain English}}

Pattern:
- `{{frame_1}}`
- `{{frame_2}}`

## Why This Matters

{{one short paragraph explaining what this unlocks}}

## Build On What You Already Know

This lesson combines with:
- `{{prior_skill_1}}`
- `{{prior_skill_2}}`

In this lesson, we will combine them like this:
- `{{combined_frame_1}}`
- `{{combined_frame_2}}`

Do not combine this lesson yet with:
- `{{too_early_combination_1}}`

## Word-Building Shortcut

Status: `safe` / `recognition only`

{{describe the shortcut briefly}}

Use only these examples in this lesson:
- `{{english_1}} -> {{spanish_1}}`
- `{{english_2}} -> {{spanish_2}}`
- `{{english_3}} -> {{spanish_3}}`

Do not assume the rule works for every English word.

## New Vocabulary

Core structure words:
- `{{word}}` = {{meaning}}

Verbs:
- `{{word}}` = {{meaning}}

Nouns / adjectives:
- `{{word}}` = {{meaning}}

## Watch Out

- {{predictable beginner mistake 1}}
- {{predictable beginner mistake 2}}
- {{predictable beginner mistake 3}}

## Worked Examples

- `{{example_1}}` = {{meaning_1}}
- `{{example_2}}` = {{meaning_2}}
- `{{example_3}}` = {{meaning_3}}
- `{{example_4}}` = {{meaning_4}}
- `{{example_5}}` = {{meaning_5}}

## Combined Examples

These examples stack today's skill with earlier skills:

- `{{stacked_example_1}}` = {{meaning_1}}
- `{{stacked_example_2}}` = {{meaning_2}}
- `{{stacked_example_3}}` = {{meaning_3}}

## Sentence Factory

Build many sentences with this frame:
- `{{frame}}`

Substitute with:
- `{{slot_option_1}}`
- `{{slot_option_2}}`
- `{{slot_option_3}}`

## Before You Start The Exercise

Make sure you can:
- {{check_1}}
- {{check_2}}
- {{check_3}}

## If You Get Things Wrong

If the mistake is about form:
- {{repair cue for structure}}

If the mistake is about vocabulary:
- {{repair cue for word family or meaning}}

If the mistake is about word order:
- {{repair cue for syntax}}
```

## Authoring Guidance

### Keep the note narrow

Bad:
- one lesson teaches a new tense, a new pronoun system, and ten new verbs

Good:
- one lesson teaches one reusable frame and a few controlled substitutions

### Prefer sentence factories over explanation

Bad:
- long abstract prose about grammar

Good:
- one clear frame such as `quiero + infinitive`
- one substitution set such as `comer`, `salir`, `visitar`, `verlo`

### Make deliberate practice possible

Every lesson note should make it obvious what counts as a likely error:

- wrong structure
- wrong pronoun
- wrong spelling or accent
- wrong derivation
- wrong preposition

If the likely error is not visible from the note, the later remediation loop will be weak.

### Keep interleaving compatible

The note should quietly prepare recombination with earlier lessons.

Example:
- a lesson teaching `voy a + infinitive` should use verbs the learner already knows
- a lesson teaching direct object pronouns should use frames the learner already controls

### Distinguish interleaving from stacking

Interleaving:
- mix items from different lessons in one exercise set
- example: one item on `es`, one item on `quiero + infinitive`, one item on pronouns

Stacking:
- combine more than one already-taught concept inside the same item
- example: `Lo quiero ver.` combines a helper verb frame with pronoun placement

The learner note should preview both:
- what is new today
- what earlier skill it can now combine with safely

### Keep stacking gradual

Bad:
- the first exercise after a new concept stacks three or four moving parts

Good:
- first, isolated control of the new concept
- then, combination with one prior concept
- only later, combination with two prior concepts when both are stable
