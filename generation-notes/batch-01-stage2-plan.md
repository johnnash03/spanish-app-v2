# Batch 01 — Stage 2 Execution Plan

5 steps. 22 lessons total. Each step = one LLM call that emits a contiguous slice of lessons following `docs/prompts/stage-2.md`.

**Execution rule:** when told "Execute step N", read this file, load the step's inputs, follow `docs/prompts/stage-2.md`, and emit the outputs. Do not skip steps. Between steps, wait for explicit user review/approval.

**Shared across every step:**
- **Prompt:** `docs/prompts/stage-2.md` (the Stage 2 prompt). Follow it verbatim.
- **Frozen catalogs:** `rules.yaml`, `transfer-patterns.yaml`. Do not modify.
- **Lesson plan:** `generation-notes/batch-01-lesson-plan.yaml`. Source of frontmatter; slice per step.
- **Teacher notes:** `ReferenceNotes.md`. Read only the §§ named by each lesson's `source_refs`.
- **Model settings:** Opus. Low temperature (0.2–0.3). Ascending lesson-ID order within a call.
- **Append to:** `generation-notes/batch-01-stage2-notes.md` (create on step 1; append on steps 2–5).

---

## Step 1 — Pilot (L1–L3)

**Lessons:** 3 (L1, L2, L3). All light: one tiny grammar + two transfer-pattern lessons.

**Inputs:**
- `docs/prompts/stage-2.md`
- `rules.yaml`, `transfer-patterns.yaml`
- `generation-notes/batch-01-lesson-plan.yaml` — lessons with `id: 1`, `id: 2`, `id: 3`
- `ReferenceNotes.md` §§2, 3, 4
- No prior lesson bodies (this is the seed)

**Outputs:**
- `lessons/lesson-01.md`
- `lessons/lesson-02.md`
- `lessons/lesson-03.md`
- `generation-notes/batch-01-stage2-notes.md` (new file)

**Post-step work (do this in the same response after emitting the three lessons):**
1. Pick one lesson as the proposed `<exemplar-grammar>` and one as `<exemplar-transfer>`. L1 is the natural grammar candidate (it introduces `negation-no-before-verb` alongside `-al` cognates); L3 is the natural transfer candidate (pure `-tion → -cion`).
2. Propose the exemplar edits to `docs/prompts/stage-2.md` — show the diff, do not apply until approved.

**Review checkpoint (user):**
- Read all three lessons. Does the voice match `ReferenceNotes.md`?
- Is clitic normalization correct (attached, no `cancelar lo`)?
- Are all anchor `es:` forms in the introduced vocab universe?
- Approve the proposed exemplars (or ask for different ones) before step 2.

---

## Step 2 — Early grammar (L4–L8)

**Lessons:** 5 (L4, L5, L6, L7, L8). First real grammar density: modals, postverbal DO pronouns, motion verbs.

**Inputs:**
- `docs/prompts/stage-2.md` **updated with exemplars from step 1**
- `rules.yaml`, `transfer-patterns.yaml`
- `generation-notes/batch-01-lesson-plan.yaml` — slice `id: 4` through `id: 8`
- `ReferenceNotes.md` §§4, 5, 6, 7
- **Prior bodies:** `lessons/lesson-01.md`, `lesson-02.md`, `lesson-03.md` (finalized in step 1)

**Outputs:**
- `lessons/lesson-04.md` through `lessons/lesson-08.md`
- Append to `generation-notes/batch-01-stage2-notes.md`

**Review checkpoint:**
- Voice consistent with step 1?
- Anchor coverage: every rule in `rules_introduced` has ≥1 anchor?
- Is `verb-motion-plus-a-before-infinitive` explained in the teacher's "put an `a` to show motion, even when not moving" framing? (L8 — the litmus test for teacher voice.)

---

## Step 3 — Mid grammar (L9–L14)

**Lessons:** 6 (L9, L10, L11, L12, L13, L14). The regular-conjugation spine + pronoun flexibility + first compound pronouns.

**Inputs:**
- `docs/prompts/stage-2.md` (exemplars populated)
- `rules.yaml`, `transfer-patterns.yaml`
- `generation-notes/batch-01-lesson-plan.yaml` — slice `id: 9` through `id: 14`
- `ReferenceNotes.md` §§8, 9, 10, 11, 12, 13
- **Prior bodies:** all of `lessons/lesson-01.md` through `lesson-08.md`

**Outputs:**
- `lessons/lesson-09.md` through `lessons/lesson-14.md`
- Append to `generation-notes/batch-01-stage2-notes.md`

**Review checkpoint:**
- L11's accent rule and L12's pronoun-placement-flexibility are both paradigm-heavy — did the body stay teacher-voiced or drift into textbook paradigm tables?
- L13's double pronouns (`me lo venden`) — clitic orthography clean?

---

## Step 4 — Stem changes + modals + pose-to-poner (L15–L18)

**Lessons:** 4 (L15, L16, L17, L18). Dense: `tengo que`, pose-to-poner transfer, then e→ie and o→ue stem changes.

**Inputs:**
- `docs/prompts/stage-2.md` (exemplars populated)
- `rules.yaml`, `transfer-patterns.yaml`
- `generation-notes/batch-01-lesson-plan.yaml` — slice `id: 15` through `id: 18`
- `ReferenceNotes.md` §§14, 15, 16
- **Prior bodies:** `lessons/lesson-01.md` through `lesson-14.md`

**Outputs:**
- `lessons/lesson-15.md` through `lessons/lesson-18.md`
- Append to `generation-notes/batch-01-stage2-notes.md`

**Review checkpoint:**
- L17 explicitly says stem changes "retroactively explain `quiero`" — does the body make that connection?
- L18 includes the non-stem-changing `tomar` as a trap — is that called out?

---

## Step 5 — Nosotros + irregulars + future (L19–L22)

**Lessons:** 4 (L19, L20, L21, L22). Nosotros forms, stem-no-split-in-we, `ir` fully irregular, `ver` irregular-yo, future-via-going-to.

**Inputs:**
- `docs/prompts/stage-2.md` (exemplars populated)
- `rules.yaml`, `transfer-patterns.yaml`
- `generation-notes/batch-01-lesson-plan.yaml` — slice `id: 19` through `id: 22`
- `ReferenceNotes.md` §§17, 18, 19, 20
- **Prior bodies:** `lessons/lesson-01.md` through `lesson-18.md`

**Outputs:**
- `lessons/lesson-19.md` through `lessons/lesson-22.md`
- Append to `generation-notes/batch-01-stage2-notes.md`

**Review checkpoint:**
- L19's "stem-changers don't split in we because stress moves to the ending" — teacher's actual explanation survives?
- L22's `nos vemos` — is `source: teacher` kept, orthography attached?

---

## After step 5 — Batch 01 retro

Before touching batch 2:

1. Read all 22 lessons end-to-end as a learner would.
2. Run the self-check items from `docs/prompts/stage-2.md` across the full set — not per-lesson, but aggregate (e.g., is `verb-irregular-yo-go` covered consistently in every lesson that reinforces it?).
3. List prompt edits needed for batch 2 — does `stage-2.md` need a v2? Do the exemplars need to change?
4. Decide whether the catalogs need any amendments discovered during Stage 2 (per the Stage 2 prompt, discrepancies are *reported*, not silently fixed).

Only after this retro is complete does batch 2 begin — which will use its own Stage 1 pass first, then its own Stage 2 execution plan.
