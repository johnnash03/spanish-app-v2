import { render, screen } from "@testing-library/react";
import { vi } from "vitest";
import { PracticeReviewScreen } from "./PracticeReviewScreen";
import type { LocalAttempt, WeakTag } from "../../types";

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
        canonical: "Yo estoy cansado.",
      },
      {
        itemId: "item-2",
        correct: true,
        errorTag: null,
        remarks: [],
        explanation: null,
        canonical: "Ella está en casa.",
      },
    ],
  }),
}));

const attempts: LocalAttempt[] = [
  {
    itemId: "item-1",
    tag: "ser-estar",
    learnerAnswer: "estoy",
    source: "Yo ___ cansado.",
  },
  {
    itemId: "item-2",
    tag: "ser-estar",
    learnerAnswer: "está",
    source: "Ella ___ en casa.",
  },
];

const practicedWeakTags: WeakTag[] = [
  { id: "ser-estar", name: "Ser vs Estar", wrongOf20: 6 },
];

describe("PracticeReviewScreen", () => {
  it("shows mastery-restored callout when all tag attempts are correct", async () => {
    render(
      <PracticeReviewScreen
        attempts={attempts}
        practicedWeakTags={practicedWeakTags}
        go={vi.fn()}
      />,
    );
    expect(await screen.findByText(/mastery restored/i)).toBeInTheDocument();
    expect(screen.getByText(/Ser vs Estar/)).toBeInTheDocument();
  });

  it("does not show mastery-restored callout when some attempts are wrong", async () => {
    const { evaluateSession } = await import("../../lib/tauri");
    vi.mocked(evaluateSession).mockResolvedValueOnce({
      sessionId: "sess-2",
      results: [
        {
          itemId: "item-1",
          correct: false,
          errorTag: "ser-estar",
          remarks: [],
          explanation: "Use estar here.",
          canonical: "estoy",
        },
        {
          itemId: "item-2",
          correct: true,
          errorTag: null,
          remarks: [],
          explanation: null,
          canonical: "está",
        },
      ],
    });

    render(
      <PracticeReviewScreen
        attempts={attempts}
        practicedWeakTags={practicedWeakTags}
        go={vi.fn()}
      />,
    );
    await screen.findByText(/1 of 2 correct/i);
    expect(screen.queryByText(/mastery restored/i)).not.toBeInTheDocument();
  });
});
