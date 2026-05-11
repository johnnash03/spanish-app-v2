import { render, screen, fireEvent } from "@testing-library/react";
import { vi } from "vitest";
import { HomeScreen } from "./HomeScreen";

vi.mock("../lib/tauri", () => ({
  isTauri: () => true,
  getCurrentUnitNumber: vi.fn().mockResolvedValue(1),
  getUnitByN: vi.fn().mockResolvedValue({ n: 1, name: "Present tense" }),
  getPendingSession: vi.fn(),
  getPipelineHealth: vi
    .fn()
    .mockResolvedValue({ activeCount: 9, band: "light" }),
}));

describe("HomeScreen – pending session banner", () => {
  it("does not show banner when no pending session", async () => {
    const { getPendingSession } = await import("../lib/tauri");
    vi.mocked(getPendingSession).mockResolvedValueOnce(null);

    render(<HomeScreen go={vi.fn()} />);

    await screen.findByText(/present tense/i);
    expect(screen.queryByText(/unsubmitted session/i)).not.toBeInTheDocument();
  });

  it("shows banner when pending session exists", async () => {
    const { getPendingSession } = await import("../lib/tauri");
    vi.mocked(getPendingSession).mockResolvedValueOnce([
      {
        itemId: "i1",
        tag: "ser-estar",
        learnerAnswer: "soy",
        source: "Yo ___ alto.",
      },
    ]);

    render(<HomeScreen go={vi.fn()} />);

    expect(await screen.findByText(/unsubmitted session/i)).toBeInTheDocument();
    expect(
      screen.getByRole("button", { name: /^review$/i }),
    ).toBeInTheDocument();
  });

  it("banner CTA navigates to sessionReview with pending attempts", async () => {
    const pendingAttempts = [
      {
        itemId: "i1",
        tag: "ser-estar",
        learnerAnswer: "soy",
        source: "Yo ___ alto.",
      },
    ];
    const { getPendingSession } = await import("../lib/tauri");
    vi.mocked(getPendingSession).mockResolvedValueOnce(pendingAttempts);

    const go = vi.fn();
    render(<HomeScreen go={go} />);

    await screen.findByText(/unsubmitted session/i);
    fireEvent.click(screen.getByRole("button", { name: /^review$/i }));

    expect(go).toHaveBeenCalledWith({
      name: "sessionReview",
      attempts: pendingAttempts,
    });
  });
});
