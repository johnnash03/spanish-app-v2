import { render, screen, fireEvent } from "@testing-library/react";
import { vi } from "vitest";
import { V2ReviewScreen } from "./ReviewScreen";
import type { V2ReviewAttempt } from "../../types";

const ATTEMPTS: V2ReviewAttempt[] = [
  {
    itemId: "a",
    source: "You can see them.",
    answer: "Los puedes ver.",
    status: "correct",
    remarks: [],
    canonical: "Puedes verlos.",
    targetSkill: "clitic.do",
  },
  {
    itemId: "b",
    source: "He wanted to eat.",
    answer: "Queria comer.",
    status: "correct",
    remarks: ["Correct — differs from “Quería comer.” only in accents."],
    canonical: "Quería comer.",
    targetSkill: "past.queria",
  },
  {
    itemId: "c",
    source: "I want to dance.",
    answer: "Yo querer bailar.",
    status: "pending",
    remarks: [],
    canonical: "Quiero bailar.",
    targetSkill: "opener.quiero",
  },
];

describe("V2ReviewScreen", () => {
  it("shows the correct count as the hero", () => {
    render(<V2ReviewScreen attempts={ATTEMPTS} go={vi.fn()} />);
    expect(screen.getByText("2 of 3 correct")).toBeInTheDocument();
  });

  it("shows deterministic accent remarks on correct items", () => {
    render(<V2ReviewScreen attempts={ATTEMPTS} go={vi.fn()} />);
    expect(
      screen.getByText(
        "Correct — differs from “Quería comer.” only in accents.",
      ),
    ).toBeInTheDocument();
  });

  it("lists unmatched answers as awaiting evaluation with the target", () => {
    render(<V2ReviewScreen attempts={ATTEMPTS} go={vi.fn()} />);
    expect(screen.getByText(/awaiting evaluation/i)).toBeInTheDocument();
    expect(screen.getByText("Yo querer bailar.")).toBeInTheDocument();
    expect(screen.getByText("Quiero bailar.")).toBeInTheDocument();
  });

  it("Done returns home", () => {
    const go = vi.fn();
    render(<V2ReviewScreen attempts={ATTEMPTS} go={go} />);
    fireEvent.click(screen.getByText("Done"));
    expect(go).toHaveBeenCalledWith({ name: "home" });
  });
});
