# Lessons MOC v2 — Practice App, Drill-Unit Indexed

The app is a **practice-only** companion to a separate video + notes channel. The video carries instruction and pronunciation; the notes supplement; the app does reps. Each unit below is a **drill set** — a tagged exercise bank targeting one micro-skill — not a teaching lesson.

The spine is structural-skill order, not vocabulary-rule order. Cognate rules are not phases here; they appear as transformation patterns inside the translation drills themselves (and live explicitly in the video/notes).

## Unit anatomy

Every unit ships with:

- **Skill tag** — machine-readable identifier the deliberate-practice engine uses to schedule retries when the learner errs (`stem.e-ie.pres`, `clitic.both.se-lo`).
- **Prerequisites** — earlier tags that must be at threshold before the unit unlocks. Lets the app run linearly *or* adaptively.
- **Drill set** — 15–25 English → Spanish translation items, mixed difficulty, with accepted variants per item.
- **Stack ratio** — % of items that combine this skill with prior tags (default 30%, climbs each phase).
- **Interleave window** — sliding window of prior tags sampled into review checkpoints.

Units are pure exercise. The "note" attached in-app is a one-liner pointing to the source video timestamp and the relevant notes section.

## Tag namespace

```
opener.<verb>            quiero, puedo, debo, tengo-que, voy-a
clitic.<do|io|both>.<placement>
question.<yes-no|wh|embedded>
conj.<tense>.<family>.<person>
stem.<change>.<tense>
irreg.<form>.<class>
reflex.<construction>
ser | estar | ser-vs-estar.<distinction>
adj.<aspect>
cont.<aspect>
perfect.<aspect>
pret.<class>
imperf.<class>
pret-vs-imperf.<aspect>
future.<aspect>
cond.<aspect>
gustar.<aspect>
prep.<aspect>
cmd.<aspect>
subj.<aspect>
lex.<topic>
```

---

## Phase 1 — Openers + infinitive

The fastest path to a producible Spanish sentence: a fixed I-form opener plus the infinitive of any verb.

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 1 | `Quiero` + inf, affirmative | `opener.quiero` | — | "I want to eat / drink / wait / try / cancel / continue." |
| 2 | `Quiero` + inf, negative | `opener.quiero.neg` | 1 | "I don't want to / I don't want to either." |
| 3 | `Puedo` + inf | `opener.puedo` | — | "I can come / leave / stay / try." |
| 4 | `Debo` + inf | `opener.debo` | — | "I must go / try / wait." |
| 5 | `Tengo que` + inf | `opener.tengo-que` | — | "I have to leave / pay / call." |
| 6 | `Voy a` + inf | `opener.voy-a` | — | "I'm going to eat / call / leave." |
| 7 | Openers interleaved | `opener.mixed` | 1, 3, 4, 5, 6 | All five openers, randomized; learner must produce the right opener for the cue. |

## Phase 2 — Direct object pronouns (attached to infinitive)

Clitics enter while the verb is still in infinitive form — fewer placement choices to manage.

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 8 | `lo` / `la` after infinitive | `clitic.do.sg.attach` | 1 | "I want to see it / cancel it / find her." |
| 9 | `los` / `las` after infinitive | `clitic.do.pl.attach` | 8 | "I'm going to invite them / buy them." |
| 10 | `me` / `te` / `nos` after infinitive | `clitic.do.person.attach` | 1, 3 | "He wants to visit me / I have to call you." |
| 11 | Mixed direct-object clitics + opener | `clitic.do.attach.mixed` | 8, 9, 10 | Random direct objects across all openers. |

## Phase 3 — Indirect objects and two-pronoun structures

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 12 | `le` / `les` (to him / her / them) after infinitive | `clitic.io.attach` | 11 | "I want to speak to him / give them something." |
| 13 | Two pronouns: `me lo` / `te lo` / `nos lo` after infinitive | `clitic.both.attach` | 11 | "He wants to give it to me / sell it to us." |
| 14 | `le` + `lo` collision → `se lo` | `clitic.both.se-lo` | 12, 13 | "I want to give it to him → quiero dárselo." |
| 15 | Two-pronoun + opener interleaved | `clitic.both.mixed` | 13, 14 | |

## Phase 4 — Question formation

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 16 | Yes/no questions (intonation + `¿…?`) | `question.yes-no` | 7 | "Do you want to come? Can you wait? Do you have to leave?" |
| 17 | Wh-questions: `qué`, `quién`, `dónde`, `cuándo`, `cómo`, `por qué`, `cuánto` | `question.wh` | 16 | Across all openers. |
| 18 | Embedded questions after `saber` (`Quiero saber si / por qué / cuándo…`) | `question.embedded` | 17 | "I want to know if she wants to come." |

## Phase 5 — Present indicative, regular `-ar`

Walk the conjugation paradigm one person at a time so each ending gets isolated reps before they get mixed.

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 19 | `-ar` yo (`-o`) | `conj.pres.ar.yo` | 1 | hablo, trabajo, espero, pago. |
| 20 | `-ar` él / ella / usted (`-a`) | `conj.pres.ar.3sg` | 19 | habla, trabaja, espera. |
| 21 | `-ar` tú (`-as`) | `conj.pres.ar.tu` | 20 | |
| 22 | `-ar` ellos / ustedes (`-an`) | `conj.pres.ar.3pl` | 20 | |
| 23 | `-ar` nosotros (`-amos`) | `conj.pres.ar.nos` | 20 | |
| 24 | `-ar` all persons interleaved | `conj.pres.ar.mixed` | 19–23 | |

## Phase 6 — Present indicative, regular `-er` / `-ir`

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 25 | `-er` all persons | `conj.pres.er.mixed` | 24 | como, vendo, leo, bebo. |
| 26 | `-ir` all persons | `conj.pres.ir.mixed` | 25 | vivo, escribo, abro, recibo. |
| 27 | `-er` vs `-ir` "we" form contrast (`-emos` vs `-imos`) | `conj.pres.we.contrast` | 25, 26 | The single point where the two paradigms diverge. |
| 28 | All three families interleaved | `conj.pres.regular.mixed` | 24, 25, 26 | |

## Phase 7 — Clitic placement with conjugated verbs

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 29 | DO clitic before finite verb (`lo veo`) | `clitic.do.before-finite` | 28 | |
| 30 | IO clitic before finite verb (`le hablo`) | `clitic.io.before-finite` | 28 | |
| 31 | Two clitics before finite verb (`me lo da`, `se lo digo`) | `clitic.both.before-finite` | 29, 30 | |
| 32 | Placement choice with opener + inf (both legal: `lo quiero ver` / `quiero verlo`) | `clitic.placement.choice` | 11, 29 | Drill produces both forms for each item. |

## Phase 8 — Stem-changing verbs, present

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 33 | `e → ie`: pensar, querer, empezar, cerrar, entender, perder, preferir | `stem.e-ie.pres` | 28 | |
| 34 | `o → ue`: poder, dormir, encontrar, mostrar, volver, contar | `stem.o-ue.pres` | 28 | |
| 35 | `e → i`: pedir, servir, repetir, seguir | `stem.e-i.pres` | 28 | |
| 36 | We-form does not change | `stem.we-form.regularity` | 33, 34, 35 | "We think / we sleep / we ask" — drill specifically the form that *doesn't* change. |
| 37 | Stem-changers interleaved with regulars | `stem.pres.mixed` | 33, 34, 35, 36 | |

## Phase 9 — Irregular `yo` and high-frequency irregulars

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 38 | `-go` class: tengo, vengo, pongo, salgo, hago, digo, traigo | `irreg.yo.go-class` | 28 | |
| 39 | `-zco` class: conozco, conduzco, traduzco | `irreg.yo.zco-class` | 28 | |
| 40 | `-oy` class: soy, doy, voy, estoy | `irreg.yo.oy-class` | 28 | |
| 41 | `sé` (saber) | `irreg.yo.se` | 28 | |
| 42 | `Tener que` + inf as habit (already known); now extend to `tener` for possession and tener-idioms (`hambre`, `sed`, `frío`, `calor`, `sueño`, `miedo`) | `lex.tener-idioms` | 38 | |
| 43 | `Saber` vs `conocer` selection | `lex.saber-conocer` | 41 | "I know him / I know that / I know how to swim." |
| 44 | `Decir`, `hacer`, `dar`, `ver` full paradigms | `irreg.full.high-freq` | 28, 38 | |

## Phase 10 — Reflexive verbs

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 45 | Reflexive concept: `me`, `te`, `se`, `nos`, `se` | `reflex.pres.basic` | 28 | llamarse, sentirse, ponerse, levantarse. |
| 46 | Reflexive infinitive placement (`quiero quedarme` / `me quiero quedar`) | `reflex.inf.placement` | 32, 45 | Both forms accepted. |
| 47 | Reciprocal `nos` (`nos vemos`, `nos hablamos`) | `reflex.reciprocal` | 45 | |
| 48 | Reflexive vs non-reflexive same verb: lavar / lavarse, ir / irse, dormir / dormirse, quedar / quedarse | `reflex.contrast` | 45 | Forced choice — drill the meaning shift. |
| 49 | Daily-routine narrative drill | `reflex.routine.narrative` | 45–48 | Multi-sentence connected output. |

## Phase 11 — `Ser`

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 50 | Ser singular: identification, profession, nationality | `ser.singular.identify` | 40 | |
| 51 | Ser plural | `ser.plural` | 50 | |
| 52 | `Ser` + `de` (origin / material / possession) | `ser.de` | 50 | |
| 53 | `Ser` + adjective (characteristic) — agreement enforced | `ser.characteristic` | 51 | |

## Phase 12 — `Estar`

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 54 | Estar singular + location | `estar.singular.location` | 40 | |
| 55 | Estar plural | `estar.plural` | 54 | |
| 56 | Estar + adjective (state) | `estar.state` | 55 | cansado, ocupado, contento, aburrido, listo, bien, mal. |
| 57 | `Hay` (there is / there are) — invariant; contrasted with `está` | `lex.hay-vs-esta` | 54 | |

## Phase 13 — Ser vs Estar selection

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 58 | Forced choice: characteristic vs state | `ser-vs-estar.char-state` | 53, 56 | |
| 59 | Adjectives that shift meaning across ser / estar (listo, bueno, aburrido, vivo, rico, malo) | `ser-vs-estar.shifting` | 58 | |
| 60 | Mixed forced-choice 30-item drill | `ser-vs-estar.mixed` | 58, 59 | |

## Phase 14 — Adjective system

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 61 | Adjective agreement (gender + number) | `adj.agreement` | 53 | |
| 62 | Adjective placement (after noun; the small "before noun" set) | `adj.placement` | 61 | |
| 63 | Demonstratives: este / ese / aquel + plurals | `adj.demonstrative` | 61 | |
| 64 | Possessives: mi / tu / su / nuestro / vuestro | `adj.possessive` | 61 | |
| 65 | Comparatives & superlatives: más / menos / que / como, el más + adj + de | `adj.comparison` | 61 | |
| 66 | Quantifiers: mucho / poco / tanto / demasiado / todo / cada | `adj.quantifier` | 61 | |

## Phase 15 — Continuous (`estar` + `-ndo`)

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 67 | Estar + `-ando` / `-iendo` formation | `cont.formation` | 56 | |
| 68 | Continuous restricted to right-now (vs simple present for near-future) | `cont.scope` | 67 | "I'm preparing it tomorrow" → simple present, not continuous. |
| 69 | Clitic with continuous: `lo estoy haciendo` / `estoy haciéndolo` (accent rule) | `cont.clitic-placement` | 32, 67 | |

## Phase 16 — Present perfect

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 70 | Haber: he, has, ha, hemos, han | `perfect.haber` | 28 | |
| 71 | Regular participle: `-ado` / `-ido` | `perfect.participle.regular` | 70 | |
| 72 | Irregular participles: visto, hecho, dicho, puesto, vuelto, escrito, abierto, roto, muerto | `perfect.participle.irregular` | 71 | |
| 73 | Clitic must precede `haber` (`lo he hecho`; never `he lo hecho`) | `perfect.clitic` | 71 | |
| 74 | Reflexive perfect: `me he perdido`, `nos hemos enamorado` | `perfect.reflex` | 49, 71 | |

## Phase 17 — Preterite

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 75 | Preterite `-ar` regular: `-é, -aste, -ó, -amos, -aron` | `pret.ar.regular` | 24 | |
| 76 | Preterite `-er` / `-ir` regular: `-í, -iste, -ió, -imos, -ieron` | `pret.er-ir.regular` | 28 | |
| 77 | Spelling-change preterite: llegué, busqué, empecé, pagué, toqué | `pret.spelling-change` | 75 | |
| 78 | Strong preterite, t-class: tener (tuve), estar (estuve), poder (pude), poner (puse), saber (supe), hacer (hice), querer (quise), venir (vine) | `pret.strong.t-class` | 76 | |
| 79 | j-stem preterite: decir (dije), traer (traje), traducir (traduje) — note `-eron`, not `-ieron` | `pret.j-stem` | 78 | |
| 80 | `Ir` and `ser` share preterite: fui / fuiste / fue / fuimos / fueron | `pret.ir-ser` | 76 | |
| 81 | Preterite mixed (regular + irregular) | `pret.mixed` | 75–80 | |

## Phase 18 — Imperfect

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 82 | Imperfect `-ar` (`-aba` family) | `imperf.ar` | 24 | |
| 83 | Imperfect `-er` / `-ir` (`-ía` family) | `imperf.er-ir` | 28 | |
| 84 | Three irregulars: era (ser), iba (ir), veía (ver) | `imperf.irregular` | 82, 83 | |

## Phase 19 — Preterite vs Imperfect selection

The hardest aspect distinction in the language; gets a dedicated phase.

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 85 | Aspect contrast: completed event vs background / habit | `pret-vs-imperf.contrast` | 81, 84 | |
| 86 | Discourse pattern: imperfect background + preterite event ("I was eating when he arrived") | `pret-vs-imperf.discourse` | 85 | |
| 87 | Forced-choice 50-item drill | `pret-vs-imperf.mixed` | 85, 86 | |

## Phase 20 — Future

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 88 | Regular future: endings on the infinitive (`-é, -ás, -á, -emos, -án`) | `future.regular` | 28 | |
| 89 | Irregular future stems: tendr-, vendr-, har-, dir-, podr-, querr-, sabr-, pondr-, saldr-, habr- | `future.irregular` | 88 | |
| 90 | Future perfect: `habré` + participle | `future.perfect` | 71, 89 | |

## Phase 21 — Conditional

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 91 | Regular conditional (`-ía` family on the infinitive) | `cond.regular` | 28 | |
| 92 | Irregular conditional stems (same set as future) | `cond.irregular` | 91 | |
| 93 | Politeness uses: `me gustaría`, `podrías`, `deberías`, `querría` | `cond.politeness` | 91 | |
| 94 | Conditional perfect: `habría` + participle (would have) | `cond.perfect` | 71, 91 | |

## Phase 22 — `Gustar` family

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 95 | `Me gusta` / `me gustan` — inverted construction | `gustar.basic` | 30 | |
| 96 | `A mí` / `a ti` emphasis & contrast | `gustar.emphasis` | 95 | |
| 97 | Same pattern verbs: encantar, interesar, parecer, doler, faltar, quedar (suit) | `gustar.family` | 95 | |
| 98 | `Me gustaría` + inf and `me habría gustado` + inf | `gustar.cond-perf` | 93, 94, 95 | |

## Phase 23 — Prepositions

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 99 | `Por` vs `para` | `prep.por-para` | — | |
| 100 | Verbs whose preposition differs from English: pensar **en**, soñar **con**, enamorarse **de**, depender **de**, casarse **con**, contar **con** | `prep.verb-specific` | — | |
| 101 | Prepositional pronouns: mí, ti, él, ella, nosotros, ustedes | `prep.pronouns` | — | |
| 102 | `Conmigo` / `contigo` | `prep.con-go` | 101 | |

## Phase 24 — Commands

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 103 | Tú affirmative regular (drop final `-s`) | `cmd.tu.affirm.regular` | 28 | |
| 104 | Tú affirmative irregulars: di, haz, ven, ten, sé, ve, sal, pon | `cmd.tu.affirm.irregular` | 103 | |
| 105 | Ustedes commands | `cmd.ustedes` | 103 | |
| 106 | Tú negative — uses present subjunctive forms | `cmd.tu.neg` | 105 | |

## Phase 25 — Subjunctive (bridge to Module 2)

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 107 | Present subjunctive forms (regular + key irregulars: sea, esté, vaya, dé, sepa, haya) | `subj.forms` | 106 | |
| 108 | `Quiero que` + subjunctive | `subj.quiero-que` | 107 | "I want him to come / I want you to know." |
| 109 | Impersonal triggers: `es importante que`, `es necesario que`, `es mejor que` | `subj.impersonal` | 107 | |

## Phase 26 — Capstone

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 110 | Big interleaved review — 80 items sampling all 25 phases | `capstone.review` | all | |
| 111 | Translation gauntlet — 50 multi-clause stacked sentences | `capstone.gauntlet` | all | "I would have liked to know if she was going to stay until you arrived, but I didn't see her." |

---

## Exercise generation rules

- **Item shape**: one English source → one canonical Spanish answer + a list of accepted variants (clitic placement options, optional subject pronouns, lexical synonyms).
- **Difficulty curve inside a unit**: items 1–3 are minimum-pair (only the new skill varies), items 4–10 introduce one prior tag, items 11+ stack 2–3 prior tags.
- **Stack ratio**: 30% in early phases, climbing to 60% by Phase 16. Capstone items are 100% stacked.
- **Error response**: a wrong item adds 3 retry items at the same tag, spaced into the next two sessions. If the same micro-skill errs three times across a window, the unit's prerequisite tags are also resampled.
- **Interleave window**: each session pulls 40% current unit, 40% sliding window of last 5 units, 20% long-tail random across mastered tags.

## Open design questions

- **Variant acceptance**: how strict on accents and punctuation? (Recommended: strict on accents that change meaning — `que` vs `qué`, `si` vs `sí` — lenient otherwise, with a "you missed an accent" hint.)
- **Mastery threshold**: 80% over the last 20 items at a given tag? Or a calibrated SRS interval? Both are defensible.
- **Tag granularity**: tag at the unit level only, or also per-item (so an item can carry multiple tags and contribute to multiple skills)? Per-item is more flexible but harder to author.
- **Skip rules**: should a learner be able to test out of a unit by passing a 10-item placement quiz? Useful for returning users.
- **Source-of-truth handoff**: each unit links to a video timestamp + notes anchor. Where is that mapping stored — in this file, or in a separate `unit-source-map.yaml`?
