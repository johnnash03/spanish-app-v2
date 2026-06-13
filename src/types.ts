export type UnitStatus = "not-started" | "in-progress" | "complete";

export type GenerationState = "idle" | "generating" | "ready" | "failed";

export interface NoteEntry {
  term: string;
  definition: string;
}

export interface Unit {
  n: number;
  name: string;
  description?: string;
  status: UnitStatus;
  phase: number;
  skillTag?: string;
  generationState?: GenerationState;
  notes?: NoteEntry[];
  prerequisites?: string[];
}

export interface Phase {
  number: number;
  name: string;
  units: Unit[];
}

export interface WeakTag {
  id: string;
  name: string;
  wrongOf20: number;
}

export interface PipelineStatus {
  label: string;
  detail: string;
  tone: "light" | "healthy" | "full" | "overloaded";
}

export interface LearnerState {
  masteredCount: number;
  activeWords: number;
  learningCount: number;
  newCount: number;
  dueCount: number;
  weakTags: WeakTag[];
  pipelineStatus: PipelineStatus;
  currentUnit: {
    phase: number;
    number: number;
    name: string;
    toward: number;
    of: number;
  };
  continueSession: {
    track: string;
    label: string;
    combinedReady: boolean;
  };
}

export interface SessionItem {
  id: string;
  source: string;
  primaryTag: string;
  stackedTags: string[];
  vocabLemmas?: string[];
}

export interface LocalAttempt {
  itemId: string;
  tag: string;
  learnerAnswer: string;
  source: string;
}

export interface EvaluationResult {
  itemId: string;
  correct: boolean;
  errorTag: string | null;
  remarks: string[];
  explanation: string | null;
  canonical: string;
}

export interface EvalSessionResponse {
  sessionId: string;
  results: EvaluationResult[];
}

export interface VocabWord {
  lemma: string;
  translation: string;
  frequencyRank: number;
  partOfSpeech: string;
}

export type PipelineBand = "light" | "healthy" | "full" | "overloaded";

export interface PipelineHealth {
  activeCount: number;
  band: PipelineBand;
}

export interface SrsCard {
  lemma: string;
  translation: string;
  frequencyRank: number;
  partOfSpeech: string;
  pipelineState: string;
  intervalDays: number;
  repetitions: number;
  selfRated: boolean;
  distractors: string[];
}

export interface VocabCardResult {
  card: SrsCard;
  correct: boolean;
}

// ─── V2 (PRD #31) ────────────────────────────────────────────────────────────

export interface V2Unit {
  id: string;
  title: string;
  phase: number;
  bankCount: number;
  generationState: GenerationState;
}

export interface V2SessionItem {
  id: string;
  source: string;
  targetSkill: string;
}

// `dodge` is a structure-avoiding correct answer (S7, #38): shown to the
// learner as correct with a nudge, worth no mastery credit.
export type V2AttemptStatus = "correct" | "pending" | "wrong" | "dodge";

export interface V2AttemptVerdict {
  attemptId: string;
  itemId: string;
  status: V2AttemptStatus;
  remarks: string[];
}

export interface V2ReviewAttempt {
  itemId: string;
  source: string;
  answer: string;
  status: V2AttemptStatus;
  remarks: string[];
  canonical: string;
  targetSkill: string;
  errorCategory: string | null;
  hint: string | null;
  explanation: string | null;
}

export type Screen =
  | { name: "home" }
  | { name: "units" }
  | { name: "unitDetail"; unitN: number }
  | { name: "session"; unitSkillTag: string }
  | { name: "sessionReview"; attempts: LocalAttempt[] }
  | { name: "practiceEntry" }
  | { name: "practiceSession"; tagId: string | null; tagName: string | null }
  | {
      name: "practiceReview";
      attempts: LocalAttempt[];
      practicedWeakTags: WeakTag[];
    }
  | { name: "vocabIntake" }
  | { name: "vocabSession" }
  | { name: "vocabReview"; results: VocabCardResult[] }
  | { name: "combinedSession" }
  | {
      name: "combinedReview";
      attempts: LocalAttempt[];
      vocabLemmasByItemId: Record<string, string[]>;
    }
  | { name: "v2Units" }
  | { name: "v2Session"; unitId: string; unitTitle: string }
  | { name: "v2Review"; attempts: V2ReviewAttempt[]; sessionId: string };
