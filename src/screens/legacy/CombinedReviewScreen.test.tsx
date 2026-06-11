import { render, screen } from "@testing-library/react";
import { vi } from "vitest";
import { CombinedReviewScreen } from "./CombinedReviewScreen";
import type { LocalAttempt } from "../../types";

vi.mock("../../lib/tauri", () => ({
  isTauri: () => true,
  evaluateSession: vi.fn().mockResolvedValue({
    sessionId: "sess-1",
    results: [
      {
        itemId: "item-1",
        correct: true,
        errorTag: null,
        remarks: [],
        explanation: null,
        canonical: "El libro está en la mesa.",
      },
      {
        itemId: "item-2",
        correct: false,
        errorTag: "articles",
        remarks: [],
        explanation: "Use the definite article here.",
        canonical: "Quiero comer la manzana.",
      },
    ],
  }),
  recordCombinedSessionReviews: vi.fn().mockResolvedValue(undefined),
}));

const attempts: LocalAttempt[] = [
  {
    itemId: "item-1",
    tag: "articles",
    learnerAnswer: "El libro está en la mesa.",
    source: "The book is on the table.",
  },
  {
    itemId: "item-2",
    tag: "ar-verbs",
    learnerAnswer: "Quiero comer manzana.",
    source: "I want to eat the apple.",
  },
];

const vocabLemmasByItemId: Record<string, string[]> = {
  "item-1": ["libro", "mesa"],
  "item-2": ["manzana"],
};

describe("CombinedReviewScreen", () => {
  it("shows correct count in header", async () => {
    render(
      <CombinedReviewScreen
        attempts={attempts}
        vocabLemmasByItemId={vocabLemmasByItemId}
        go={vi.fn()}
      />,
    );
    expect(await screen.findByText(/1 of 2 correct/i)).toBeInTheDocument();
  });

  const isAnnotationP = (_: string, el: Element | null) =>
    el?.tagName?.toLowerCase() === "p" &&
    (el?.textContent ?? "").includes("advanced in pipeline");

  it("shows vocab advancement annotations for correct items", async () => {
    render(
      <CombinedReviewScreen
        attempts={attempts}
        vocabLemmasByItemId={vocabLemmasByItemId}
        go={vi.fn()}
      />,
    );
    const annotations = await screen.findAllByText(isAnnotationP);
    const texts = annotations.map((el) => el.textContent ?? "");
    expect(texts.some((t) => t.includes("libro"))).toBe(true);
    expect(texts.some((t) => t.includes("mesa"))).toBe(true);
  });

  it("does not show vocab annotations for incorrect items", async () => {
    render(
      <CombinedReviewScreen
        attempts={attempts}
        vocabLemmasByItemId={vocabLemmasByItemId}
        go={vi.fn()}
      />,
    );
    await screen.findByText(/1 of 2 correct/i);
    const annotations = screen.queryAllByText(isAnnotationP);
    expect(
      annotations.every((el) => !(el.textContent ?? "").includes("manzana")),
    ).toBe(true);
  });

  it("calls recordCombinedSessionReviews with lemmas from correct items only", async () => {
    const { recordCombinedSessionReviews } = await import("../../lib/tauri");
    render(
      <CombinedReviewScreen
        attempts={attempts}
        vocabLemmasByItemId={vocabLemmasByItemId}
        go={vi.fn()}
      />,
    );
    await screen.findByText(/1 of 2 correct/i);
    expect(recordCombinedSessionReviews).toHaveBeenCalledWith([
      "libro",
      "mesa",
    ]);
  });
});
