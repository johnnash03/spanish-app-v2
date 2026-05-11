import { render, screen, fireEvent } from "@testing-library/react";
import { vi } from "vitest";
import { SessionReviewScreen } from "./SessionReviewScreen";
import type { LocalAttempt } from "../types";

const makeAttempt = (
  itemId: string,
  tag: string,
  learnerAnswer: string,
): LocalAttempt => ({ itemId, tag, learnerAnswer, source: `${itemId} source` });

vi.mock("../lib/tauri", () => ({
  isTauri: () => true,
  evaluateSession: vi.fn(),
}));

async function mockEval(results: object[]) {
  const { evaluateSession } = await import("../lib/tauri");
  vi.mocked(evaluateSession).mockResolvedValueOnce({
    sessionId: "sess-1",
    results,
  } as never);
}

describe("SessionReviewScreen – error cascade CTA", () => {
  it("does not show CTA when fewer than 3 errors on same tag", async () => {
    await mockEval([
      {
        itemId: "i1",
        correct: false,
        errorTag: "ser-estar",
        remarks: [],
        explanation: null,
        canonical: "soy",
      },
      {
        itemId: "i2",
        correct: false,
        errorTag: "ser-estar",
        remarks: [],
        explanation: null,
        canonical: "eres",
      },
    ]);

    render(
      <SessionReviewScreen
        attempts={[
          makeAttempt("i1", "ser-estar", "estoy"),
          makeAttempt("i2", "ser-estar", "eres"),
        ]}
        go={vi.fn()}
      />,
    );

    await screen.findByText(/0 of 2 correct/i);
    expect(
      screen.queryByText(/struggling with this skill/i),
    ).not.toBeInTheDocument();
  });

  it("shows CTA with practice button when 3+ errors on same tag", async () => {
    await mockEval([
      {
        itemId: "i1",
        correct: false,
        errorTag: "ser-estar",
        remarks: [],
        explanation: null,
        canonical: "soy",
      },
      {
        itemId: "i2",
        correct: false,
        errorTag: "ser-estar",
        remarks: [],
        explanation: null,
        canonical: "eres",
      },
      {
        itemId: "i3",
        correct: false,
        errorTag: "ser-estar",
        remarks: [],
        explanation: null,
        canonical: "es",
      },
    ]);

    render(
      <SessionReviewScreen
        attempts={[
          makeAttempt("i1", "ser-estar", "estoy"),
          makeAttempt("i2", "ser-estar", "eres"),
          makeAttempt("i3", "ser-estar", "esta"),
        ]}
        go={vi.fn()}
      />,
    );

    await screen.findByText(/0 of 3/i);
    expect(screen.getByText(/struggling with this skill/i)).toBeInTheDocument();
    expect(
      screen.getByRole("button", { name: /practice this skill/i }),
    ).toBeInTheDocument();
  });

  it("CTA button navigates to practiceSession with the cascade tag", async () => {
    await mockEval([
      {
        itemId: "i1",
        correct: false,
        errorTag: "ser-estar",
        remarks: [],
        explanation: null,
        canonical: "soy",
      },
      {
        itemId: "i2",
        correct: false,
        errorTag: "ser-estar",
        remarks: [],
        explanation: null,
        canonical: "eres",
      },
      {
        itemId: "i3",
        correct: false,
        errorTag: "ser-estar",
        remarks: [],
        explanation: null,
        canonical: "es",
      },
    ]);

    const go = vi.fn();
    render(
      <SessionReviewScreen
        attempts={[
          makeAttempt("i1", "ser-estar", "a"),
          makeAttempt("i2", "ser-estar", "b"),
          makeAttempt("i3", "ser-estar", "c"),
        ]}
        go={go}
      />,
    );

    await screen.findByText(/0 of 3/i);
    fireEvent.click(
      screen.getByRole("button", { name: /practice this skill/i }),
    );
    expect(go).toHaveBeenCalledWith({
      name: "practiceSession",
      tagId: "ser-estar",
      tagName: "ser estar",
    });
  });
});
