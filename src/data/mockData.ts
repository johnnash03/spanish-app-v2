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
      },
      {
        n: 2,
        name: "Estar & location",
        description:
          "Master estar, the second 'to be' verb. Focus on location, temporary states, and the essential ser vs. estar contrast.",
        status: "complete",
        phase: 1,
      },
      {
        n: 3,
        name: "Regular -ar verbs (present)",
        description:
          "Build the present-tense conjugation pattern for regular -ar verbs across all six persons.",
        status: "complete",
        phase: 1,
      },
      {
        n: 4,
        name: "Regular -er and -ir verbs",
        description:
          "Extend the present tense to -er and -ir verbs. Note where -er and -ir endings coincide.",
        status: "complete",
        phase: 1,
      },
      {
        n: 5,
        name: "Articles and gender",
        description:
          "Understand how grammatical gender works in Spanish and when to use definite vs. indefinite articles.",
        status: "complete",
        phase: 1,
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
      },
      {
        n: 7,
        name: "Preterite — regular verbs",
        description:
          "Build the regular preterite conjugations for -ar, -er, and -ir verbs. Practice yo / tú / él / nosotros forms with action verbs you already know.",
        status: "in-progress",
        phase: 2,
      },
      {
        n: 8,
        name: "Preterite — irregular verbs",
        description:
          "Tackle the most common irregular preterite stems: ir/ser, tener, estar, hacer, and more.",
        status: "not-started",
        phase: 2,
      },
      {
        n: 9,
        name: "Imperfect tense",
        description:
          "Learn the imperfect tense for ongoing past states, habitual actions, and scene-setting.",
        status: "not-started",
        phase: 2,
      },
      {
        n: 10,
        name: "Preterite vs Imperfect",
        description:
          "The core contrast between Spanish's two past tenses — when each applies and how to navigate the boundary.",
        status: "not-started",
        phase: 2,
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
      },
      {
        n: 12,
        name: "Indirect object pronouns",
        description:
          "Add indirect object pronouns to your toolkit and handle the double-pronoun combinations that follow.",
        status: "not-started",
        phase: 3,
      },
      {
        n: 13,
        name: "Reflexive verbs",
        description:
          "Understand reflexive constructions and the wide range of verbs that take reflexive pronouns in Spanish.",
        status: "not-started",
        phase: 3,
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
      },
      {
        n: 15,
        name: "Subjunctive — wishes & doubt",
        description:
          "Apply the subjunctive in the contexts that trigger it most: wishes, recommendations, and expressions of doubt.",
        status: "not-started",
        phase: 4,
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
