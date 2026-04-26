# Reference Notes v2

This file is the cleaned teacher-reference version of [ReferenceNotes.md](/Users/arpan/projects/spanish-app-v1/ReferenceNotes.md).

Its purpose is not to be a lesson script. Its purpose is to be a reliable source for later lesson specs, exercise generation, error analysis, and remediation.

## How to use this file

- Use `Canonical` items as safe generation material.
- Use `Heuristic` items as memory hooks or explanation aids, not as hard rules.
- Use `Warning` items to prevent predictable learner mistakes.
- Use the sequencing notes to decide what can appear in beginner lessons and what should wait.
- Do not generate exercises directly from the raw notes when a point is marked `Needs review`.

## Status Labels

- `Canonical`: stable rule or pattern suitable for exercises.
- `Heuristic`: useful shortcut that helps beginners notice patterns, but has important exceptions.
- `Warning`: a common mistake, overgeneralization, or wording trap to avoid.
- `Needs review`: useful idea, but not yet precise enough for generation.

## Generation Guardrails

- Keep examples within already-taught grammar.
- Prefer short, high-frequency verbs and concrete nouns.
- Do not introduce a new structure and a new irregular verb in the same first exposure unless the lesson is explicitly about that irregularity.
- Keep explanation language simple and pattern-based.
- For deliberate practice, track the exact error type, not just whether the whole sentence was wrong.

---

## 1. Core Scope and Sequence

The raw notes cover much more than the first stage of a beginner course. For exercise generation, split the material into these layers:

### Layer A: Core beginner canon

- Pronunciation and stress basics
- Cognate awareness with tight limits
- Infinitives and common helper verbs
- Present tense regular verbs
- Very common irregular present forms
- Direct objects, indirect objects, and reflexive pronouns
- Gender, number, articles, and basic adjectives
- `ser` vs `estar`
- Present with future context
- `voy a + infinitive`
- Present progressive
- Present perfect
- Basic prepositions and personal `a`
- `gustar`-type structures
- Preterite and imperfect at a basic level

### Layer B: Upper-beginner extensions

- Future and conditional
- Perfect conditional and other compound forms
- More preposition shifts
- Relative words such as `lo que`, `quien`, `cual`
- Commands
- Introductory present subjunctive

### Layer C: Keep out of early generation

- Dense chains with multiple pronouns unless the lesson is explicitly about them
- Fine semantic differences between near-synonyms
- Broad etymology claims with many exceptions
- Low-frequency or stylistically marked constructions

---

## 2. Orthography and Pronunciation

### RNV2-PHON-01: Silent `h`
Status: `Canonical`

- The letter `h` is silent in modern Spanish.
- Examples:
  - `hablar`
  - `horrible`
  - `he hablado`

Warning:
- Do not teach students to ignore spelling because `h` is silent. They still need to write it correctly.

### RNV2-PHON-02: Default stress
Status: `Canonical`

- If a word ends in a vowel, `n`, or `s`, stress usually falls on the penultimate syllable.
- Otherwise, stress usually falls on the final syllable.
- Written accents show that the word breaks the default stress rule, or they distinguish words in writing.
- Examples:
  - `habla`, `hablan`, `importante`
  - `hablar`, `comer`
  - `tradición` is a good example of a word whose written accent must be preserved

Warning:
- The raw notes mix correct stress logic with many missing written accents. Exercise generation must use standard orthography.

### RNV2-PHON-03: Accent marks matter
Status: `Canonical`

- Accent marks are part of correct spelling.
- They can mark stress or distinguish meanings.
- High-value contrasts:
  - `si` = if
  - `sí` = yes
  - `tu` = your, `tú` = you
  - `mi` = my, `mí` = me
  - question words such as `cómo`, `dónde`, `qué`, `cuándo`, `quién`, `cuál` need written accents when used in direct or indirect questions

Warning:
- The raw notes often omit accents in examples. Any generated Spanish should be normalized before use.

### RNV2-PHON-04: Sound approximations
Status: `Heuristic`

- Some spelling-to-sound memory hooks are useful early on, but keep them light.
- Example:
  - `j` often has a breathy sound unlike English `j`
  - `rr` is stronger than single `r`

Needs review:
- Avoid overly narrow sound mnemonics unless a pronunciation module is being built.

---

## 3. Cognates and Word-Building

These patterns are powerful for beginners, but they are pattern-recognition tools, not guarantees.

### RNV2-COG-01: `-tion` -> `-ción`
Status: `Heuristic`

- Many English words ending in `-tion` correspond to Spanish words ending in `-ción`.
- Examples:
  - `conversation` -> `conversación`
  - `generation` -> `generación`
  - `confirmation` -> `confirmación`

Teaching note:
- This is useful for recognition and vocabulary expansion.

Warning:
- Do not tell students to convert every `-tion` word mechanically.
- Standard written Spanish usually requires the accented form `-ción`.

### RNV2-COG-02: Noun to verb via `-ar`
Status: `Heuristic`

- Some learned vocabulary lets students notice a related verb in `-ar`.
- Examples:
  - `preparación` -> `preparar`
  - `confirmación` -> `confirmar`
  - `exploración` -> `explorar`

Warning:
- This is a discovery strategy, not a productive spelling rule.

### RNV2-COG-03: `-mente` adverbs
Status: `Canonical`

- Many adverbs in `-mente` correspond to English adverbs in `-ly`.
- Examples:
  - `realmente`
  - `constantemente`
  - `posiblemente`

### RNV2-COG-04: `-dad`, `-idad`, `-edad`
Status: `Heuristic`

- Many English abstract nouns in `-ity` map to Spanish nouns in `-dad`, `-idad`, or `-edad`.
- Examples:
  - `clarity` -> `claridad`
  - `nationality` -> `nacionalidad`
  - `possibility` -> `posibilidad`

Warning:
- Do not collapse all `-ity` words into a single transformation rule.

### RNV2-COG-05: `-ivo`, `-ante`, `-ente`
Status: `Heuristic`

- Some adjective patterns are transparent and worth teaching as recognition aids.
- Examples:
  - `creative` -> `creativo`
  - `intensive` -> `intensivo`
  - `important` -> `importante`
  - `different` -> `diferente`

Warning:
- Many individual spellings and stress patterns still need explicit checking.

### RNV2-COG-06: Common overreach to avoid
Status: `Warning`

- Students will over-trust cognates and create non-words.
- Exercise generation should explicitly target:
  - false or partial cognates
  - missing accents
  - English spelling transfer
  - over-generated forms such as direct English-to-Spanish conversions

---

## 4. Verb Basics and Infinitive Chains

### RNV2-VERB-01: Infinitives
Status: `Canonical`

- Spanish dictionary forms usually end in `-ar`, `-er`, or `-ir`.
- Examples:
  - `hablar`
  - `comer`
  - `vivir`

### RNV2-VERB-02: High-frequency helper verbs
Status: `Canonical`

- Very early lessons can build a lot of output from a small helper set:
  - `quiero + infinitive`
  - `voy a + infinitive`
  - `intento + infinitive`
  - `debo + infinitive`
  - `tengo que + infinitive`
  - `puedo + infinitive`

Examples:
- `Quiero comer.`
- `Voy a visitar la casa.`
- `Intento verlo.`
- `Debo continuar.`
- `Tengo que salir.`
- `Puedo venir.`

### RNV2-VERB-03: Motion verbs often link with `a`
Status: `Canonical`

- Verbs like `ir`, `venir`, `pasar`, `salir` commonly connect to another infinitive with `a`.
- Examples:
  - `Voy a comer.`
  - `Viene a visitarme.`
  - `Paso a verte.`
  - `Sale a bailar.`

Warning:
- Do not generalize `a` after every first verb in a chain.

### RNV2-VERB-04: Present can express scheduled or intended future
Status: `Canonical`

- Spanish often uses the present tense with future time markers.
- Examples:
  - `La veo más tarde.`
  - `Salgo mañana.`
  - `Hablamos la semana que viene.`

Teaching note:
- This should be taught early because it keeps beginner output natural without early tense overload.

---

## 5. Pronouns: Direct, Indirect, and Reflexive

### RNV2-PRON-01: Direct object pronouns
Status: `Canonical`

- Early core set:
  - `lo`, `la`, `los`, `las`
- Use them for direct objects already understood from context.
- Examples:
  - `Lo veo.`
  - `La quiero ver.`
  - `Los compro.`

### RNV2-PRON-02: Indirect object pronouns
Status: `Canonical`

- Early core set:
  - `me`, `te`, `le`, `nos`, `les`
- They often replace `to` in English, but not in a one-to-one way.
- Examples:
  - `Me habla.`
  - `Te compro algo.`
  - `Le doy el libro.`
  - `Nos venden algo.`

Teaching note:
- Beginners should learn function before terminology.

### RNV2-PRON-03: Indirect before direct
Status: `Canonical`

- When an indirect and a direct object pronoun appear together, the indirect one comes first.
- Examples:
  - `Me lo venden.`
  - `Te lo doy.`
  - `Nos la compran.`

### RNV2-PRON-04: `le` / `les` -> `se` before `lo/la/los/las`
Status: `Canonical`

- Spanish avoids sequences like `le lo`.
- `le` or `les` becomes `se` before direct object pronouns.
- Examples:
  - `Se lo di.`
  - `Se las vendí.`

### RNV2-PRON-05: Pronoun placement
Status: `Canonical`

- With a conjugated verb alone, object pronouns usually go before the conjugated verb.
  - `Lo veo.`
  - `Me llama.`
- With an infinitive, a pronoun can attach to the infinitive or go before the conjugated verb.
  - `Quiero verlo.`
  - `Lo quiero ver.`
- With a gerund, a pronoun can attach to the gerund or go before the conjugated verb.
  - `Estoy preparándolo.`
  - `Lo estoy preparando.`
- With affirmative commands, the pronoun attaches.
  - `Cómpralo.`
  - `Espérenme.`
- With negative commands, the pronoun goes before the verb.
  - `No lo compres.`
  - `No me esperen.`

Warning:
- The raw notes frequently separate attached forms with spaces, such as `ver lo` or `quedar me`. Those forms should not be used in generated exercises.

### RNV2-PRON-06: Reflexive forms
Status: `Canonical`

- Many useful beginner verbs appear in reflexive form:
- `quedarse`
- `llamarse`
- `darse cuenta`
- `perderse`
- `dormirse`

Examples:
- `Me quedo aquí.`
- `Te llamas Ana.`
- `Se da cuenta.`
- `Nos perdimos.`
- `Me duermo.`

### RNV2-PRON-07: Pronouns carry meaning, not literal English prepositions
Status: `Canonical`

- `me`, `te`, `le`, `nos`, `les` often encode meanings that English expresses with a preposition.
- Examples:
  - `Me habla.` = speaks to me
  - `Me compra algo.` = buys something for me
  - `Le respondí.` = responded to her / him / you formal

Teaching note:
- Exercise prompts should target interpretation, not word-for-word translation.

---

## 6. Articles, Gender, Number, and Adjectives

### RNV2-NOUN-01: Basic gender patterns
Status: `Canonical`

- Many nouns ending in `-a` are feminine.
- Many nouns ending in `-o` are masculine.
- Examples:
  - `la casa`
  - `el carro`

Warning:
- These are patterns, not absolute rules.

### RNV2-NOUN-02: High-value exceptions
Status: `Canonical`

- Some common nouns ending in `-ma` are masculine.
  - `el problema`
  - `el sistema`
  - `el programa`
- Some common nouns ending in `-o` are feminine.
  - `la mano`
  - `la foto`
  - `la moto`

### RNV2-NOUN-03: Many `-cion` nouns are feminine
Status: `Canonical`

- Examples:
  - `la conversación`
  - `la situación`
  - `la generación`

### RNV2-NOUN-04: Adjectives usually follow nouns
Status: `Canonical`

- Examples:
  - `una cámara digital`
  - `la situación global`
  - `las casas grandes`

### RNV2-NOUN-05: Adjective agreement
Status: `Canonical`

- Adjectives agree with the noun in gender and number.
- Examples:
  - `el carro rojo`
  - `la casa roja`
  - `los carros rojos`
  - `las casas rojas`

Warning:
- Early generation should avoid adjective stacks unless agreement is the target.

---

## 7. Present Tense Foundations

### RNV2-PRES-01: Regular present tense
Status: `Canonical`

- Use `hablar`, `comer`, and `vivir` as anchor verbs.
- High-priority forms for early production:
  - `yo`
  - `tú`
  - `él/ella/usted`
  - `nosotros`
  - `ellos/ellas/ustedes`

Teaching note:
- This is a better anchor than teaching a long list of unrelated verbs.

### RNV2-PRES-02: Very common irregular first-person forms
Status: `Canonical`

- Examples:
  - `tengo`
  - `vengo`
  - `hago`
  - `salgo`
  - `pongo`
  - `doy`
  - `voy`
  - `soy`
  - `se`
  - `veo`

Teaching note:
- These should be introduced as high-frequency exceptions, not as one rule.

### RNV2-PRES-03: Stem-changing verbs
Status: `Canonical`

- Common beginner patterns:
  - `e -> ie`: `pensar`, `entender`, `preferir`, `querer`
  - `o -> ue`: `poder`, `dormir`, `volver`, `encontrar`
  - some `e -> i`: `pedir`, `elegir`, `seguir`
- `nosotros` often keeps the original stem in the present.
- Examples:
  - `pienso` / `pensamos`
  - `puedo` / `podemos`
  - `prefiero` / `preferimos`

Warning:
- Do not teach all stem changes together in one first exposure.

### RNV2-PRES-04: Present as ongoing vs general
Status: `Canonical`

- Spanish present tense can cover:
  - habits
  - general truths
  - near future with time context
- Examples:
  - `Como con Pablo mañana.`
  - `Vivo aquí.`
  - `No tomo alcohol.`

---

## 8. `ser` and `estar`

### RNV2-BE-01: Core contrast
Status: `Canonical`

- `ser` often introduces identity, classification, or relatively stable description.
- `estar` often introduces state, condition, or location.
- Examples:
  - `Soy estudiante.`
  - `Es posible.`
  - `Estoy cansado.`
  - `Estamos aquí.`

### RNV2-BE-02: States vs traits with the same adjective
Status: `Canonical`

- Some adjectives change interpretation with `ser` vs `estar`.
- Examples:
  - `Es aburrido.` = boring
  - `Esta aburrido.` = bored
  - `Es libre.` = free by nature / not constrained
  - `Esta libre.` = available / free at the moment
  - `Es listo.` = clever
  - `Esta listo.` = ready

### RNV2-BE-03: Participles used as adjectives usually go with `estar`
Status: `Canonical`

- Examples:
  - `Esta cerrado.`
  - `Esta abierto.`
  - `Esta roto.`
  - `Esta perdido.`

Warning:
- Do not generalize this to all adjective choices without context.

### RNV2-BE-04: Dead and similar states
Status: `Canonical`

- Some meanings that English learners may expect with `be` still use `estar`.
- Example:
  - `Esta muerto.`

---

## 9. Progressive, Perfect, Future, and Conditional

### RNV2-ASP-01: Present progressive
Status: `Canonical`

- Form:
  - `estar + gerund`
- Examples:
  - `Estoy hablando.`
  - `Estamos desayunando.`
  - `Lo estoy preparando.`

Teaching note:
- Use this for actions happening right now, not as the default future.

### RNV2-ASP-02: Gerund formation
Status: `Canonical`

- `-ar` -> `-ando`
- `-er`, `-ir` -> `-iendo`
- Examples:
  - `hablando`
  - `comiendo`
  - `viviendo`

### RNV2-ASP-03: Present perfect
Status: `Canonical`

- Form:
  - `haber + past participle`
- Examples:
  - `He hablado.`
  - `Hemos comido.`
  - `Ha venido.`
  - `Han perdido.`

Warning:
- After forms of `haber`, use the participle. The raw notes sometimes mix infinitives there.

### RNV2-ASP-04: Common past participles
Status: `Canonical`

- Regular:
  - `hablado`
  - `comido`
  - `vivido`
- High-value irregulars:
  - `roto`
  - `abierto`
  - `muerto`
  - `dicho`
  - `hecho`

### RNV2-ASP-05: Simple future
Status: `Canonical`

- Form:
  - infinitive + future endings
- Examples:
  - `comeré`
  - `iremos`
  - `lo encontrará`
  - `me quedaré`

Teaching note:
- Teach it, but note that beginner speech will often prefer present with future context or `voy a`.

### RNV2-ASP-06: Conditional
Status: `Canonical`

- Form:
  - infinitive + conditional endings
- Examples:
  - `hablaría`
  - `comería`
  - `iría`
  - `me gustaría`
  - `podría`

Teaching note:
- `me gustaría` and `podría` are especially high value.

### RNV2-ASP-07: Compound conditional and modal chains
Status: `Upper-beginner extension`

- Examples:
  - `Habría ido.`
  - `Podría haber ido.`
  - `Debería haberlo hecho.`

Warning:
- Keep these out of early mixed-exercise generation.

---

## 10. Past Tenses

### RNV2-PAST-01: Imperfect as line/background/habit
Status: `Canonical`

- Use the imperfect for:
  - habitual past actions
  - background description
  - ongoing past context
- Examples:
  - `Hablaba con María.`
  - `Cocinaba cuando llegó María.`
  - `Tenía una casa grande.`
  - `Iba a llamarte.`

### RNV2-PAST-02: Preterite as completed event
Status: `Canonical`

- Use the preterite for bounded completed events.
- Examples:
  - `Hablé.`
  - `Comí.`
  - `Llegó.`
  - `Lo vi.`
  - `Nos perdimos.`

### RNV2-PAST-03: High-value contrast
Status: `Canonical`

- Imperfect sets the scene; preterite marks the event.
- Example:
  - `Cocinaba cuando sonó el teléfono.`

Teaching note:
- This contrast should be practiced with timeline meaning, not just endings.

### RNV2-PAST-04: Preterite anchors
Status: `Canonical`

- Use `hablar` and `comer` as hook verbs.
- Examples:
  - `hablé`, `habló`, `hablamos`, `hablaste`, `hablaron`
  - `comí`, `comió`, `comimos`, `comiste`, `comieron`

### RNV2-PAST-05: High-value irregular preterites
Status: `Upper-beginner extension`

- Examples:
  - `di`, `dio`
  - `vi`, `vio`
  - `fui`, `fue`

Warning:
- Introduce only when lesson scope supports them.

---

## 11. `gustar` and Similar Patterns

### RNV2-GUST-01: `gustar`
Status: `Canonical`

- Best beginner explanation:
  - the thing is pleasing to the person
- Examples:
  - `Me gusta.`
  - `Nos gusta.`
  - `Le gustan.`

Warning:
- Do not translate it as a normal subject-verb-object pattern in early instruction.

### RNV2-GUST-02: `interesar` and `parecer`
Status: `Canonical`

- These work well after `gustar`.
- Examples:
  - `Me interesa.`
  - `Le interesa bailar.`
  - `Me parece bien.`
  - `No me parece bien.`

### RNV2-GUST-03: Conditional politeness
Status: `Canonical`

- `Me gustaría + infinitive` is a high-value polite structure.
- Examples:
  - `Me gustaría comer ahora.`
  - `Me gustaría verlo.`

---

## 12. Prepositions and Their Non-English Mappings

### RNV2-PREP-01: Do not force English prepositions onto Spanish
Status: `Canonical`

- Many Spanish verbs encode meanings that English expresses with extra prepositions.
- Examples:
  - `Me habla.`
  - `Le respondí.`
  - `Me duele la pierna.`

### RNV2-PREP-02: High-value verb + preposition combinations
Status: `Canonical`

- `pensar en`
- `soñar con`
- `depender de`
- `enamorarse de`
- `contar con`

Examples:
- `Pienso en ti.`
- `Sueño contigo.`
- `Depende de ella.`
- `Me enamoro de ti.`

### RNV2-PREP-03: `por` vs `para`
Status: `Canonical`

- High-value beginner meanings:
  - `para` = for, intended for, in order to
  - `por` = because of, through, during, in exchange for, ago
- Examples:
  - `Esto es para ti.`
  - `Estoy aquí para verte.`
  - `Lo hice por ti.`
  - `Hace dos semanas.`
  - `Me quedo por tres días.`

### RNV2-PREP-04: No stranded prepositions
Status: `Canonical`

- Spanish does not leave prepositions hanging at the end in the way English often does.
- Examples:
  - `La chica con la que trabajo.`
  - `No sé con quién tengo que hacerlo.`

### RNV2-PREP-05: Personal `a`
Status: `Canonical`

- Use personal `a` with specific people as direct objects.
- Examples:
  - `Veo a María.`
  - `Quiero encontrar al fontanero.`
  - `Espero que vean a María.`

Warning:
- Do not overuse it with things or with indefinite non-specific nouns in beginner drills.

---

## 13. Demonstratives, Possessives, and Common Function Words

### RNV2-FUNC-01: Demonstratives
Status: `Canonical`

- Core set for beginner drills:
  - `este`, `esta`, `estos`, `estas`
  - `ese`, `esa`, `esos`, `esas`
  - neutral forms for "this/that" as ideas: `esto`, `eso`

Examples:
- `este hombre`
- `esa noche`
- `No quiero eso.`

### RNV2-FUNC-02: Possessives
Status: `Canonical`

- High-value forms:
  - `mi`, `mis`
  - `tu`, `tus`
  - `su`, `sus`
  - `nuestro`, `nuestra`, `nuestros`, `nuestras`

Examples:
- `mi casa`
- `sus casas`
- `nuestra casa`

### RNV2-FUNC-03: Common connectors and question words
Status: `Canonical`

- `que / qué`
- `si / sí`
- `por qué`
- `cuando / cuándo`
- `donde / dónde`
- `como / cómo`
- `quien / quién`
- `cual / cuál`
- `lo que`

Teaching note:
- Question-word accent rules should be reinforced separately in final lesson specs.

---

## 14. Commands and Introductory Subjunctive

These are useful, but they should sit after the stable beginner foundation.

### RNV2-SUBJ-01: Triggered present subjunctive
Status: `Upper-beginner extension`

- High-value triggers from the notes:
  - `quiero que`
  - `prefiero que`
  - `espero que`
  - `hace falta que`
  - `para que`

Examples:
- `Quiero que esperes.`
- `Prefiero que se quede.`
- `Espero que cuentes conmigo.`

### RNV2-SUBJ-02: Future reference after `cuando`
Status: `Upper-beginner extension`

- When `cuando` refers to a future event, Spanish often uses the subjunctive.
- Example:
  - `Cuando hablemos, te digo.`

### RNV2-SUBJ-03: Negative commands
Status: `Upper-beginner extension`

- Negative commands align well with subjunctive teaching.
- Examples:
  - `No hables.`
  - `No te quedes aquí.`
  - `No lo des.`

### RNV2-SUBJ-04: Affirmative informal commands
Status: `Upper-beginner extension`

- High-frequency rule:
  - often related to the `tú` present form without final `s`
- Examples:
  - `compra`
  - `come`
  - `vende`
- High-value irregular:
  - `haz`

Warning:
- Keep command generation narrowly scoped so pronoun placement does not become overloaded.

---

## 15. Common Beginner Error Taxonomy

This section exists so future exercises can target exact weaknesses.

### ERR-01: Orthography

- missing accent mark
- wrong accent placement
- English spelling transferred into Spanish
- omitted `h`
- missing `n` or `s` endings

### ERR-02: Cognate overreach

- invented Spanish form from English
- wrong suffix transformation
- false friend usage

### ERR-03: Verb morphology

- wrong infinitive ending
- wrong present ending
- wrong stem-change form
- wrong tense choice
- wrong participle after `haber`

### ERR-04: Pronoun selection

- wrong direct object pronoun
- wrong indirect object pronoun
- failure to change `le/les` to `se`
- reflexive vs non-reflexive confusion

### ERR-05: Pronoun placement

- split attached form such as `ver lo`
- pronoun after negative command
- pronoun before affirmative command when attachment is required

### ERR-06: Syntax and connectors

- missing `a` in `voy a`
- missing `que` after trigger phrases
- missing personal `a`
- English word order transfer

### ERR-07: `ser` vs `estar`

- using `ser` for a state
- using `estar` for identity/classification
- misreading adjectives that change meaning across the pair

### ERR-08: Prepositions

- using English preposition instead of Spanish pattern
- omitting the verb's required preposition
- stranding a preposition

---

## 16. Review Queue: Useful but Not Safe Yet

These ideas appear often enough in the raw notes to be worth keeping, but they are not ready to drive exercise generation without tighter wording.

### RQ-01: Very broad etymology rules
Status: `Needs review`

- Examples:
  - "`-al` words come from Latin so we can convert them"
  - "`-ant` or `-ent` just add `-e`"
  - "`-ary` becomes `-ario`"

Problem:
- Too many exceptions and spelling traps.

### RQ-02: Mechanical noun-to-verb conversion
Status: `Needs review`

- Examples:
  - "remove `-tion` and add `-ar`"

Problem:
- Sometimes helpful for teacher intuition, but unsafe as a production rule.

### RQ-03: Over-literal English glosses
Status: `Needs review`

- Examples:
  - "I want to know if he wants to invite me" explained through very literal intermediate English
  - "I think to eat later"

Problem:
- Good for discovery, but lesson specs should present normal English prompts too.

### RQ-04: Pronunciation mnemonics without phonetic limits
Status: `Needs review`

- Example:
  - strong sound simplifications that do not generalize cleanly

### RQ-05: Low-confidence semantic claims
Status: `Needs review`

- Examples:
  - exact differences among near-synonyms
  - claims about what is "more common" without corpus support

---

## 17. Recommended Next Files

This v2 reference is the source layer. The next layer should not be more prose. It should be structured lesson specs.

Recommended next artifacts:

- `lesson-spec-template.md`
- `error-taxonomy.yaml`
- `concept-inventory.yaml`
- `allowed-vocabulary-by-stage.yaml`

Minimum fields for each future lesson spec:

- `lesson_id`
- `core_objective`
- `new_concepts`
- `interleave_from`
- `allowed_vocabulary`
- `forbidden_or_not_yet_taught`
- `target_error_types`
- `exercise_mix`
- `teacher_notes`

---

## 18. Suggested Pilot Sequence

Use these as the first generation-safe blocks:

### Pilot Block 1

- cognate awareness with limits
- infinitives
- `quiero`, `voy a`, `intento`
- direct object pronouns `lo`, `la`

### Pilot Block 2

- `debo`, `tengo que`, `puedo`
- present tense anchors
- very common irregular first-person forms
- `sí`, `qué`, `por qué`

### Pilot Block 3

- indirect objects `me`, `te`, `le`, `nos`
- pronoun placement with infinitives
- `dar`, `decir`, `hacer` in high-frequency frames

### Pilot Block 4

- gender and number
- adjective agreement
- `ser` vs `estar`
- present with future context

### Pilot Block 5

- present progressive
- present perfect
- `gustar`-type verbs
- basic prepositions and personal `a`

This gives enough structure for deliberate practice and interleaving without flooding the learner.
