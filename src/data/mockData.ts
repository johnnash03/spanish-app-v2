import type { LearnerState, Phase } from "../types";

export const PHASES: Phase[] = [
  {
    number: 1,
    name: "Present indicative & basics",
    units: [
      {
        n: 1,
        name: "Subject pronouns & ser",
        description:
          "Learn the subject pronouns and master ser — the first of two Spanish verbs meaning 'to be.' Practice identity, origin, and description.",
        status: "complete",
        phase: 1,
        skillTag: "opener.quiero",
        generationState: "ready",
        notes: [
          {
            term: "yo / tú / él / ella",
            definition:
              "I / you / he / she — the four singular subject pronouns",
          },
          {
            term: "nosotros / vosotros / ellos",
            definition: "we / you all / they — the plural subject pronouns",
          },
          {
            term: "ser",
            definition:
              "to be (permanent or inherent qualities: identity, origin, profession)",
          },
          {
            term: "soy / eres / es / somos / sois / son",
            definition:
              "present-tense conjugation of ser across all six persons",
          },
        ],
      },
      {
        n: 2,
        name: "Estar & location",
        description:
          "Master estar, the second 'to be' verb. Focus on location, temporary states, and the essential ser vs. estar contrast.",
        status: "complete",
        phase: 1,
        skillTag: "opener.quiero.neg",
        generationState: "ready",
        notes: [
          {
            term: "estar",
            definition:
              "to be (location, temporary states, feelings, conditions)",
          },
          {
            term: "estoy / estás / está / estamos / estáis / están",
            definition:
              "present-tense conjugation of estar across all six persons",
          },
          {
            term: "ser vs. estar",
            definition:
              "ser for permanent identity; estar for location and temporary conditions",
          },
          {
            term: "cansado / cansada",
            definition:
              "tired (masc. / fem.) — a common adjective used with estar",
          },
        ],
      },
      {
        n: 3,
        name: "Regular -ar verbs (present)",
        description:
          "Build the present-tense conjugation pattern for regular -ar verbs across all six persons.",
        status: "complete",
        phase: 1,
        skillTag: "opener.puedo",
        generationState: "ready",
        notes: [
          {
            term: "hablar",
            definition:
              "to speak — the model -ar verb used to show the conjugation pattern",
          },
          {
            term: "-o / -as / -a / -amos / -áis / -an",
            definition: "present-tense endings for regular -ar verbs",
          },
          { term: "trabajar", definition: "to work" },
          { term: "escuchar", definition: "to listen" },
          { term: "caminar", definition: "to walk" },
        ],
      },
      {
        n: 4,
        name: "Regular -er and -ir verbs",
        description:
          "Extend the present tense to -er and -ir verbs. Note where -er and -ir endings coincide.",
        status: "complete",
        phase: 1,
        skillTag: "opener.debo",
        generationState: "ready",
        notes: [
          {
            term: "comer / beber",
            definition: "to eat / to drink — model -er verbs",
          },
          {
            term: "vivir / escribir",
            definition: "to live / to write — model -ir verbs",
          },
          {
            term: "-o / -es / -e / -emos / -éis / -en",
            definition: "-er present-tense endings",
          },
          {
            term: "-o / -es / -e / -imos / -ís / -en",
            definition:
              "-ir present-tense endings (nosotros/vosotros differ from -er)",
          },
        ],
      },
      {
        n: 5,
        name: "Articles and gender",
        description:
          "Understand how grammatical gender works in Spanish and when to use definite vs. indefinite articles.",
        status: "complete",
        phase: 1,
        skillTag: "opener.tengo-que",
        generationState: "ready",
        notes: [
          {
            term: "el / la / los / las",
            definition:
              "definite articles (the) — masculine singular/plural and feminine singular/plural",
          },
          {
            term: "un / una / unos / unas",
            definition: "indefinite articles (a, an, some)",
          },
          {
            term: "género",
            definition:
              "grammatical gender — every Spanish noun is masculine or feminine",
          },
          {
            term: "libro / mesa",
            definition:
              "book (masc.) / table (fem.) — common nouns used in exercises",
          },
        ],
      },
    ],
  },
  {
    number: 2,
    name: "Past tenses",
    units: [
      {
        n: 6,
        name: "Preterite — overview",
        description:
          "Introduction to the preterite: what it signals, when Spanish reaches for it, and the key contrast with the imperfect.",
        status: "complete",
        phase: 2,
        skillTag: "opener.voy-a",
        generationState: "ready",
        notes: [
          {
            term: "pretérito indefinido",
            definition:
              "the preterite — used for completed, bounded past events",
          },
          {
            term: "ayer / anoche",
            definition:
              "yesterday / last night — time markers that signal the preterite",
          },
          {
            term: "ya",
            definition:
              "already — often used with preterite to signal a completed action",
          },
        ],
      },
      {
        n: 7,
        name: "Preterite — regular verbs",
        description:
          "Build the regular preterite conjugations for -ar, -er, and -ir verbs. Practice yo / tú / él / nosotros forms with action verbs you already know.",
        status: "in-progress",
        phase: 2,
        skillTag: "opener.mixed",
        generationState: "ready",
        notes: [
          {
            term: "-é / -aste / -ó / -amos / -asteis / -aron",
            definition: "preterite endings for regular -ar verbs",
          },
          {
            term: "-í / -iste / -ió / -imos / -isteis / -ieron",
            definition: "preterite endings for regular -er and -ir verbs",
          },
          {
            term: "hablar → hablé",
            definition: "I spoke — example of yo form in the preterite",
          },
          {
            term: "comer → comió",
            definition: "he/she ate — example of él/ella form in the preterite",
          },
          {
            term: "vivir → vivimos",
            definition: "we lived — example of nosotros form in the preterite",
          },
        ],
      },
      {
        n: 8,
        name: "Preterite — irregular verbs",
        description:
          "Tackle the most common irregular preterite stems: ir/ser, tener, estar, hacer, and more.",
        status: "not-started",
        phase: 2,
        skillTag: "clitic.do.sg.attach",
        generationState: "generating",
        notes: [
          {
            term: "ir / ser → fui, fuiste, fue…",
            definition:
              "ir and ser share identical preterite forms — context determines meaning",
          },
          {
            term: "tener → tuve",
            definition:
              "tener (to have) has the irregular stem tuv- in the preterite",
          },
          {
            term: "hacer → hice",
            definition:
              "hacer (to do/make) has the stem hic- (note the spelling change in 3rd sg: hizo)",
          },
          {
            term: "estar → estuve",
            definition: "estar (to be) has the stem estuv- in the preterite",
          },
          {
            term: "poder → pude",
            definition: "poder (to be able) has the stem pud- in the preterite",
          },
        ],
      },
      {
        n: 9,
        name: "Imperfect tense",
        description:
          "Learn the imperfect tense for ongoing past states, habitual actions, and scene-setting.",
        status: "not-started",
        phase: 2,
        skillTag: "clitic.do.pl.attach",
        generationState: "failed",
        notes: [
          {
            term: "pretérito imperfecto",
            definition:
              "the imperfect — used for ongoing past states, habitual actions, and background description",
          },
          {
            term: "-aba / -abas / -aba / -ábamos / -abais / -aban",
            definition: "imperfect endings for -ar verbs",
          },
          {
            term: "-ía / -ías / -ía / -íamos / -íais / -ían",
            definition: "imperfect endings for -er and -ir verbs",
          },
          {
            term: "siempre / todos los días",
            definition:
              "always / every day — time markers that typically signal the imperfect",
          },
          {
            term: "ser → era / ir → iba / ver → veía",
            definition: "the three irregular imperfect verbs",
          },
        ],
      },
      {
        n: 10,
        name: "Preterite vs Imperfect",
        description:
          "The core contrast between Spanish's two past tenses — when each applies and how to navigate the boundary.",
        status: "not-started",
        phase: 2,
        generationState: "idle",
      },
    ],
  },
  {
    number: 3,
    name: "Object pronouns & reflexives",
    units: [
      {
        n: 11,
        name: "Direct object pronouns",
        description:
          "Replace direct objects with pronouns and practice correct placement — pre-verb, attached to infinitive, and with commands.",
        status: "not-started",
        phase: 3,
        generationState: "idle",
      },
      {
        n: 12,
        name: "Indirect object pronouns",
        description:
          "Add indirect object pronouns to your toolkit and handle the double-pronoun combinations that follow.",
        status: "not-started",
        phase: 3,
        generationState: "idle",
      },
      {
        n: 13,
        name: "Reflexive verbs",
        description:
          "Understand reflexive constructions and the wide range of verbs that take reflexive pronouns in Spanish.",
        status: "not-started",
        phase: 3,
        generationState: "idle",
      },
    ],
  },
  {
    number: 4,
    name: "Subjunctive foundations",
    units: [
      {
        n: 14,
        name: "Present subjunctive — formation",
        description:
          "Form the present subjunctive for regular and key irregular verbs across all persons.",
        status: "not-started",
        phase: 4,
        generationState: "idle",
      },
      {
        n: 15,
        name: "Subjunctive — wishes & doubt",
        description:
          "Apply the subjunctive in the contexts that trigger it most: wishes, recommendations, and expressions of doubt.",
        status: "not-started",
        phase: 4,
        generationState: "idle",
      },
    ],
  },
];

export const LEARNER: LearnerState = {
  masteredCount: 247,
  activeWords: 17,
  learningCount: 6,
  newCount: 3,
  dueCount: 12,
  weakTags: [
    { id: "ser-estar", name: "Ser vs Estar", wrongOf20: 6 },
    { id: "preterite-irregulars", name: "Preterite irregulars", wrongOf20: 5 },
    {
      id: "ind-obj-pronouns",
      name: "Indirect object pronouns",
      wrongOf20: 4,
    },
  ],
  pipelineStatus: {
    label: "Healthy",
    detail: "room for 3 more",
    tone: "healthy",
  },
  currentUnit: {
    phase: 2,
    number: 7,
    name: "Preterite — regular verbs",
    toward: 12,
    of: 20,
  },
  continueSession: {
    track: "Grammar",
    label: "Phase 2 · Unit 7",
    combinedReady: true,
  },
};

export function getAllUnits(): Phase["units"] {
  return PHASES.flatMap((p) => p.units);
}

export function getUnitByN(n: number) {
  return getAllUnits().find((u) => u.n === n);
}

export function hasUnmetPrereqs(unitN: number): boolean {
  const allUnits = getAllUnits();
  const priorUnits = allUnits.filter((u) => u.n < unitN);
  return priorUnits.some((u) => u.status !== "complete");
}

export function getMissingPrereqNames(unitN: number): string[] {
  const allUnits = getAllUnits();
  return allUnits
    .filter((u) => u.n < unitN && u.status !== "complete")
    .map((u) => u.name)
    .slice(0, 2);
}
