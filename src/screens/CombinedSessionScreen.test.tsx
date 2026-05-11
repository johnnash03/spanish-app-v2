import { render, screen } from "@testing-library/react";
import { vi, beforeEach } from "vitest";
import { CombinedSessionScreen } from "./CombinedSessionScreen";

vi.mock("../lib/tauri", () => ({
  isTauri: () => true,
  assembleCombinedQueue: vi.fn().mockResolvedValue([
    {
      id: "item-1",
      source: "The book is on the table.",
      primaryTag: "articles",
      stackedTags: [],
      vocabLemmas: ["libro", "mesa"],
    },
    {
      id: "item-2",
      source: "I want to eat the apple.",
      primaryTag: "ar-verbs",
      stackedTags: [],
      vocabLemmas: ["manzana"],
    },
  ]),
}));

beforeEach(() => {
  vi.clearAllMocks();
});

describe("CombinedSessionScreen", () => {
  it("shows exercises from the queue without tag labels", async () => {
    render(<CombinedSessionScreen go={vi.fn()} />);
    expect(
      await screen.findByText("The book is on the table."),
    ).toBeInTheDocument();
    expect(screen.getByText("I want to eat the apple.")).toBeInTheDocument();
  });

  it("does not show tag labels above exercises", async () => {
    render(<CombinedSessionScreen go={vi.fn()} />);
    await screen.findByText("The book is on the table.");
    expect(screen.queryByText("articles")).not.toBeInTheDocument();
    expect(screen.queryByText("ar-verbs")).not.toBeInTheDocument();
  });

  it("shows empty state when no exercises are available after timeout", async () => {
    vi.useFakeTimers();
    const { assembleCombinedQueue } = await import("../lib/tauri");
    vi.mocked(assembleCombinedQueue).mockResolvedValue([]);
    render(<CombinedSessionScreen go={vi.fn()} />);
    // Let the first poll complete (returns [], starts generating state).
    await vi.runAllTimersAsync();
    // Advance past the 120s timeout so the next poll flips to empty.
    vi.advanceTimersByTime(120_001);
    await vi.runAllTimersAsync();
    expect(screen.getByText(/no exercises available/i)).toBeInTheDocument();
    vi.useRealTimers();
  });
});
