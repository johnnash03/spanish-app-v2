export type UnitStatus = "not-started" | "in-progress" | "complete";

export interface Unit {
  n: number;
  name: string;
  description: string;
  status: UnitStatus;
  phase: number;
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

export type Screen =
  | { name: "home" }
  | { name: "units" }
  | { name: "unitDetail"; unitN: number };
