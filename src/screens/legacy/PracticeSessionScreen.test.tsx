import { render, screen } from "@testing-library/react";
import { vi, beforeEach } from "vitest";
import { PracticeSessionScreen } from "./PracticeSessionScreen";

vi.mock("../../lib/tauri", () => ({
  isTauri: () => true,
  assembleDpQueue: vi.fn().mockResolvedValue([
    {
      id: "item-1",
      source: "Yo ___ cansado.",
      primaryTag: "ser-estar",
      stackedTags: [],
    },
    {
      id: "item-2",
      source: "Ella ___ en casa.",
      primaryTag: "ser-estar",
      stackedTags: [],
    },
  ]),
}));

beforeEach(() => {
  vi.clearAllMocks();
});

describe("PracticeSessionScreen", () => {
  it("shows tag name in banner when a specific tagId is given", async () => {
    render(
      <PracticeSessionScreen
        tagId="ser-estar"
        tagName="Ser vs Estar"
        go={vi.fn()}
      />,
    );
    expect(await screen.findByText("Ser vs Estar")).toBeInTheDocument();
  });

  it("shows 'All weak skills' label when tagId is null", async () => {
    render(<PracticeSessionScreen tagId={null} tagName={null} go={vi.fn()} />);
    expect(await screen.findByText(/all weak skills/i)).toBeInTheDocument();
  });

  it("shows exercises from the queue", async () => {
    render(<PracticeSessionScreen tagId={null} tagName={null} go={vi.fn()} />);
    expect(await screen.findByText("Yo ___ cansado.")).toBeInTheDocument();
  });
});
