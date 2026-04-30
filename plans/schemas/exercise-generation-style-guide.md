# Exercise Generation Style Guide
*Pre-dev artifact #18 — Tier 5 Content*

Rules for authoring exercise items — whether by human or LLM generation prompts.
This content is injected into the system prompts of artifacts #14 and #15.
Must be stable before the pilot exercise bank (artifact #19) is authored.

---

## Style Rules

### 1. English Cue Tone
Write in neutral everyday English — conversational but not slangy.

- **Good**: "I want to call her tomorrow", "He can't find his keys", "We need to leave now"
- **Avoid formal/academic**: "One must consider the implications", "It is necessary to depart"
- **Avoid slang/colloquial**: "Wanna grab coffee?", "Gonna head out"

The register should match real-world everyday communication — what the learner will actually use.

### 2. Vocabulary
Use simple, high-frequency vocabulary (A2-B1 level). Each item tests a grammar construction — vocabulary should not be an additional challenge.

Avoid obscure nouns, verbs, or expressions that a beginner-intermediate learner might not know. When in doubt, use the simplest word that makes the sentence natural.

*(Future: a vocabulary track will supply a list of learner-acquired words to the generation prompt. When available, prefer using those words in sentences where grammatically natural.)*

### 3. Sentence Length and Complexity
Sentences should feel natural and contextually grounded — not artificially stripped or padded.

- Don't remove context that makes a sentence realistic: "I need to call her before she leaves" is better than "I need to call her" if the extra clause feels natural
- Don't add clauses purely for length that don't serve the construction being tested
- Length follows naturally from stacking complexity — minimum-pair items are naturally short, fully stacked items are naturally longer

### 4. Grammatical Person
Vary grammatical person naturally across items in a unit. Do not default to first person for every sentence. A unit's items should collectively exercise the construction across different persons (yo, tú, él/ella, nosotros, ellos) as the sentences allow naturally.

### 5. Canonical Answer Format
Omit subject pronouns by default. Match natural Spanish usage.

- **Correct**: "Quiero comer", "¿Puedes ayudarme?", "Tenemos que salir"
- **Avoid**: "Yo quiero comer", "Tú puedes ayudarme", "Nosotros tenemos que salir"

Subject pronouns are grammatically valid and accepted by the evaluator — they just shouldn't be the default in canonical answers.

### 6. Ambiguity
Prefer unambiguous English cues that clearly guide the learner toward the target construction.

If a sentence could reasonably translate two different ways that test different skills, add context to make the intended translation clear:
- **Ambiguous**: "He goes to the store" (habitual present vs progressive)
- **Clear**: "He goes to the store every day" (clearly habitual present)

When ambiguity slips through despite good authoring, the evaluator handles it gracefully — marking a valid alternative as correct with an explanation. Authors should minimize ambiguity, not eliminate it at all costs.

### 7. Dialect
Target neutral Latin American Spanish throughout.

- Use `ustedes` (not `vosotros`) for second person plural
- Use `tú` (not `vos`) for second person singular informal
- Avoid regionally marked vocabulary (e.g. prefer `tomar` or `agarrar` over `coger`)
- No Castilian-specific forms or expressions

---

## What This Guide Does Not Cover

- **Difficulty curve** (minimum-pair vs stacked item structure) — defined in the generation prompt (artifact #14)
- **Stack ratio** (how many items are stacked per unit) — defined in artifact #9
- **Tag assignment** (`primaryTag` and `stackedTags`) — governed by the unit schema and generation prompt instructions
