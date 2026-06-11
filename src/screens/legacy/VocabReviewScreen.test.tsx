import { render, screen, fireEvent } from "@testing-library/react";
import { vi } from "vitest";
import { VocabReviewScreen } from "./VocabReviewScreen";
import type { VocabCardResult } from "../../types";

const makeCard = (lemma: string, translation: string) => ({
  lemma,
  translation,
  frequencyRank: 1,
  partOfSpeech: "verb",
  pipelineState: "learning",
  intervalDays: 1,
  repetitions: 1,
  selfRated: false,
  distractors: [],
});

const RESULTS: VocabCardResult[] = [
  { card: makeCard("comer", "to eat"), correct: true },
  { card: makeCard("hablar", "to speak"), correct: true },
  { card: makeCard("correr", "to run"), correct: false },
];

describe("VocabReviewScreen", () => {
  it("shows correct and incorrect counts", () => {
    render(<VocabReviewScreen results={RESULTS} go={vi.fn()} />);
    expect(screen.getByText("2")).toBeInTheDocument(); // 2 correct
    expect(screen.getByText("1")).toBeInTheDocument(); // 1 again
  });

  it("shows wrong words in the review section", () => {
    render(<VocabReviewScreen results={RESULTS} go={vi.fn()} />);
    expect(screen.getByText("correr")).toBeInTheDocument();
    expect(screen.getByText("to run")).toBeInTheDocument();
  });

  it("shows correct words as compact chips", () => {
    render(<VocabReviewScreen results={RESULTS} go={vi.fn()} />);
    expect(screen.getByText("comer")).toBeInTheDocument();
    expect(screen.getByText("hablar")).toBeInTheDocument();
  });

  it("navigates home when back button is clicked", () => {
    const go = vi.fn();
    render(<VocabReviewScreen results={RESULTS} go={go} />);
    fireEvent.click(screen.getByText("Back to home"));
    expect(go).toHaveBeenCalledWith({ name: "home" });
  });

  it("shows perfect message when all correct", () => {
    const perfect = RESULTS.map((r) => ({ ...r, correct: true }));
    render(<VocabReviewScreen results={perfect} go={vi.fn()} />);
    expect(screen.getByText("Perfect session")).toBeInTheDocument();
  });

  it("shows empty state message when no results", () => {
    render(<VocabReviewScreen results={[]} go={vi.fn()} />);
    expect(screen.getByText("Nothing reviewed")).toBeInTheDocument();
  });
});
