import { render, screen, fireEvent } from "@testing-library/react";
import { vi, beforeEach } from "vitest";
import { VocabSessionScreen } from "./VocabSessionScreen";
import * as tauri from "../lib/tauri";

const MC_CARD = {
  lemma: "comer",
  translation: "to eat",
  frequencyRank: 142,
  partOfSpeech: "verb",
  pipelineState: "new",
  intervalDays: 1,
  repetitions: 0,
  selfRated: false,
  distractors: ["to drink", "to sleep", "to run"],
};

const SELF_RATED_CARD = {
  lemma: "hablar",
  translation: "to speak",
  frequencyRank: 98,
  partOfSpeech: "verb",
  pipelineState: "learning",
  intervalDays: 7,
  repetitions: 2,
  selfRated: true,
  distractors: [],
};

vi.mock("../lib/tauri", () => ({
  isTauri: () => true,
  getVocabSessionCards: vi.fn().mockResolvedValue([
    {
      lemma: "comer",
      translation: "to eat",
      frequencyRank: 142,
      partOfSpeech: "verb",
      pipelineState: "new",
      intervalDays: 1,
      repetitions: 0,
      selfRated: false,
      distractors: ["to drink", "to sleep", "to run"],
    },
  ]),
  recordVocabReview: vi.fn().mockResolvedValue(undefined),
}));

beforeEach(() => {
  vi.clearAllMocks();
  vi.mocked(tauri.getVocabSessionCards).mockResolvedValue([MC_CARD]);
  vi.mocked(tauri.recordVocabReview).mockResolvedValue(undefined);
});

describe("VocabSessionScreen", () => {
  describe("MC card", () => {
    it("shows the card lemma after loading", async () => {
      render(<VocabSessionScreen go={vi.fn()} />);
      expect(await screen.findByText("comer")).toBeInTheDocument();
    });

    it("shows all four MC options", async () => {
      render(<VocabSessionScreen go={vi.fn()} />);
      await screen.findByText("comer");
      expect(screen.getByText("to eat")).toBeInTheDocument();
      expect(screen.getByText("to drink")).toBeInTheDocument();
      expect(screen.getByText("to sleep")).toBeInTheDocument();
      expect(screen.getByText("to run")).toBeInTheDocument();
    });

    it("calls recordVocabReview correct=true for the right pick", async () => {
      render(<VocabSessionScreen go={vi.fn()} />);
      await screen.findByText("comer");
      fireEvent.click(screen.getByText("to eat"));
      expect(tauri.recordVocabReview).toHaveBeenCalledWith("comer", true);
    });

    it("calls recordVocabReview correct=false for a wrong pick", async () => {
      render(<VocabSessionScreen go={vi.fn()} />);
      await screen.findByText("comer");
      fireEvent.click(screen.getByText("to drink"));
      expect(tauri.recordVocabReview).toHaveBeenCalledWith("comer", false);
    });

    it("does not record a second answer after first pick", async () => {
      render(<VocabSessionScreen go={vi.fn()} />);
      await screen.findByText("comer");
      fireEvent.click(screen.getByText("to drink"));
      fireEvent.click(screen.getByText("to eat"));
      expect(tauri.recordVocabReview).toHaveBeenCalledTimes(1);
    });
  });

  describe("self-rated card", () => {
    beforeEach(() => {
      vi.mocked(tauri.getVocabSessionCards).mockResolvedValue([
        SELF_RATED_CARD,
      ]);
    });

    it("shows Reveal button", async () => {
      render(<VocabSessionScreen go={vi.fn()} />);
      await screen.findByText("hablar");
      expect(screen.getByText("Reveal")).toBeInTheDocument();
    });

    it("shows translation after reveal", async () => {
      render(<VocabSessionScreen go={vi.fn()} />);
      await screen.findByText("hablar");
      fireEvent.click(screen.getByText("Reveal"));
      expect(screen.getByText("to speak")).toBeInTheDocument();
    });

    it("shows Again / Good / Easy after reveal", async () => {
      render(<VocabSessionScreen go={vi.fn()} />);
      await screen.findByText("hablar");
      fireEvent.click(screen.getByText("Reveal"));
      expect(screen.getByText("Again")).toBeInTheDocument();
      expect(screen.getByText("Good")).toBeInTheDocument();
      expect(screen.getByText("Easy")).toBeInTheDocument();
    });

    it("records correct=false for Again", async () => {
      render(<VocabSessionScreen go={vi.fn()} />);
      await screen.findByText("hablar");
      fireEvent.click(screen.getByText("Reveal"));
      fireEvent.click(screen.getByText("Again"));
      expect(tauri.recordVocabReview).toHaveBeenCalledWith("hablar", false);
    });

    it("records correct=true for Good", async () => {
      render(<VocabSessionScreen go={vi.fn()} />);
      await screen.findByText("hablar");
      fireEvent.click(screen.getByText("Reveal"));
      fireEvent.click(screen.getByText("Good"));
      expect(tauri.recordVocabReview).toHaveBeenCalledWith("hablar", true);
    });
  });

  describe("end session", () => {
    it("shows End & review link after answering a card", async () => {
      render(<VocabSessionScreen go={vi.fn()} />);
      await screen.findByText("comer");
      fireEvent.click(screen.getByText("to eat"));
      expect(await screen.findByText(/end & review/i)).toBeInTheDocument();
    });

    it("navigates to vocabReview when End & review is tapped", async () => {
      const go = vi.fn();
      render(<VocabSessionScreen go={go} />);
      await screen.findByText("comer");
      fireEvent.click(screen.getByText("to eat"));
      fireEvent.click(await screen.findByText(/end & review/i));
      expect(go).toHaveBeenCalledWith(
        expect.objectContaining({ name: "vocabReview" }),
      );
    });

    it("vocabReview results contain the card outcome", async () => {
      const go = vi.fn();
      render(<VocabSessionScreen go={go} />);
      await screen.findByText("comer");
      fireEvent.click(screen.getByText("to eat"));
      fireEvent.click(await screen.findByText(/end & review/i));
      const call = go.mock.calls[0][0] as {
        name: string;
        results: { card: { lemma: string }; correct: boolean }[];
      };
      expect(call.results).toHaveLength(1);
      expect(call.results[0].card.lemma).toBe("comer");
      expect(call.results[0].correct).toBe(true);
    });
  });

  describe("empty state", () => {
    it("shows nothing-due message when no cards are returned", async () => {
      vi.mocked(tauri.getVocabSessionCards).mockResolvedValueOnce([]);
      render(<VocabSessionScreen go={vi.fn()} />);
      expect(await screen.findByText(/nothing due/i)).toBeInTheDocument();
    });
  });
});
