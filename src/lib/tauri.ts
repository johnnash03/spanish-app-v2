import { invoke } from "@tauri-apps/api/core";
import type { GenerationState } from "../types";

export function isTauri(): boolean {
  return typeof window !== "undefined" && "__TAURI__" in window;
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
