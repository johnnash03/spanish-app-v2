// Mock data — mid-journey learner state
const MOCK = {
  learner: {
    masteredCount: 247,
    activeWords: 17,
    learningCount: 6,
    newCount: 3,
    dueCount: 12,
    weakTags: [
      { id: 'ser-estar', name: 'Ser vs Estar', wrongOf20: 6 },
      { id: 'preterite-irregulars', name: 'Preterite irregulars', wrongOf20: 5 },
      { id: 'ind-obj-pronouns', name: 'Indirect object pronouns', wrongOf20: 4 },
    ],
    pipelineStatus: { label: 'Healthy', detail: 'room for 3 more', tone: 'healthy' },
    currentUnit: { phase: 2, number: 7, name: 'Preterite — regular verbs', toward: 12, of: 20 },
    continueSession: { track: 'Grammar', label: 'Phase 2 · Unit 7', combinedReady: true },
  },

  phases: [
    {
      number: 1, name: 'Present indicative & basics',
      units: [
        { n: 1, name: 'Subject pronouns & ser', status: 'complete' },
        { n: 2, name: 'Estar & location', status: 'complete' },
        { n: 3, name: 'Regular -ar verbs (present)', status: 'complete' },
        { n: 4, name: 'Regular -er and -ir verbs', status: 'complete' },
        { n: 5, name: 'Articles and gender', status: 'complete' },
      ],
    },
    {
      number: 2, name: 'Past tenses',
      units: [
        { n: 6, name: 'Preterite — overview', status: 'complete' },
        { n: 7, name: 'Preterite — regular verbs', status: 'in-progress' },
        { n: 8, name: 'Preterite — irregular verbs', status: 'not-started' },
        { n: 9, name: 'Imperfect tense', status: 'not-started' },
        { n: 10, name: 'Preterite vs Imperfect', status: 'not-started' },
      ],
    },
    {
      number: 3, name: 'Object pronouns & reflexives',
      units: [
        { n: 11, name: 'Direct object pronouns', status: 'not-started' },
        { n: 12, name: 'Indirect object pronouns', status: 'not-started' },
        { n: 13, name: 'Reflexive verbs', status: 'not-started' },
      ],
    },
    {
      number: 4, name: 'Subjunctive foundations',
      units: [
        { n: 14, name: 'Present subjunctive — formation', status: 'not-started' },
        { n: 15, name: 'Subjunctive — wishes & doubt', status: 'not-started' },
      ],
    },
  ],

  currentUnit: {
    phase: 2,
    number: 7,
    name: 'Preterite — regular verbs',
    description: 'Build the regular preterite conjugations for -ar, -er, and -ir verbs. Practice yo / tú / él / nosotros forms with action verbs you already know.',
    prereqMet: true,
    reading: {
      sections: [
        {
          id: 'when',
          title: 'When you reach for it',
          body: 'The preterite is Spanish\'s "snapshot" past tense. Use it for actions you can pin to a moment — *Cené a las ocho*, *Llegamos ayer*. If the action has a beginning and an end inside the sentence, it\'s preterite. If you\'re painting a backdrop ("she was reading", "we used to live there"), that\'s the imperfect, which lives in Unit 9.',
        },
        {
          id: 'formation',
          title: 'Formation',
          body: 'Drop the -ar / -er / -ir ending from the infinitive and add the preterite endings. The -er and -ir endings are identical — that\'s a small mercy. Stress falls on the ending in the yo and él/ella forms (caminé, caminó), so the written accent is doing real work — it tells you where to land your voice.',
          table: {
            head: ['', 'caminar (-ar)', 'comer (-er)', 'vivir (-ir)'],
            rows: [
              ['yo', 'caminé', 'comí', 'viví'],
              ['tú', 'caminaste', 'comiste', 'viviste'],
              ['él / ella / Ud.', 'caminó', 'comió', 'vivió'],
              ['nosotros', 'caminamos', 'comimos', 'vivimos'],
              ['vosotros', 'caminasteis', 'comisteis', 'vivisteis'],
              ['ellos / Uds.', 'caminaron', 'comieron', 'vivieron'],
            ],
          },
        },
        {
          id: 'examples',
          title: 'In the wild',
          examples: [
            { es: 'Caminé al café ayer por la tarde.', en: 'I walked to the café yesterday afternoon.' },
            { es: 'Cenamos tarde anoche.', en: 'We ate dinner late last night.' },
            { es: '¿A qué hora llegaste?', en: 'What time did you arrive?' },
            { es: 'Vivieron en Madrid dos años.', en: 'They lived in Madrid for two years.' },
          ],
        },
        {
          id: 'pitfalls',
          title: 'Watch out for',
          bullets: [
            'The nosotros form for -ar and -ir verbs is identical to the present tense (caminamos, vivimos). Context — and any time marker like *ayer* or *anoche* — does the disambiguating.',
            'The yo and él/ella forms always carry a written accent. Without it, *camino* (I walk) and *caminó* (he walked) collapse into one word.',
            'Spelling shifts before -é: -car → -qué (busqué), -gar → -gué (llegué), -zar → -cé (empecé). Sound, not pattern.',
          ],
        },
      ],
    },
    glossary: [
      { lemma: 'caminar', en: 'to walk' },
      { lemma: 'comer', en: 'to eat' },
      { lemma: 'vivir', en: 'to live' },
      { lemma: 'ayer', en: 'yesterday' },
      { lemma: 'anoche', en: 'last night' },
      { lemma: 'la semana pasada', en: 'last week' },
      { lemma: 'el café', en: 'coffee / café' },
      { lemma: 'temprano', en: 'early' },
    ],
  },

  // Practice items with mock results so review screen reads well
  reviewItems: {
    correct: 12, total: 15,
    masteredTags: ['Preterite — regular -ar verbs'],
    wrongs: [
      {
        en: 'I walked to the café yesterday.',
        user: 'Caminé al café ayer.',
        correct: 'Caminé al café ayer.',
        right: true, // we'll override into wrong below for display variety
      },
    ],
    actualWrongs: [
      {
        en: 'She ate dinner late last night.',
        user: 'Ella comió cena tarde anoche.',
        correct: 'Ella cenó tarde anoche.',
        hint: 'Spanish has a single verb for "to eat dinner". Reach for it instead of the literal pair.',
        explain: 'Cenar means "to eat dinner / to have supper." When the meal itself is the action, Spanish prefers the dedicated verb (desayunar, almorzar, cenar) over comer + meal.',
      },
      {
        en: 'We lived in Madrid last week.',
        user: 'Vivemos en Madrid la semana pasada.',
        correct: 'Vivimos en Madrid la semana pasada.',
        hint: 'Check the nosotros ending for -ir verbs in the preterite.',
        explain: 'For regular -ir verbs, the nosotros preterite ending is -imos (vivimos), not -emos. The form happens to look identical to the present tense — context disambiguates.',
      },
      {
        en: 'They arrived early.',
        user: 'Ellos llegaron tempranamente.',
        correct: 'Ellos llegaron temprano.',
        hint: 'Spanish often uses the bare adjective as an adverb where English uses -ly.',
        explain: 'Temprano works both as adjective and adverb — no -mente needed. Tempranamente exists but sounds bookish; native speakers say llegaron temprano.',
      },
    ],
    corrects: [
      { en: 'I walked to the café yesterday.', user: 'Caminé al café ayer.' },
      { en: 'You (tú) ate at home.', user: 'Comiste en casa.' },
      { en: 'He drank coffee.', user: 'Él bebió café.' },
      { en: 'We learned a lot.', user: 'Aprendimos mucho.' },
      { en: 'I lived there for two years.', user: 'Viví allí por dos años.' },
      { en: 'She wrote three letters.', user: 'Ella escribió tres cartas.' },
    ],
  },

  // Vocab bank — top of frequency list, mixed states
  vocabBank: [
    { rank: 1, lemma: 'el', en: 'the (m.)', pos: 'art.', state: 'mastered' },
    { rank: 2, lemma: 'de', en: 'of, from', pos: 'prep.', state: 'mastered' },
    { rank: 3, lemma: 'que', en: 'that, which', pos: 'pron.', state: 'mastered' },
    { rank: 4, lemma: 'y', en: 'and', pos: 'conj.', state: 'mastered' },
    { rank: 5, lemma: 'a', en: 'to, at', pos: 'prep.', state: 'mastered' },
    { rank: 87, lemma: 'libro', en: 'book', pos: 'n. m.', state: 'mastered' },
    { rank: 142, lemma: 'gato', en: 'cat', pos: 'n. m.', state: 'mastered' },
    { rank: 198, lemma: 'ventana', en: 'window', pos: 'n. f.', state: 'learning' },
    { rank: 213, lemma: 'caminar', en: 'to walk', pos: 'v.', state: 'learning' },
    { rank: 247, lemma: 'cenar', en: 'to eat dinner', pos: 'v.', state: 'learning' },
    { rank: 284, lemma: 'temprano', en: 'early', pos: 'adv.', state: 'learning' },
    { rank: 312, lemma: 'anoche', en: 'last night', pos: 'adv.', state: 'learning' },
    { rank: 358, lemma: 'tardar', en: 'to take (time), delay', pos: 'v.', state: 'new' },
    { rank: 401, lemma: 'recoger', en: 'to pick up, gather', pos: 'v.', state: 'new' },
    { rank: 457, lemma: 'aprovechar', en: 'to take advantage of', pos: 'v.', state: 'new' },
    { rank: 502, lemma: 'mientras', en: 'while', pos: 'conj.', state: 'untouched' },
    { rank: 568, lemma: 'orilla', en: 'shore, edge', pos: 'n. f.', state: 'untouched' },
    { rank: 624, lemma: 'aliento', en: 'breath, encouragement', pos: 'n. m.', state: 'untouched' },
  ],

  newWords: [
    { rank: 401, lemma: 'recoger', en: 'to pick up, gather', pos: 'verb' },
    { rank: 457, lemma: 'aprovechar', en: 'to take advantage of', pos: 'verb' },
    { rank: 502, lemma: 'mientras', en: 'while', pos: 'conjunction' },
    { rank: 568, lemma: 'orilla', en: 'shore, edge', pos: 'noun, feminine' },
    { rank: 624, lemma: 'aliento', en: 'breath, encouragement', pos: 'noun, masculine' },
  ],

  flashcardCurrent: {
    type: 'mc',
    lemma: 'temprano',
    options: ['often', 'early', 'sometimes', 'late'],
  },
  flashcardCurrentRecall: {
    type: 'recall',
    lemma: 'orilla',
    en: 'shore, edge',
    revealed: false,
  },
};

window.MOCK = MOCK;
