import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import { vi, beforeEach } from "vitest";
import { V2SessionScreen } from "./SessionScreen";
import * as tauri from "../../lib/tauri";

const ITEMS = [
  { id: "a", source: "You can see them.", targetSkill: "clitic.do" },
  { id: "b", source: "I want to eat.", targetSkill: "opener.quiero" },
];

vi.mock("../../lib/tauri", () => ({
  isTauri: () => true,
  v2StartSession: vi.fn(),
  v2SubmitAttempt: vi.fn(),
  v2EndSession: vi.fn(),
}));

beforeEach(() => {
  vi.clearAllMocks();
  vi.mocked(tauri.v2StartSession).mockResolvedValue({
    sessionId: "ses-1",
    items: ITEMS,
  });
  vi.mocked(tauri.v2SubmitAttempt).mockResolvedValue({
    attemptId: "att-1",
    itemId: "a",
    status: "correct",
    remarks: [],
  });
  vi.mocked(tauri.v2EndSession).mockResolvedValue([]);
});

function renderScreen(go = vi.fn()) {
  render(
    <V2SessionScreen unitId="opener.quiero" unitTitle="Quiero + inf" go={go} />,
  );
  return go;
}

async function typeAndEnter(text: string) {
  const input = await screen.findByPlaceholderText("Type the Spanish…");
  fireEvent.change(input, { target: { value: text } });
  fireEvent.keyDown(input, { key: "Enter" });
}

describe("V2SessionScreen", () => {
  it("shows one item at a time", async () => {
    renderScreen();
    expect(await screen.findByText("You can see them.")).toBeInTheDocument();
    expect(screen.queryByText("I want to eat.")).not.toBeInTheDocument();
  });

  it("Enter submits eagerly and advances to the next cue", async () => {
    renderScreen();
    await screen.findByText("You can see them.");
    await typeAndEnter("Los puedes ver.");
    expect(tauri.v2SubmitAttempt).toHaveBeenCalledWith(
      "ses-1",
      "a",
      "Los puedes ver.",
    );
    expect(await screen.findByText("I want to eat.")).toBeInTheDocument();
    expect(screen.queryByText("You can see them.")).not.toBeInTheDocument();
  });

  it("never shows a verdict mid-session", async () => {
    vi.mocked(tauri.v2SubmitAttempt).mockResolvedValue({
      attemptId: "att-1",
      itemId: "a",
      status: "correct",
      remarks: ["Correct — differs from “Puedes verlos.” only in accents."],
    });
    renderScreen();
    await screen.findByText("You can see them.");
    await typeAndEnter("puedes verlos");
    await screen.findByText("I want to eat.");
    expect(screen.queryByText(/differs from/)).not.toBeInTheDocument();
    expect(screen.queryByText(/✓/)).not.toBeInTheDocument();
  });

  it("Enter with an empty answer does not submit or advance", async () => {
    renderScreen();
    await screen.findByText("You can see them.");
    await typeAndEnter("   ");
    expect(tauri.v2SubmitAttempt).not.toHaveBeenCalled();
    expect(screen.getByText("You can see them.")).toBeInTheDocument();
  });

  it("counts attempted items in the top bar", async () => {
    renderScreen();
    await screen.findByText("You can see them.");
    expect(screen.getByText("0 attempted")).toBeInTheDocument();
    await typeAndEnter("Los puedes ver.");
    expect(await screen.findByText("1 attempted")).toBeInTheDocument();
  });

  it("Skip advances without submitting", async () => {
    renderScreen();
    await screen.findByText("You can see them.");
    fireEvent.click(screen.getByText("Skip"));
    expect(tauri.v2SubmitAttempt).not.toHaveBeenCalled();
    expect(await screen.findByText("I want to eat.")).toBeInTheDocument();
    expect(screen.getByText("0 attempted")).toBeInTheDocument();
  });

  it("End & review with zero attempts bounces home without ending", async () => {
    const go = renderScreen();
    await screen.findByText("You can see them.");
    fireEvent.click(screen.getByText("End & review"));
    await waitFor(() => expect(go).toHaveBeenCalledWith({ name: "home" }));
    expect(tauri.v2EndSession).not.toHaveBeenCalled();
  });

  it("End & review after attempts ends the session and opens the review", async () => {
    const reviewAttempts = [
      {
        itemId: "a",
        source: "You can see them.",
        answer: "Los puedes ver.",
        status: "correct" as const,
        remarks: [],
        canonical: "Puedes verlos.",
        targetSkill: "clitic.do",
        errorCategory: null,
        hint: null,
        explanation: null,
      },
    ];
    vi.mocked(tauri.v2EndSession).mockResolvedValue(reviewAttempts);
    const go = renderScreen();
    await screen.findByText("You can see them.");
    await typeAndEnter("Los puedes ver.");
    fireEvent.click(screen.getByText("End & review"));
    await waitFor(() =>
      expect(go).toHaveBeenCalledWith({
        name: "v2Review",
        attempts: reviewAttempts,
        sessionId: "ses-1",
      }),
    );
    expect(tauri.v2EndSession).toHaveBeenCalledWith("ses-1");
  });

  it("offers End & review after the queue is exhausted", async () => {
    renderScreen();
    await screen.findByText("You can see them.");
    await typeAndEnter("Los puedes ver.");
    await screen.findByText("I want to eat.");
    await typeAndEnter("Quiero comer.");
    expect(
      await screen.findByText(/every exercise in this unit/i),
    ).toBeInTheDocument();
    expect(screen.getByText("End & review")).toBeInTheDocument();
  });

  it("shows the empty state when the unit has no banked items", async () => {
    vi.mocked(tauri.v2StartSession).mockRejectedValue(
      new Error("unit has no banked items"),
    );
    renderScreen();
    expect(await screen.findByText(/no exercises ready/i)).toBeInTheDocument();
  });
});
