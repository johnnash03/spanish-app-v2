import { render, screen } from "@testing-library/react";
import { vi } from "vitest";
import { PracticeEntryScreen } from "./PracticeEntryScreen";

describe("PracticeEntryScreen", () => {
  it("renders each weak tag name", () => {
    render(<PracticeEntryScreen go={vi.fn()} />);
    expect(screen.getByText("Ser vs Estar")).toBeInTheDocument();
    expect(screen.getByText("Preterite irregulars")).toBeInTheDocument();
    expect(screen.getByText("Indirect object pronouns")).toBeInTheDocument();
  });

  it("renders a Practice all weak skills CTA", () => {
    render(<PracticeEntryScreen go={vi.fn()} />);
    expect(
      screen.getByRole("button", { name: /practice all weak skills/i }),
    ).toBeInTheDocument();
  });

  it("renders a per-tag Practice button for each weak tag", () => {
    render(<PracticeEntryScreen go={vi.fn()} />);
    expect(screen.getAllByRole("button", { name: /^practice$/i })).toHaveLength(
      3,
    );
  });

  it("navigates to practiceSession with null tagId when Practice all is clicked", async () => {
    const go = vi.fn();
    render(<PracticeEntryScreen go={go} />);
    screen.getByRole("button", { name: /practice all weak skills/i }).click();
    expect(go).toHaveBeenCalledWith({
      name: "practiceSession",
      tagId: null,
      tagName: null,
    });
  });

  it("navigates to practiceSession with tag data when per-tag Practice is clicked", async () => {
    const go = vi.fn();
    render(<PracticeEntryScreen go={go} />);
    const perTagButtons = screen.getAllByRole("button", {
      name: /^practice$/i,
    });
    perTagButtons[0].click();
    expect(go).toHaveBeenCalledWith({
      name: "practiceSession",
      tagId: "ser-estar",
      tagName: "Ser vs Estar",
    });
  });
});
