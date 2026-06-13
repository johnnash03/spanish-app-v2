import { render, screen, fireEvent, act } from "@testing-library/react";
import { vi } from "vitest";
import { V2ReviewScreen } from "./ReviewScreen";
import * as tauri from "../../lib/tauri";
import type { V2ReviewAttempt } from "../../types";

vi.mock("../../lib/tauri", () => ({
  isTauri: vi.fn(() => false),
  v2SessionReview: vi.fn(),
}));

function attempt(over: Partial<V2ReviewAttempt>): V2ReviewAttempt {
  return {
    itemId: "x",
    source: "",
    answer: "",
    status: "correct",
    remarks: [],
    canonical: "",
    targetSkill: "",
    errorCategory: null,
    hint: null,
    explanation: null,
    ...over,
  };
}

const ATTEMPTS: V2ReviewAttempt[] = [
  attempt({
    itemId: "a",
    source: "You can see them.",
    answer: "Los puedes ver.",
    status: "correct",
    canonical: "Puedes verlos.",
    targetSkill: "clitic.do",
  }),
  attempt({
    itemId: "b",
    source: "He wanted to eat.",
    answer: "Queria comer.",
    status: "correct",
    remarks: ["Correct — differs from “Quería comer.” only in accents."],
    canonical: "Quería comer.",
    targetSkill: "past.queria",
  }),
  attempt({
    itemId: "c",
    source: "I want to dance.",
    answer: "Yo querer bailar.",
    status: "pending",
    canonical: "Quiero bailar.",
    targetSkill: "opener.quiero",
  }),
];

describe("V2ReviewScreen", () => {
  beforeEach(() => {
    vi.mocked(tauri.isTauri).mockReturnValue(false);
    vi.mocked(tauri.v2SessionReview).mockReset();
  });

  it("shows the correct count as the hero", () => {
    render(
      <V2ReviewScreen attempts={ATTEMPTS} sessionId="ses-1" go={vi.fn()} />,
    );
    expect(screen.getByText("2 of 3 correct")).toBeInTheDocument();
  });

  it("shows deterministic accent remarks on correct items", () => {
    render(
      <V2ReviewScreen attempts={ATTEMPTS} sessionId="ses-1" go={vi.fn()} />,
    );
    expect(
      screen.getByText(
        "Correct — differs from “Quería comer.” only in accents.",
      ),
    ).toBeInTheDocument();
  });

  it("lists unmatched answers as awaiting evaluation with the target", () => {
    render(
      <V2ReviewScreen attempts={ATTEMPTS} sessionId="ses-1" go={vi.fn()} />,
    );
    expect(screen.getByText(/awaiting evaluation/i)).toBeInTheDocument();
    expect(screen.getByText("Yo querer bailar.")).toBeInTheDocument();
    expect(screen.getByText("Quiero bailar.")).toBeInTheDocument();
  });

  it("shows wrong answers with hint, explanation, and the correct form", () => {
    const wrong = attempt({
      itemId: "d",
      source: "We want to cancel the meeting.",
      answer: "Quieromos cancelar la reunion.",
      status: "wrong",
      canonical: "Queremos cancelar la reunión.",
      errorCategory: "verb-form",
      hint: "Check the nosotros form of querer.",
      explanation: "The first-person plural of querer is queremos.",
    });
    render(
      <V2ReviewScreen attempts={[wrong]} sessionId="ses-1" go={vi.fn()} />,
    );
    expect(screen.getByText("0 of 1 correct")).toBeInTheDocument();
    expect(screen.getByText(/needs work/i)).toBeInTheDocument();
    expect(
      screen.getByText("Check the nosotros form of querer."),
    ).toBeInTheDocument();
    expect(
      screen.getByText("The first-person plural of querer is queremos."),
    ).toBeInTheDocument();
    expect(
      screen.getByText("Queremos cancelar la reunión."),
    ).toBeInTheDocument();
  });

  it("counts a structure dodge as correct and shows its nudge", () => {
    const dodge = attempt({
      itemId: "e",
      source: "I want to eat.",
      answer: "Me gustaría comer.",
      status: "dodge",
      remarks: [
        "Correct Spanish — but this one drills “Quiero + infinitive”. Try a version that uses it.",
      ],
      canonical: "Quiero comer.",
    });
    render(
      <V2ReviewScreen attempts={[dodge]} sessionId="ses-1" go={vi.fn()} />,
    );
    expect(screen.getByText("1 of 1 correct")).toBeInTheDocument();
    expect(screen.getByText(/Try a version that uses it/)).toBeInTheDocument();
    expect(screen.queryByText(/needs work/i)).not.toBeInTheDocument();
  });

  it("polls while attempts are pending and shows resolutions as they land", async () => {
    vi.useFakeTimers();
    vi.mocked(tauri.isTauri).mockReturnValue(true);
    const resolved = [
      ATTEMPTS[0],
      ATTEMPTS[1],
      attempt({
        ...ATTEMPTS[2],
        status: "wrong",
        errorCategory: "verb-form",
        hint: "Conjugate querer for yo.",
        explanation: "Querer must be conjugated: quiero.",
      }),
    ];
    vi.mocked(tauri.v2SessionReview).mockResolvedValue(resolved);

    render(
      <V2ReviewScreen attempts={ATTEMPTS} sessionId="ses-1" go={vi.fn()} />,
    );
    expect(screen.getByText("2 of 3 correct")).toBeInTheDocument();

    await act(async () => {
      await vi.advanceTimersByTimeAsync(2500);
    });
    expect(tauri.v2SessionReview).toHaveBeenCalledWith("ses-1");
    expect(screen.getByText(/needs work/i)).toBeInTheDocument();
    expect(screen.getByText("Conjugate querer for yo.")).toBeInTheDocument();
    expect(screen.queryByText(/awaiting evaluation/i)).not.toBeInTheDocument();

    // Nothing pending anymore: polling stops.
    vi.mocked(tauri.v2SessionReview).mockClear();
    await act(async () => {
      await vi.advanceTimersByTimeAsync(5000);
    });
    expect(tauri.v2SessionReview).not.toHaveBeenCalled();
    vi.useRealTimers();
  });

  it("Done returns home", () => {
    const go = vi.fn();
    render(<V2ReviewScreen attempts={ATTEMPTS} sessionId="ses-1" go={go} />);
    fireEvent.click(screen.getByText("Done"));
    expect(go).toHaveBeenCalledWith({ name: "home" });
  });
});
