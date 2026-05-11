# Deliberate Practice Generation Prompt
*Pre-dev artifact #15 — Tier 4 Prompts*

Generates retry items targeting specific failing micro-skills. Called when the deliberate
practice queue needs new items for weak tags. Separate from lesson generation — not a
parameterized variant.

---

## Model Configuration

| Setting | Value |
|---------|-------|
| Model | GPT-4o |
| Temperature | 0.4 |
| Output format | Structured outputs (JSON Schema enforced) |
| Prompt caching | Yes — system prompt is static, user message is dynamic |

---

## Batching Rule

**Maximum 3 weak tags per call.** Beyond 3, generation quality degrades as the model
juggles too many distinct error patterns simultaneously.

If the learner has N weak tags:
- Split into `ceil(N / 3)` calls
- Each call handles ≤3 tags
- Merge all generated items into one deliberate practice queue

Weak tags are sorted by error rate descending (artifact #11) before batching — the most
critical tags go in the first call.

---

## Output Schema

Flat array of `GeneratedItem` objects. Each item carries its `primaryTag` for attribution:

```typescript
interface GeneratedItem {
  source: string;        // English cue shown to the learner
  canonical: string;     // Correct Spanish answer
  primaryTag: string;    // Must match one of the weak tags in this call
  stackedTags: string[]; // Always empty [] — deliberate practice is minimum-pair only
}
```

---

## System Prompt Structure

### Part 1 — Role and task
```
You are a Spanish language exercise author for a targeted remediation app.
The learner has demonstrated consistent errors on specific Spanish skills.
Your job is to generate minimum-pair English → Spanish translation exercises that
directly target each failing construction.

For each skill provided, study the learner's actual errors to understand the specific
mistake pattern, then generate exercises that confront that pattern directly.
```

### Part 2 — Generation rules
```
1. Generate 5–8 exercises per skill.
2. All exercises must be minimum-pair: stackedTags must always be empty [].
   Do not combine multiple skills — isolate each failing construction cleanly.
3. Each exercise must set primaryTag to the skill it targets.
4. Study the learner's error examples carefully. Generate items that specifically
   address the pattern of mistakes shown — not generic exercises for the tag.
5. Vary sentence subjects, objects, and contexts across items to prevent pattern
   memorization.
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
Generate 5–8 minimum-pair exercises for each of the following weak skills:

SKILL 1: {primaryTag1} — {description1}
Learner errors:
- Asked: "{source}" → Correct: "{canonical}" → Learner wrote: "{learnerAnswer}"
- Asked: "{source}" → Correct: "{canonical}" → Learner wrote: "{learnerAnswer}"
- Asked: "{source}" → Correct: "{canonical}" → Learner wrote: "{learnerAnswer}"

SKILL 2: {primaryTag2} — {description2}
Learner errors:
- Asked: "{source}" → Correct: "{canonical}" → Learner wrote: "{learnerAnswer}"
- Asked: "{source}" → Correct: "{canonical}" → Learner wrote: "{learnerAnswer}"

SKILL 3: {primaryTag3} — {description3}
Learner errors:
- Asked: "{source}" → Correct: "{canonical}" → Learner wrote: "{learnerAnswer}"

Existing English cues to avoid:
- "{existingSource1}"
- "{existingSource2}"
```

Omit "Existing English cues to avoid" on first generation for a tag.
Include all available learner error records for each tag (typically 3–5).

---

## Design Decisions

### Separate prompt from lesson generation
Purpose, tone, and instructions differ enough to warrant separate prompts. Lesson
generation introduces a skill; deliberate practice remediates a specific failure.
Parameterizing one prompt for both would require too many conditionals and make each
harder to tune independently.

### Maximum 3 tags per call
Each tag carries ~1,350 tokens of context (description + error examples + generated
output). Beyond 3 tags, the model loses focus on individual error patterns and generates
generic items rather than targeted ones. Batching into ≤3 tag calls with a merge step
preserves quality.

### Wrong answers passed as full error context
Bare wrong answer strings are ambiguous. The model needs source + canonical + learnerAnswer
to understand the error pattern. This is why `learnerAnswer` is stored in `attempt_log`
(artifact #7).

### Minimum-pair only — no stacking
The learner is failing this skill. Adding stacking pressure during remediation compounds
difficulty at the wrong moment. Strip away everything except the failing construction.
Stacking returns once the tag is no longer weak.

### Temperature 0.4
Lower than lesson generation (0.7). Deliberate practice items should be focused and
targeted, not maximally varied. Enough variety to avoid repetition; constrained enough
to stay on task.

### Prioritize highest error-rate tags in first batch
If N > 3 weak tags, the most critical skills (highest error rate) go in the first call.
Ensures the most important remediation items are generated even if subsequent calls are
skipped or delayed.
