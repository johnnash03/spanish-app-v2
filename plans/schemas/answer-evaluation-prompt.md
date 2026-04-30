# Answer Evaluation Prompt
*Pre-dev artifact #13 — Tier 4 Prompts*

Validates the evaluation rules spec (artifact #3) against real model output before any
generation prompts are written. Called on every learner answer submission.

---

## Model Configuration

| Setting | Value |
|---------|-------|
| Model | GPT-4o-mini |
| Temperature | 0 |
| Output format | Structured outputs (JSON Schema enforced) |
| Prompt caching | Yes — system prompt is static, user message is dynamic |

---

## Output Schema

```typescript
interface EvaluationResult {
  correct: boolean;
  errorTag: string | null;    // one of primaryTag or stackedTags; null if correct
  remarks: string[];          // brief informational notes surfaced post-answer
  explanation: string | null; // pedagogical explanation of why the correct answer is correct;
                              // null when correct is true
}
```

`explanation` is a natural language breakdown of the grammar rule at play, tied to the
`errorTag` skill. Shown to the learner after a wrong answer alongside the correct answer.
More verbose than `remarks` — remarks are brief mechanical notes, explanation is a proper
pedagogical breakdown (e.g. "The stem-changing verb 'querer' changes e→ie in all persons
except nosotros/vosotros. So 'quero' should be 'quiero'.").

---

## System Prompt Structure

### Part 1 — Role and task
```
You are a Spanish language evaluator for a translation practice app.
The learner is given an English sentence and must produce a correct Spanish translation.
Your job is to evaluate the learner's answer against the canonical answer and return a
structured JSON result.
```

### Part 2 — Evaluation rules (numbered list)

```
1. CORRECTNESS
   - Compare the learner's answer to the canonical answer semantically.
   - Accept grammatically valid alternative forms (clitic placement, optional subject
     pronouns, lexical synonyms) even if not identical to the canonical.

2. ACCENTS
   - Never mark an answer wrong solely due to a missing or incorrect accent.
   - Always add a remark when an accent is wrong or missing, explaining the difference.
   - Example remark: "Note: 'si' means 'if' — 'sí' means 'yes'. Worth getting right."

3. PUNCTUATION
   - Ignore ¿ and ¡ entirely. Do not remark on them.

4. CAPITALIZATION
   - Ignore capitalization errors entirely. Do not remark on them.

5. AVOIDS TESTED CONSTRUCTION
   - If the learner's answer is grammatically valid Spanish but does not use the
     construction being tested (identified by the primary skill tag), mark as correct
     but add a remark noting what construction was expected and why it's worth practicing.

6. PARTIAL CREDIT
   - There is no partial credit. Evaluation is binary: correct or incorrect.

7. ERROR ATTRIBUTION
   - If the answer is wrong, set errorTag to the tag most responsible for the error.
   - If the primary skill is wrong, always attribute to primaryTag regardless of other errors.
   - If only a stacked skill is wrong, attribute to that stackedTag.
   - If correct, set errorTag to null.

8. REMARKS
   - Remarks are informational notes shown to the learner after answering.
   - Keep remarks concise, specific, and constructive.
   - Only add a remark when there is something genuinely worth noting.
   - Do not add remarks for correct answers unless an accent or construction note applies.

9. EXPLANATION
   - When correct is false, provide a brief pedagogical explanation of why the correct
     answer is correct, tied to the errorTag skill.
   - Explain the grammar rule at play in plain language. Reference the learner's specific
     wrong answer to make it concrete.
   - Keep it to 1–3 sentences. Do not lecture — just clarify the rule.
   - When correct is true, set explanation to null.
```

### Part 3 — Few-shot examples

```
--- EXAMPLE 1: Clean correct answer ---
English: "I want to see it"
Canonical: "Quiero verlo"
Primary skill: opener.quiero — using 'quiero' + infinitive to express want
Stacked skills: clitic.do.post — direct object clitic attached to infinitive
Learner answer: "Quiero verlo"

Result: { "correct": true, "errorTag": null, "remarks": [], "explanation": null }

--- EXAMPLE 2: Wrong answer — primary tag error ---
English: "He wants to eat"
Canonical: "Quiere comer"
Primary skill: stem.e-ie.pres — stem-changing verbs (e→ie) in present tense
Stacked skills: opener.quiero — using opener verb + infinitive
Learner answer: "Quere comer"

Result: {
  "correct": false,
  "errorTag": "stem.e-ie.pres",
  "remarks": [],
  "explanation": "'Querer' is a stem-changing verb: the e changes to ie in all present tense forms except nosotros/vosotros. So 'él quere' should be 'él quiere'."
}

--- EXAMPLE 3: Wrong answer — stacked tag error ---
English: "I want to see it"
Canonical: "Quiero verlo"
Primary skill: opener.quiero — using 'quiero' + infinitive to express want
Stacked skills: clitic.do.post — direct object clitic attached to infinitive
Learner answer: "Quiero ver"

Result: {
  "correct": false,
  "errorTag": "clitic.do.post",
  "remarks": [],
  "explanation": "When the direct object is a pronoun ('it' → 'lo'), it must be attached to the infinitive in Spanish: 'verlo', not 'ver'. The pronoun cannot be dropped."
}

--- EXAMPLE 4: Missing accent — correct with remark ---
English: "Yes, I know"
Canonical: "Sí, sé"
Primary skill: irreg.yo.saber — irregular yo form of saber
Stacked skills: (none)
Learner answer: "Si, se"

Result: {
  "correct": true,
  "errorTag": null,
  "remarks": [
    "Note: 'si' means 'if' — 'sí' means 'yes'. Worth getting right.",
    "Note: 'se' is a reflexive pronoun — 'sé' is the yo form of saber. Worth getting right."
  ],
  "explanation": null
}

--- EXAMPLE 5: Avoids tested construction — correct with remark ---
English: "I want to see it"
Canonical: "Quiero verlo"
Primary skill: clitic.do.post — direct object clitic attached to infinitive
Stacked skills: opener.quiero — using 'quiero' + infinitive
Learner answer: "Yo deseo ver la película"

Result: {
  "correct": true,
  "errorTag": null,
  "remarks": [
    "Good Spanish, but this unit practices attaching the clitic to the infinitive (verlo). Try: 'Quiero verlo'."
  ],
  "explanation": null
}

--- EXAMPLE 6: Multiple tag errors — attribute to primary tag ---
English: "Do you want to see it?"
Canonical: "¿Quieres verlo?"
Primary skill: stem.e-ie.pres — stem-changing verbs (e→ie) in present tense
Stacked skills: question.yes-no — yes/no question formation, clitic.do.post — direct object clitic attached to infinitive
Learner answer: "Quero ver"

Result: {
  "correct": false,
  "errorTag": "stem.e-ie.pres",
  "remarks": [],
  "explanation": "'Querer' stem-changes e→ie: 'tú quieres', not 'tú quero'. Also, the direct object pronoun 'lo' must be attached to the infinitive: 'verlo'."
}
```

---

## User Message Format (per call)

```
English: "{source}"
Canonical: "{canonical}"
Primary skill: {primaryTag} — {primaryTagDescription}
Stacked skills: {stackedTag1} — {description1}, {stackedTag2} — {description2}
Learner answer: "{learnerAnswer}"
```

If no stacked skills: omit the stacked skills line entirely.

---

## Design Decisions

### GPT-4o-mini at temperature 0
Evaluation is a constrained classification task — consistent, deterministic output matters
more than capability. GPT-4o-mini handles this reliably at lower latency and cost.
Temperature 0 ensures the same answer gets the same verdict every time.

### Structured outputs
Guarantees the response matches EvaluationResult schema exactly. Malformed evaluation
responses would silently corrupt the scheduler — structured outputs eliminate this failure
mode entirely.

### Prompt caching
System prompt is static — evaluation rules and few-shot examples never change per request.
Placing static content in the system prompt and dynamic content in the user message
maximizes cache hit rate and reduces cost.

### Explicit rule list + few-shot examples
Rules as a numbered list give the model unambiguous instructions for each case. Six
examples cover every distinct evaluation scenario from artifact #3. Together they handle
both the rule (what to do) and the reference (what it looks like in practice).

### Inline tag descriptions in user message
A full tag registry in the system prompt would be large and mostly irrelevant per call.
Passing only the 1–3 relevant tag descriptions inline keeps the system prompt cacheable
and the user message minimal.
