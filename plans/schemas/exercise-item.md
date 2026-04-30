# Exercise Item Schema
*Pre-dev artifact #1 — Tier 1 Foundation*

The atomic unit of the app. Every prompt, algorithm, and DB table is shaped around this.

---

## TypeScript Interface

```typescript
interface ExerciseItem {
  id: string;           // UUID — stable across edits
  source: string;       // English cue shown to the user
  canonical: string;    // Correct Spanish answer (server-side only; passed to evaluator, never to frontend)
  primaryTag: string;   // The skill being drilled (e.g. "stem.e-ie.pres")
  stackedTags: string[]; // Prior skills mixed into this item; empty for minimum-pair items
}
```

## Derived Values

- **Difficulty** — `stackedTags.length + 1`. Not stored; computed at runtime.
  - 1 = minimum-pair (items 1–3 in a unit)
  - 2 = one prior tag (items 4–10)
  - 3+ = fully stacked (items 11+)

---

## Design Decisions

### ID — UUID
Stable identity that survives content edits. Content-derived IDs (slug, hash) would orphan progress history whenever an item is corrected. Human-readable slugs are fragile when items are reordered.

### Canonical answer — stored server-side, not sent to frontend
The canonical answer exists for the evaluator prompt, not for string matching. It is passed server-side to the evaluation LLM and never exposed to the client during exercise mode.

### No variants field
Evaluation is LLM-based, not string-matching. The evaluator LLM is anchored to the canonical answer and uses its own Spanish knowledge to accept valid alternatives (clitic placement, optional pronouns, lexical synonyms). An explicit variants list would add authoring overhead without improving reliability at this stage. Revisit if LLM evaluation proves inconsistent on specific variant classes.

### primaryTag vs flat tags array
`primaryTag` is the skill being drilled. `stackedTags` are prior skills mixed in. The split is required by downstream algorithms: error cascade logic targets the micro-skill being drilled, deliberate practice retries target a specific failing tag. A flat `tags[]` loses that signal.

### No difficulty field
Fully derived from `stackedTags.length`. Storing it explicitly creates a field that can drift out of sync with the actual stacking. The generation prompt governs the difficulty curve at authoring time.

### No unitId on item
The unit holds a list of item IDs, not the reverse. Items are self-contained and may be referenced by multiple contexts (original unit queue, deliberate practice retry queue). Embedding `unitId` would couple the item to a single unit.

### No display metadata
All items share the same instruction ("Translate this sentence"). This is static UI copy that lives in the frontend and is not part of the data model.

### No timestamps or provenance
Deferred. The pilot exercise bank (artifact #19) is small enough that authoring provenance is not critical. Add `createdAt`, `updatedAt`, and `generatedBy` if auditing or regeneration workflows require it.

---

## Implementation Note

TypeScript interface is the source of truth. Derive a JSON Schema from it using `zod` for:
- Runtime validation at API boundaries
- Structured output schema injected into LLM generation prompts
