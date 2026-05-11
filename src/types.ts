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
  | { name: "vocabIntake" };
