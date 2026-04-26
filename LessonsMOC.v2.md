# Lessons MOC v2 — Practice App, Drill-Unit Indexed

The app is a **practice-only** companion to a separate video + notes channel. The video carries instruction and pronunciation; the notes supplement; the app does reps. Each unit below is a **drill set** — a tagged exercise bank targeting one micro-skill — not a teaching lesson.

The spine is structural-skill order, not vocabulary-rule order. Cognate rules are not phases here; they appear as transformation patterns inside the translation drills themselves (and live explicitly in the video/notes).

The MOC takes the learner from zero to a solid B2: full subjunctive (forms, triggers, conjunctions, sequence of tenses), `si`-clauses across all three types, passive and impersonal constructions, reported speech, periphrastic verb constructions, the verbs of becoming, accidental `se`, and the deeper preposition / negation / discourse work. Regional dialect forms (`vosotros` for Spain, voseo for the River Plate) are deliberately omitted — the app teaches a neutral standard.

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
subj.<pres|imperf|perfect|pluperfect|trigger|conj|relative>.<aspect>
si-clause.<type>
relative.<pronoun>
report.<aspect>
passive.<type>
periph.<verb>
becoming.<verb>
se-special.<aspect>
neg.<aspect>
discourse.<aspect>
por-para.<use>
compare.<aspect>
morpho.<aspect>
verb-pair.<pair>
num-time.<aspect>
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

## Phase 25 — Subjunctive: forms and core triggers

The subjunctive is a **mood**, not a tense — it surfaces in subordinate clauses when the main clause expresses will, emotion, doubt, or impersonal valuation. This phase teaches the present-subjunctive forms and the four trigger families.

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 107 | Present subjunctive — regular forms (`-ar` → `-e`, `-er`/`-ir` → `-a`) | `subj.pres.regular` | 106 | hablar → hable, comer → coma, vivir → viva — all five persons. |
| 108 | Present subjunctive — yo-derived (irregular `yo` form drives the whole subjunctive paradigm: tener → tenga, hacer → haga, decir → diga, conocer → conozca, salir → salga) | `subj.pres.yo-derived` | 107, 38 | |
| 109 | Present subjunctive — fully irregular: sea, esté, vaya, dé, sepa, haya | `subj.pres.irregular` | 107 | |
| 110 | Will / influence triggers: `querer que`, `pedir que`, `decir que` (request reading), `esperar que`, `exigir que`, `recomendar que`, `aconsejar que` | `subj.trigger.influence` | 109 | |
| 111 | Emotion triggers: `alegrarse de que`, `sentir que`, `temer que`, `gustar que`, `molestar que`, `sorprender que` | `subj.trigger.emotion` | 109 | |
| 112 | Doubt / denial triggers: `dudar que`, `no creer que`, `no pensar que`, `es posible que`, `es imposible que`, `no es verdad que` | `subj.trigger.doubt` | 109 | Note the subjunctive flips off when the trigger is affirmed: `creo que viene` (ind) vs `no creo que venga` (subj). |
| 113 | Impersonal triggers: `es importante que`, `es necesario que`, `es mejor que`, `es raro que`, `ojalá (que)` | `subj.trigger.impersonal` | 109 | |
| 114 | Trigger interleaved | `subj.trigger.mixed` | 110–113 | |

## Phase 26 — Subjunctive after conjunctions

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 115 | Time conjunctions when future-pointing: `cuando`, `en cuanto`, `hasta que`, `antes de que`, `después de que`, `mientras`, `tan pronto como` | `subj.conj.time` | 109 | "When he arrives, we'll eat" → `Cuando llegue, comeremos`. |
| 116 | Time conjunctions: indicative vs subjunctive (already-happened / habitual = ind, future / uncertain = subj) | `subj.conj.time.contrast` | 115 | Forced choice between `Cuando llega` and `Cuando llegue`. |
| 117 | Purpose conjunctions: `para que`, `a fin de que` | `subj.conj.purpose` | 109 | |
| 118 | Condition / exception conjunctions: `a menos que`, `con tal de que`, `sin que`, `en caso de que` | `subj.conj.condition` | 109 | |
| 119 | `Aunque` + indicative vs subjunctive (factual concession vs hypothetical) | `subj.conj.aunque` | 109 | "Even though he's tired" (ind) vs "Even if he were tired" (subj). |
| 120 | Conjunction interleaved | `subj.conj.mixed` | 115–119 | |

## Phase 27 — Imperfect subjunctive

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 121 | Imperfect subjunctive — formation rule: 3rd-person plural preterite minus `-ron`, plus `-ra` family endings (`-ra, -ras, -ra, -´ramos, -ran`) | `subj.imperf.formation` | 81, 109 | hablaron → hablara; comieron → comiera; vivieron → viviera. |
| 122 | Imperfect subjunctive — irregular stems inherited from preterite: tuviera, estuviera, fuera, hiciera, dijera, pudiera, supiera, viniera, quisiera, trajera | `subj.imperf.irregular` | 121, 78, 79 | |
| 123 | `-se` variant (hablase, comiese): recognize as accepted alternate; production focus stays on `-ra` | `subj.imperf.se-variant` | 121 | |
| 124 | Sequence of tenses: past trigger + imperfect subjunctive (`Quería que viniera`, `Le pedí que lo hiciera`, `Era importante que estuvieras`) | `subj.imperf.sequence` | 121 | |
| 125 | After `como si` (as if) — always imperfect or past perfect subjunctive, never indicative | `subj.imperf.como-si` | 121 | "He talks as if he knew" → `Habla como si supiera`. |
| 126 | Polite uses: `quisiera`, `debiera`, `pudiera` (softer than the conditional) | `subj.imperf.politeness` | 121 | |

## Phase 28 — Compound subjunctive forms

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 127 | Present perfect subjunctive: `haya` + participle (`Espero que haya llegado`, `No creo que lo haya hecho`) | `subj.perfect` | 109, 71 | |
| 128 | Past perfect subjunctive: `hubiera` + participle (`Si hubiera sabido`, `Como si hubiera visto un fantasma`) | `subj.pluperfect` | 121, 71 | |

## Phase 29 — `Si`-clauses (the conditional construction)

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 129 | Type 1 — real / likely: `Si` + present indicative, present or future indicative | `si-clause.real` | 88 | "If I have time, I'll go" → `Si tengo tiempo, voy / iré`. |
| 130 | Type 2 — hypothetical: `Si` + imperfect subjunctive, conditional | `si-clause.hypothetical` | 92, 121 | "If I had time, I'd go" → `Si tuviera tiempo, iría`. |
| 131 | Type 3 — counterfactual past: `Si` + past perfect subjunctive, conditional perfect | `si-clause.counterfactual` | 94, 128 | "If I had known, I would have come" → `Si hubiera sabido, habría venido`. |
| 132 | Mixed type — counterfactual past with present consequence (`Si hubiera estudiado, ahora hablaría español`) | `si-clause.mixed` | 130, 131 | |
| 133 | Si-clause forced-choice 40-item drill | `si-clause.selection` | 129–132 | The hardest selection task in Spanish grammar — needs heavy reps. |

## Phase 30 — Relative clauses

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 134 | `que` (subject or object, people or things) — the workhorse relative | `relative.que` | — | |
| 135 | `quien` / `quienes` — people, often after a preposition (`la persona con quien hablo`) | `relative.quien` | 134 | |
| 136 | `el que` / `la que` / `los que` / `las que` / `lo que` — disambiguating; `lo que` for "what / the thing that" | `relative.el-que` | 134 | |
| 137 | `el cual` / `la cual` / `los cuales` / `las cuales` — formal, after long prepositions (`detrás del cual`, `delante de la cual`) | `relative.el-cual` | 136 | |
| 138 | `cuyo` / `cuya` / `cuyos` / `cuyas` — possessive "whose"; agrees with the thing possessed | `relative.cuyo` | 134 | |
| 139 | Relative adverbs: `donde`, `cuando`, `como` | `relative.adverb` | 134 | |
| 140 | Restrictive vs non-restrictive (the comma changes the meaning) | `relative.restrictive` | 134 | "Mis hermanos que viven en Madrid" (only those who) vs "Mis hermanos, que viven en Madrid" (all of them, by the way). |
| 141 | Subjunctive in relative clauses: existing antecedent (ind) vs non-existing / uncertain antecedent (subj) | `subj.relative` | 109, 134 | "Busco a la persona que sabe…" vs "Busco a alguien que sepa…". |

## Phase 31 — Reported speech

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 142 | Reported statements: tense backshift (present → imperfect, preterite/perfect → past perfect, future → conditional) | `report.statement` | 121, 91 | "Dice que viene" → "Dijo que venía / había venido / vendría." |
| 143 | Reported questions: `si` for yes/no, wh-words for content; no inversion | `report.question` | 142 | "Me preguntó si quería ir / qué quería." |
| 144 | Reported commands: `decir / pedir / ordenar` + `que` + subjunctive (present subj after present trigger; imperfect subj after past trigger) | `report.command` | 109, 121 | "Me dijo que viniera." |
| 145 | Time and place adjustments (`hoy → ese día`, `mañana → al día siguiente`, `aquí → allí`, `este → ese`) | `report.deixis` | 142 | |

## Phase 32 — Passive and impersonal constructions

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 146 | True passive: `ser` + participle (+ optional `por` agent) — used less in Spanish than English | `passive.ser` | 71 | "The book was written by her" → `El libro fue escrito por ella`. |
| 147 | Passive `se` (the most common passive in Spanish): `Se venden libros`, `Se cerró la puerta`, `Se construyeron las casas` | `passive.se` | 28 | Verb agrees with the grammatical subject. |
| 148 | Impersonal `se`: `Se dice que`, `Se come bien aquí`, `Se vive bien` — verb stays singular; no specific subject | `passive.se.impersonal` | 28 | |
| 149 | Selection: ser-passive vs passive-se vs impersonal-se | `passive.selection` | 146–148 | |

## Phase 33 — Periphrastic verb constructions

The big set of "auxiliary + verb" patterns that English handles with adverbs ("just," "again," "usually," "still") or full clauses ("I've been doing X for two years").

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 150 | `Acabar de` + inf — "just did X" | `periph.acabar-de` | 28 | `Acabo de comer` = I just ate. |
| 151 | `Soler` + inf — "usually X / used to X" (stem-changing: suelo / sueles / suele) | `periph.soler` | 33 | |
| 152 | `Llevar` + tiempo + gerund — "have been Xing for [time]" | `periph.llevar-gerund` | 67 | `Llevo tres años estudiando` = I've been studying for three years. |
| 153 | `Volver a` + inf — "do X again" | `periph.volver-a` | 28 | |
| 154 | `Seguir` / `continuar` + gerund — "continue Xing / still X" | `periph.seguir-gerund` | 67 | |
| 155 | `Dejar de` + inf — "stop Xing" | `periph.dejar-de` | 28 | |
| 156 | `Empezar a` / `comenzar a` / `ponerse a` + inf — "start Xing" (the last is more sudden) | `periph.start` | 28 | |
| 157 | `Estar a punto de` + inf — "be about to X" | `periph.estar-a-punto-de` | 56 | |
| 158 | `Tratar de` / `intentar` + inf — "try to X" | `periph.try` | 28 | |
| 159 | Periphrasis interleaved | `periph.mixed` | 150–158 | |

## Phase 34 — Verbs of becoming

English `become` covers what Spanish splits across six verbs by mechanism and permanence.

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 160 | `Ponerse` + adj — temporary state, often emotional (`Se puso triste`, `Se puso rojo`) | `becoming.ponerse` | 45 | |
| 161 | `Volverse` + adj/noun — sudden character change, often involuntary (`Se volvió loco`) | `becoming.volverse` | 45 | |
| 162 | `Hacerse` + noun/adj — gradual change, often by effort (profession, religion: `Se hizo médico`, `Se hizo rico`) | `becoming.hacerse` | 45 | |
| 163 | `Llegar a ser` + noun — achievement after a process (`Llegó a ser presidente`) | `becoming.llegar-a-ser` | 28 | |
| 164 | `Convertirse en` + noun — transformation (`Se convirtió en una estrella`) | `becoming.convertirse` | 45 | |
| 165 | `Quedarse` + adj — resulting state, often after loss or change (`Se quedó solo`, `Se quedó dormido`) | `becoming.quedarse` | 45 | |
| 166 | Becoming-verb selection drill | `becoming.selection` | 160–165 | Forced choice across the six: which becoming verb fits this English sentence? |

## Phase 35 — Special `se` constructions

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 167 | Accidental / unplanned `se`: `Se me cayó`, `Se me olvidó`, `Se me rompió`, `Se le ocurrió` (the speaker disclaims agency) | `se-special.accidental` | 45 | "I dropped it" → `Se me cayó` (lit. it fell from me). |
| 168 | Aspectual `se`: `comerse`, `beberse`, `dormirse`, `irse` — adds completion or intensity (`Me comí toda la pizza`) | `se-special.aspectual` | 45 | |

## Phase 36 — Negation refinements

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 169 | Double negation: `no… nunca`, `no… nadie`, `no… nada`, `no… ningún/ninguna` — pre-verb position drops the `no` (`Nunca como carne` vs `No como carne nunca`) | `neg.double` | — | |
| 170 | `ni… ni` — neither… nor | `neg.ni-ni` | — | |
| 171 | Contrast pairs: `tampoco` vs `también`, `ya no` vs `todavía no` | `neg.contrast-pairs` | — | |

## Phase 37 — Connectives and discourse markers

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 172 | `Pero` vs `sino` vs `sino que` (`sino` after a negation contrasting two nouns/adjs; `sino que` before a clause) | `discourse.pero-sino` | — | "No es alto sino bajo" vs "No estudia sino que trabaja." |
| 173 | `Aunque` deeper dive — concession in extended discourse | `discourse.aunque` | 119 | |
| 174 | Markers: `sin embargo`, `en cambio`, `además`, `por lo tanto`, `así que`, `pues`, `mientras que` | `discourse.markers` | — | Productive use across multi-sentence outputs. |

## Phase 38 — `Por` vs `Para` (deep)

`Por` and `para` each have ~6 distinct uses; the single unit in Phase 23 was a placeholder.

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 175 | `Por`: cause / reason ("because of"), exchange ("for, in exchange for"), substitution ("on behalf of") | `por-para.por.cause-exchange` | 99 | |
| 176 | `Por`: duration, time of day, motion through, agent in passive | `por-para.por.time-motion` | 99 | |
| 177 | `Para`: destination, deadline, recipient | `por-para.para.dest-deadline` | 99 | |
| 178 | `Para`: purpose ("in order to"), opinion ("for me, …"), comparison ("for a beginner, …") | `por-para.para.purpose-opinion` | 99 | |
| 179 | `Por` vs `para` forced-choice drill — minimum-pair items | `por-para.selection` | 175–178 | |

## Phase 39 — Comparatives, superlatives, diminutives

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 180 | Irregular comparatives: `mejor`, `peor`, `mayor`, `menor` | `compare.irregular` | 65 | |
| 181 | Equality: `tan + adj/adv + como`, `tanto/a/os/as + noun + como`, `tanto como` (with verbs) | `compare.equality` | 65 | |
| 182 | Absolute superlative `-ísimo` and its spelling shifts (`rico → riquísimo`, `amable → amabilísimo`, `blanco → blanquísimo`, `feliz → felicísimo`) | `compare.absolute` | 61 | |
| 183 | Diminutives `-ito` / `-ita` and meaning shifts (size, affection, attenuation: `un momentito`, `un cafecito`, `ahorita`) | `morpho.diminutive` | — | |

## Phase 40 — Verb-pair selection (advanced)

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 184 | `Pedir` (request something) vs `preguntar` (ask a question) | `verb-pair.pedir-preguntar` | 28 | |
| 185 | `Llevar` (take, bring along, wear) vs `traer` (bring here) | `verb-pair.llevar-traer` | 28 | Direction relative to the speaker is the cue. |
| 186 | `Ir` vs `irse` (the `-se` adds emphasis on departure); `salir` vs `irse` | `verb-pair.ir-irse-salir` | 48 | |
| 187 | Ser vs estar — advanced cases: `estar muerto`, `ser/estar feliz`, `ser/estar joven`, `ser/estar guapo`, `ser/estar listo` | `verb-pair.ser-estar.advanced` | 60 | |
| 188 | `Saber` vs `conocer` — review with new domains (skills, recipes, places, languages, people) | `verb-pair.saber-conocer.review` | 43 | |

## Phase 41 — Numbers, time, dates

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 189 | Ordinals: `primero/primer`, `segundo`, `tercero/tercer`, `cuarto`, …, `décimo`; apocope before masculine singular nouns (`el primer día`) | `num-time.ordinal` | — | |
| 190 | Fractions and percentages: `un tercio`, `dos tercios`, `el 25 por ciento`, `la mitad`, `el doble` | `num-time.fraction` | — | |
| 191 | Date format and prepositions: `el lunes`, `en marzo`, `en 2024`, `hace dos días`, `dentro de una semana`, `a los veinte años` | `num-time.date` | — | |
| 192 | Big numbers: `mil`, `un millón`, `mil millones`; reading aloud (`mil novecientos ochenta y cuatro`) | `num-time.big` | — | |

## Phase 42 — Capstone B2

| # | Unit | Skill tag | Prereqs | Drill |
|---|---|---|---|---|
| 193 | Big interleaved review — 120 items sampling all 41 phases, weighted toward each learner's tag-level error history | `capstone.review` | all | |
| 194 | Multi-clause translation gauntlet — 60 B2 sentences combining subjunctive, si-clauses, reported speech, passive, periphrasis, becoming verbs | `capstone.gauntlet` | all | "If I had known they were saying he had become a doctor, I would have called him a long time ago." |
| 195 | Sustained narrative production — write a short story using all four past forms (preterite, imperfect, present perfect, past perfect) and at least one passage of reported speech | `capstone.narrative` | all | |

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
