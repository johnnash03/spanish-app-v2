# Curriculum data — authoring notes (S3, #34)

This directory is the source of truth for the v2 curriculum. Everything here
is validated fatally at startup by the loader (`../loader.rs`): unknown
references, DAG cycles, duplicate grants, cognate units, and non-monotonic
licensing all refuse to boot the app.

Authored against **Lessons MOC v2** (`LessonsMOC.v2.md`) and the frozen v2
PRD (#31). This slice covers MOC Phases 1–4 (units 1–18: openers,
direct-object clitics, indirect/two-pronoun clitics, question formation).
Later phases are authored in subsequent slices on the same conventions.

## Ambient set (`ambient_set.json`, v2) — settled contents

The day-0 base licensed in every unit from the first exercise on. Per the
PRD it contains only what a zero-knowledge learner needs for natural
sentences with no hidden leaks:

- **Articles** — el/la/los/las, un/una/unos/unas (`art.def`, `art.indef`).
- **Gender & plural basics** — `gender.agreement.basic`,
  `plural.formation.basic`. These are pattern licenses, not word lists.
- **Negation with `no`** — `neg.no.preverbal`. "No quiero esperar" is legal
  from unit 1; tampoco waits for `opener.quiero.neg`.
- **Subject pronouns** — yo/tú/él/ella/usted/nosotros/nosotras/ellos/ellas/
  ustedes, plus `pron.subject.optional` (subjects are dropped by default,
  used for emphasis or disambiguation — Language Transfer lesson 5). The
  pronouns are inert until a unit licenses a matching verb form.
- **Particles** — y, o, sí, también.
- **Cognate patterns** — the five v1 patterns as pattern references
  (`cognate.*`), never drill units or stacking tags. They license the
  cognate-derived adjective/noun space (normal, importante, posible, …) for
  natural sentences without flooding vocabulary.

**Deliberate exclusions** (review these at sign-off):

- **No verb forms.** Not even `es`, although Language Transfer teaches "Es
  ilegal" in lesson 2. The PRD's ambient enumeration has no verbs, and ser
  is curriculum material (MOC Phase 11). If day-0 "es + cognate adjective"
  sentences are wanted, `es` should be an explicit ambient grant — flagged
  as an open question below.
- **No content words.** Content vocabulary comes from unit seed grants and
  the learner's window, never from the ambient set.

## Power-verb registry (`power_verbs.json`, v2) — 45 verbs, final

Curriculum citizens: their conjugated forms are licensed cell by cell by
units, they populate the conjugation map, and learning one exemplar is meant
to unlock its whole class. Coverage rationale, class by class:

| Class                 | Verbs                                                                                                             | Why these                                                                                                                                                                                                                                                                |
| --------------------- | ----------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `irregular-core` (18) | ser, estar, tener, hacer, ir, ver, poder, dar, saber, poner, decir, haber, venir, traer, salir, oír, caer, querer | The PRD's named irregular core: one-off paradigms that must each be learned directly. decir explicitly included (user story 29). querer/poder/tener/venir are also stem-changers, but their strong preterites and irregular futures make them core, not class exemplars. |
| `regular-ar` (3)      | hablar, trabajar, gustar                                                                                          | Two plain exemplars per the PRD's "one or two"; gustar added because MOC Phase 22 drills its inverted construction with enumerated forms (gusta/gustan), which requires registry membership.                                                                             |
| `regular-er` (2)      | comer, deber                                                                                                      | deber doubles as the `opener.debo` citizen — opener units grant enumerated forms, so deber must be registered.                                                                                                                                                           |
| `regular-ir` (2)      | vivir, escribir                                                                                                   | escribir also carries the irregular participle _escrito_ (MOC unit 72).                                                                                                                                                                                                  |
| `spelling.-car` (2)   | buscar, tocar                                                                                                     | Preterite _busqué/toqué_ (MOC unit 77).                                                                                                                                                                                                                                  |
| `spelling.-gar` (2)   | llegar, pagar                                                                                                     | _llegué/pagué_; pagar is already drilled in `opener.tengo-que`.                                                                                                                                                                                                          |
| `spelling.-zar` (1)   | empezar                                                                                                           | _empecé_; doubles as an e→ie stem-changer (MOC unit 33).                                                                                                                                                                                                                 |
| `spelling.-cer` (1)   | conocer                                                                                                           | _conozco_ (MOC unit 39); saber-vs-conocer selection (unit 43).                                                                                                                                                                                                           |
| `spelling.-cir` (1)   | traducir                                                                                                          | _traduzco_ plus the j-stem preterite _traduje_ (MOC units 39, 79).                                                                                                                                                                                                       |
| `spelling.-gir` (1)   | elegir                                                                                                            | _elijo_; doubles as e→i.                                                                                                                                                                                                                                                 |
| `spelling.-guir` (1)  | seguir                                                                                                            | _sigo_; doubles as e→i and is the Phase 29 periphrasis verb (_seguir + gerund_). Named in the PRD core list; classified here so the -guir spelling class has its exemplar.                                                                                               |
| `spelling.-uir` (1)   | construir                                                                                                         | _construyo_ (y-insertion).                                                                                                                                                                                                                                               |
| `stem.e-ie` (4)       | pensar (ar), entender (er), preferir (ir), sentir (ir)                                                            | One per conjugation, per the PRD. sentir added as second -ir exemplar because reflexive _sentirse_ anchors MOC Phase 10.                                                                                                                                                 |
| `stem.o-ue` (3)       | encontrar (ar), volver (er), dormir (ir)                                                                          | volver also carries participle _vuelto_; dormir also shows the preterite 3rd-person vowel shift (_durmió_).                                                                                                                                                              |
| `stem.e-i` (2)        | pedir, servir                                                                                                     | -ir only (the family has no -ar/-er members). Both named in the PRD core list; classified as the e→i exemplars.                                                                                                                                                          |
| `stem.u-ue` (1)       | jugar                                                                                                             | The only u→ue verb in the language; also -gar spelling (_jugué_).                                                                                                                                                                                                        |

Count: 18 + 3 + 2 + 2 + 2 + 2 + 1 + 1 + 1 + 1 + 1 + 1 + 4 + 3 + 2 + 1 = 45.

Verbs the PRD lists in the irregular core that are classified by paradigm
family instead: **pedir, servir** (stem.e-i.ir), **seguir**
(spelling.-guir). The `class` field names the family a verb exemplifies for
the conjugation map; membership in the registry is what makes a verb a
curriculum citizen, and all three are in.

## Units (`units.json`, curriculum v2) — conventions

- **Grants are deltas.** A unit licenses only what is new; effective
  licensing = ambient ∪ ancestors ∪ own grant, and the loader rejects
  re-grants ("which unit teaches this?" must have one answer).
- **Power-verb infinitives are enumerated** (`{lemma, "inf", surface}`).
  The open `inf` vocab slot (granted once, by `opener.quiero`) covers only
  open-class vocabulary verbs.
- **Function words ride construction tags, never vocab grants.** Clitic
  pronouns (lo/la/los/las/me/te/nos/le/les/se), question words, tampoco,
  and "si" are carried by their construction (`clitic.do.sg.attach-to-inf`,
  `question.wh.fronting`, `neg.tampoco`, …). Two reasons: the PRD excludes
  function words from the vocabulary system entirely, and `la/los/las`
  would collide with the ambient articles under the one-granting-source
  rule. Unit `vocab` grants are content-word seeds only.
- **Construction ids are distinct from unit ids** (unit
  `clitic.both.se-lo` grants construction `clitic.both.se-lo-substitution`)
  so the two namespaces never read ambiguously.
- **Conjugated forms enter exactly where the MOC's drill examples demand
  them**, not by paradigm: 1sg openers in Phase 1; 3sg quiere/puede/tiene at
  unit 10 ("He wants to visit me"); 2sg quieres/puedes/debes/tienes/vas at
  unit 16 ("Do you want to come?"). Paradigm walking proper starts in MOC
  Phase 5.

### Deviations from the MOC prerequisite table

Licensing flows along prerequisite edges, and every element has exactly one
granting source — so a unit can only use material reachable through its
prereqs. The MOC's looser edges were written for unlock gating (which is
soft anyway: all units are startable at all times). Deviations, all
additive:

1. **Openers are chained** (quiero → puedo → debo → tengo-que → voy-a)
   where the MOC lists units 3–6 with no prereqs. Parallel openers could
   not share the infinitive slot or each other's seed verbs (e.g. "I have
   to **leave**" needs salir, granted by `opener.puedo`). Matches the
   Language Transfer teaching order.
2. **`opener.mixed` requires `opener.quiero.neg`** (MOC lists 1,3,4,5,6) so
   negative items mix into the interleave.
3. **Clitic and question phases hang off `opener.mixed`** (MOC hangs unit 8
   off unit 1 and unit 10 off 1,3) because the MOC's own drill examples
   range over all five openers ("I'm **going to** invite them", "I **have
   to** call you").
4. **`clitic.both.attach` requires `clitic.io.attach`** (MOC says 11):
   "He wants to **give** it to me" needs dar, granted at unit 12.
5. **`question.embedded` additionally requires `clitic.do.person.attach`**
   (MOC says 17 only): "I want to know if she **wants** to come" needs
   quiere, granted at unit 10.

## Learner sign-off checklist (#34)

This is your curriculum. Review and record sign-off (or change requests) on
issue #34:

- [ ] Ambient set: agree verbs stay out day-0? (Open question: license
      ambient `es` for "Es normal"-type sentences, per Language Transfer
      lesson 2?)
- [ ] Ambient set: subject-pronoun list and particles (y, o, sí, también)
      feel right as day-0 material?
- [ ] Power verbs: 45 lemmas — any you'd swap? (e.g. tocar↔sacar,
      entender↔perder, encontrar↔contar, escribir↔abrir)
- [ ] Unit seed vocabulary: the content verbs granted per unit (esperar,
      intentar, cancelar, … explicar) are words you actually want to practice?
- [ ] Opener chaining and the other four MOC deviations above acceptable?
- [ ] 3sg forms at unit 10 / 2sg forms at unit 16 — happy with where
      he/she/you forms enter?

Inspect any unit's effective licensing with:

```sh
cargo run --bin dump_licensing -- question.embedded
```
