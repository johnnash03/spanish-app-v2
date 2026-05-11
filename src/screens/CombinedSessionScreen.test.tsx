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

  it("shows empty state when no exercises are available", async () => {
    const { assembleCombinedQueue } = await import("../lib/tauri");
    vi.mocked(assembleCombinedQueue).mockResolvedValueOnce([]);
    render(<CombinedSessionScreen go={vi.fn()} />);
    expect(
      await screen.findByText(/no exercises available/i),
    ).toBeInTheDocument();
  });
});
