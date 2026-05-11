import { invoke } from "@tauri-apps/api/core";
import type {
  GenerationState,
  VocabWord,
  PipelineHealth,
  SrsCard,
} from "../types";

export function isTauri(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

export async function triggerGeneration(
  skillTag: string,
): Promise<GenerationState> {
  const state = await invoke<string>("trigger_generation", { skillTag });
  return state as GenerationState;
}

export async function getUnitGenerationState(
  skillTag: string,
): Promise<GenerationState> {
  const state = await invoke<string>("get_unit_generation_state", { skillTag });
  return state as GenerationState;
}

export async function retryGeneration(skillTag: string): Promise<void> {
  await invoke("retry_generation", { skillTag });
}

export async function assembleSessionQueue(
  activeUnitTag: string,
): Promise<import("../types").SessionItem[]> {
  return invoke("assemble_session_queue", { activeUnitTag });
}

export async function listUnits(): Promise<import("../types").Unit[]> {
  const rows = await invoke<
    {
      n: number;
      name: string;
      phase: number;
      skillTag: string;
      generationState: string;
      status: string;
      prerequisites: string[];
    }[]
  >("list_units");
  return rows.map((r) => ({
    n: r.n,
    name: r.name,
    phase: r.phase,
    skillTag: r.skillTag,
    generationState: r.generationState as import("../types").GenerationState,
    status: r.status as import("../types").UnitStatus,
    prerequisites: r.prerequisites,
  }));
}

export async function getUnitByN(
  n: number,
): Promise<import("../types").Unit | null> {
  const row = await invoke<{
    n: number;
    name: string;
    phase: number;
    skillTag: string;
    generationState: string;
    status: string;
    prerequisites: string[];
  } | null>("get_unit_by_n", { n });
  if (!row) return null;
  return {
    n: row.n,
    name: row.name,
    phase: row.phase,
    skillTag: row.skillTag,
    generationState: row.generationState as import("../types").GenerationState,
    status: row.status as import("../types").UnitStatus,
    prerequisites: row.prerequisites,
  };
}

export async function getCurrentUnitNumber(): Promise<number> {
  return invoke<number>("get_current_unit_number");
}

export async function submitSessionAttempts(
  attempts: import("../types").LocalAttempt[],
): Promise<void> {
  await invoke("submit_session_attempts", {
    attempts: attempts.map((a) => ({
      itemId: a.itemId,
      tag: a.tag,
      learnerAnswer: a.learnerAnswer,
    })),
  });
}

export async function evaluateSession(
  sessionId: string | null,
  attempts: import("../types").LocalAttempt[],
): Promise<import("../types").EvalSessionResponse> {
  return invoke("evaluate_session", {
    sessionId,
    attempts: attempts.map((a) => ({
      itemId: a.itemId,
      tag: a.tag,
      learnerAnswer: a.learnerAnswer,
    })),
  });
}

export async function getWeakTags(): Promise<import("../types").WeakTag[]> {
  return invoke("get_weak_tags");
}

export async function assembleDpQueue(): Promise<
  import("../types").SessionItem[]
> {
  return invoke("assemble_dp_queue");
}

export async function triggerDpGeneration(): Promise<void> {
  await invoke("trigger_dp_generation");
}

export async function getPendingSession(): Promise<
  import("../types").LocalAttempt[] | null
> {
  const result = await invoke<
    | {
        itemId: string;
        tag: string;
        learnerAnswer: string;
        source: string;
      }[]
    | null
  >("get_pending_session");
  if (!result) return null;
  return result.map((r) => ({
    itemId: r.itemId,
    tag: r.tag,
    learnerAnswer: r.learnerAnswer,
    source: r.source,
  }));
}

export async function getNextUntouchedWords(
  count: number,
): Promise<VocabWord[]> {
  return invoke<VocabWord[]>("get_next_untouched_words", { count });
}

export async function commitIntakeBatch(lemmas: string[]): Promise<void> {
  await invoke("commit_intake_batch", { lemmas });
}

export async function getPipelineHealth(): Promise<PipelineHealth> {
  return invoke<PipelineHealth>("get_pipeline_health");
}

export async function getVocabSessionCards(limit: number): Promise<SrsCard[]> {
  return invoke<SrsCard[]>("get_vocab_session_cards", { limit });
}

export async function recordVocabReview(
  lemma: string,
  correct: boolean,
): Promise<void> {
  await invoke("record_vocab_review", { lemma, correct });
}

export async function markVocabWordMastered(lemma: string): Promise<boolean> {
  return invoke<boolean>("mark_vocab_word_mastered", { lemma });
}

export async function assembleCombinedQueue(): Promise<
  import("../types").SessionItem[]
> {
  return invoke("assemble_combined_queue");
}

export async function recordCombinedSessionReviews(
  correctLemmas: string[],
): Promise<void> {
  await invoke("record_combined_session_reviews", { correctLemmas });
}
