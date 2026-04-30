# Unit Metadata Schema
*Pre-dev artifact #2 — Tier 1 Foundation*

The container exercises live in. Defines the prereq graph, stack ratio, and interleave settings.

---

## TypeScript Interface

```typescript
interface Unit {
  skillTag: string;       // Primary key — machine-readable unique identifier (e.g. "stem.e-ie.pres")
  title: string;          // Human-readable name for UI display (e.g. "Stem-changing verbs: e→ie, present tense")
  phase: number;          // Phase this unit belongs to; used to derive stack ratio
  prerequisites: string[]; // Skill tags that must be at mastery threshold before this unit unlocks
}
```

## Derived Values

- **Stack ratio** — derived from `phase` via a progression table defined in config (30% early phases → 60% Phase 16 → 100% capstone). Not stored on the unit.
- **Unit status** (`locked | unlocked | in_progress | completed`) — derived from the learner's mastery state at runtime. Belongs in user progress schema (artifact #5), not here.

---

## Unit–Item Relationship

Items are linked to their unit implicitly via `ExerciseItem.primaryTag`. There is no `itemIds` list on the unit.

- To fetch a unit's items: query all `ExerciseItem` records where `primaryTag = unit.skillTag`
- Regenerating a unit's exercises means adding new items with the same `primaryTag`; bad items are deleted outright
- Quality of generated items is assumed to be sufficient for v1. Per-item retirement (e.g. an `active` flag) is noted as a future concern if the item bank scales or quality becomes an issue

---

## Design Decisions

### skillTag as primary key
Every downstream reference (prereq lists, error cascade, deliberate practice scheduling, mastery tracking) already uses skill tags. Introducing a separate UUID creates two IDs for the same thing. Skill tags are stable by design — they don't change when content is edited.

### prerequisites as skill tags, not unit numbers
Unit numbers are positional and fragile — inserting or reordering units breaks all numeric prereq references. Skill tags are semantically stable. The unlock logic resolves them directly against the user's mastery state.

### stackRatio derived from phase, not stored per unit
The stack ratio is a phase-level pedagogical decision: low ratio in early phases (learner is building a mental model of a new skill), high ratio in later phases (learner integrates multiple skills for fluency). It is not intrinsic to any individual unit. A single progression table in config maps phase → ratio; per-unit overrides can be added later if needed.

### phase stored on unit
The unit owns its phase membership — it doesn't change. Storing it avoids a join or external lookup every time stack ratio or phase context is needed at runtime.

### interleaveWindow is session state
The interleave window (sliding window of last 5 units) is learner-specific and dynamic. It shifts as the learner progresses. It belongs in the session schema (artifact #6), not on the unit.

### Unit status is derived
`locked | unlocked | in_progress | completed` is learner-specific. The same unit is locked for one user and completed for another. Runtime status is derived from the user progress schema (artifact #5).

### No sourceRef
Video timestamp and notes anchor linking is out of scope. Notes are considered independently generated content.

### No itemIds list
The unit-item relationship is implicit through primaryTag. Explicit ID lists would need to be kept in sync on every item addition or deletion — unnecessary coupling when a tag-based query is sufficient.
