# Lesson Spec Template

This template is for the machine-facing lesson specification used to generate exercises, deliberate-practice retries, and interleaved review.

The note teaches. The spec constrains.

## Design Rules

- The spec must define what is allowed, not just what is taught.
- The spec must be narrow enough that wrong answers are diagnosable.
- Interleaving should be intentional, not random.
- Skill stacking should be intentional, not accidental.
- Derived vocabulary must come from approved families only.
- If a concept is not yet stable enough for generation, mark it forbidden.

## Template

```md
# Lesson Spec {{lesson_id}}

## Metadata

- lesson_id: `{{lesson_id}}`
- title: `{{title}}`
- stage: `{{core_beginner | upper_beginner}}`
- learner_note: `{{path_to_note}}`
- primary_goal: `{{one sentence}}`

## Core Objective

{{what the learner should be able to do after the lesson}}

## New Concepts

- concept_id: `{{concept_1}}`
  label: `{{label_1}}`
  mastery_target: `{{recognize | produce | contrast | repair}}`
- concept_id: `{{concept_2}}`
  label: `{{label_2}}`
  mastery_target: `{{recognize | produce | contrast | repair}}`

## Prerequisites

- `{{prior_concept_1}}`
- `{{prior_concept_2}}`

## Allowed Vocabulary

Core function words:
- `{{word}}`

Verbs:
- `{{verb_1}}`
- `{{verb_2}}`

Nouns and adjectives:
- `{{word_1}}`
- `{{word_2}}`

Approved derived families:
- `{{family_id_1}}`
- `{{family_id_2}}`

## Forbidden Or Not Yet Taught

- `{{forbidden_grammar_1}}`
- `{{forbidden_grammar_2}}`
- `{{forbidden_vocabulary_behavior_1}}`

## Sentence Frames Allowed

- `{{frame_1}}`
- `{{frame_2}}`
- `{{frame_3}}`

## Sentence Frames Not Allowed

- `{{frame_to_avoid_1}}`
- `{{frame_to_avoid_2}}`

## Target Error Types

- `{{error_type_1}}`
- `{{error_type_2}}`
- `{{error_type_3}}`

## Exercise Mix

- current_lesson_only: `{{40-60%}}`
- current_plus_recent_review: `{{25-40%}}`
- deliberate_error_repair: `{{10-25%}}`
- older_interleaved_review: `{{0-15%}}`

## Exercise Types Allowed

- `{{recognition}}`
- `{{controlled_translation}}`
- `{{error_correction}}`
- `{{reordering}}`

## Exercise Types Not Allowed

- `{{open_ended_paragraph_writing}}`
- `{{free_conversation}}`
- `{{untaught_multi-step_transformations}}`

## Deliberate Practice Policy

If learner misses `{{error_type_1}}`:
- remediation_prompt_shape: `{{minimal pair | micro-drill | one-error repair}}`
- retry_window: `{{same session | next session}}`

If learner misses `{{error_type_2}}`:
- remediation_prompt_shape: `{{minimal pair | micro-drill | one-error repair}}`
- retry_window: `{{same session | next session}}`

## Interleaving Policy

Compatible prior lessons:
- `{{lesson_id}}`
- `{{lesson_id}}`

Mixing rules:
- {{what can be mixed safely}}
- {{what should not yet be mixed}}

## Skill Stacking Policy

Primary new concept:
- `{{new_concept_id}}`

Stackable prior concepts:
- `{{prior_concept_id_1}}`
- `{{prior_concept_id_2}}`

Required combinations for this lesson:
- `{{new_concept + one_prior_concept}}`
- `{{new_concept + one_prior_concept}}`

Allowed but optional combinations:
- `{{new_concept + two_prior_concepts}}`

Forbidden combinations for now:
- `{{new_concept + too_many_other_concepts}}`
- `{{new_concept + unstable_prior_concept}}`

Max stack depth:
- concepts_per_item: `{{1 | 2 | 3}}`

Stacking distribution:
- isolated_new_concept: `{{30-50%}}`
- new_concept_plus_one_prior: `{{30-50%}}`
- new_concept_plus_two_priors: `{{0-20%}}`

Per-item metadata required:
- `primary_target_concept`
- `supporting_concepts`
- `stack_depth`

## Generation Constraints

- Keep prompts short and concrete.
- Use only approved vocabulary and derived families.
- Do not introduce hidden grammar.
- Keep one primary difficulty per item.
- Every generated item must declare its concept stack explicitly.
- Reject items whose stack exceeds the lesson's max stack depth.
- Reject items that combine concepts outside the allowed stacking list.
- Prefer one clearly correct answer unless the task explicitly allows variants.

## Stacking Validator Checklist

Every generated item should be checked against this list before it is accepted.

Required metadata on each item:
- `primary_target_concept`
- `supporting_concepts`
- `stack_depth`
- `uses_derived_family`
- `forbidden_check_passed`

Validation questions:

1. Is the item's main difficulty the lesson's primary target or an allowed review target?
2. Does the item use only concepts that are already taught?
3. Does the item's concept combination appear in `Required combinations` or `Allowed but optional combinations`?
4. Is the `stack_depth` less than or equal to the lesson maximum?
5. Does the item avoid every combination listed in `Forbidden combinations for now`?
6. Does the vocabulary stay inside `Allowed Vocabulary` and approved derived families?
7. If a derived family is used, is that family allowed for this lesson and safe at this stack depth?
8. Does the item avoid hidden grammar not declared in the lesson spec?
9. Can a teacher point to one clear reason the learner got the item wrong?
10. If the learner misses the item, can the error be routed to one primary error type?

Reject the item if any answer is `no`.

## Hidden Grammar Checklist

Reject the item if it silently introduces any of the following:

- an untaught tense or mood
- an untaught pronoun pattern
- an untaught reflexive construction
- an untaught preposition pattern
- an untaught agreement pattern
- a lexical item outside the allowed vocabulary policy
- an alternative answer that depends on untaught grammar

## Stack Depth Guidance

Use this as the default policy unless a lesson explicitly overrides it:

- depth `1`: isolated concept control
- depth `2`: new concept plus one stable prior concept
- depth `3`: only when both prior concepts are already stable and error attribution remains clear

Warnings:

- Depth `3` should be rare in core beginner lessons.
- If an item feels natural but the cause of failure would be ambiguous, it is over-stacked.
- If a generated sentence needs a long teacher explanation to justify why it belongs in the lesson, reject it.

## Answer Key Constraints

- Accept these variants:
  - `{{variant_1}}`
  - `{{variant_2}}`
- Reject these predictable errors:
  - `{{error_form_1}}`
  - `{{error_form_2}}`

## Teacher Notes

- {{sequencing rationale}}
- {{what this lesson unlocks later}}
- {{residual risk to monitor}}
```

## High-Value Fields

If you only keep a small number of fields, do not drop these:

- `Core Objective`
- `Allowed Vocabulary`
- `Forbidden Or Not Yet Taught`
- `Sentence Frames Allowed`
- `Target Error Types`
- `Exercise Mix`
- `Interleaving Policy`

These are the fields that keep generation disciplined.

Add these as soon as you start composing lessons:

- `Skill Stacking Policy`
- `Per-item metadata required`

## Common Failure Modes

- Spec names a grammar point but does not define allowed vocabulary.
- Spec allows derivation without approved word families.
- Spec interleaves lessons that create ambiguity about the real source of an error.
- Spec says lessons "build on each other" but never states the required combinations.
- Generator creates only isolated items, so learners never practice composition.
- Generator over-stacks items, so errors become ambiguous and discouraging.
- Generator creates technically valid items that sneak in hidden grammar.
- Generator produces stacked items without metadata, so validation becomes guesswork.
- Spec mixes too many new ideas, so remediation becomes vague.
