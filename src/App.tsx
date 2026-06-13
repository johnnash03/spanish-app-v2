import { useState } from "react";
import "./App.css";
// The legacy (v1) screens are the default UI until the v2 home lands
// (S14, #45); they then demote to a Legacy menu entry. See PRD #31.
import { HomeScreen } from "./screens/legacy/HomeScreen";
import { UnitListScreen } from "./screens/legacy/UnitListScreen";
import { UnitDetailScreen } from "./screens/legacy/UnitDetailScreen";
import { SessionScreen } from "./screens/legacy/SessionScreen";
import { SessionReviewScreen } from "./screens/legacy/SessionReviewScreen";
import { PracticeEntryScreen } from "./screens/legacy/PracticeEntryScreen";
import { PracticeSessionScreen } from "./screens/legacy/PracticeSessionScreen";
import { PracticeReviewScreen } from "./screens/legacy/PracticeReviewScreen";
import { VocabIntakeScreen } from "./screens/legacy/VocabIntakeScreen";
import { VocabSessionScreen } from "./screens/legacy/VocabSessionScreen";
import { VocabReviewScreen } from "./screens/legacy/VocabReviewScreen";
import { CombinedSessionScreen } from "./screens/legacy/CombinedSessionScreen";
import { CombinedReviewScreen } from "./screens/legacy/CombinedReviewScreen";
// V2 practice loop (S6, #37): unit picker → one-at-a-time session →
// batched review. Reached via the temporary link on the legacy home.
import { V2UnitListScreen } from "./screens/v2/UnitListScreen";
import { V2SessionScreen } from "./screens/v2/SessionScreen";
import { V2ReviewScreen } from "./screens/v2/ReviewScreen";
import type { Screen } from "./types";

function App() {
  const [screen, setScreen] = useState<Screen>({ name: "home" });

  if (screen.name === "units") {
    return <UnitListScreen go={setScreen} />;
  }

  if (screen.name === "unitDetail") {
    return <UnitDetailScreen unitN={screen.unitN} go={setScreen} />;
  }

  if (screen.name === "session") {
    return <SessionScreen unitSkillTag={screen.unitSkillTag} go={setScreen} />;
  }

  if (screen.name === "sessionReview") {
    return <SessionReviewScreen attempts={screen.attempts} go={setScreen} />;
  }

  if (screen.name === "practiceEntry") {
    return <PracticeEntryScreen go={setScreen} />;
  }

  if (screen.name === "practiceSession") {
    return (
      <PracticeSessionScreen
        tagId={screen.tagId}
        tagName={screen.tagName}
        go={setScreen}
      />
    );
  }

  if (screen.name === "practiceReview") {
    return (
      <PracticeReviewScreen
        attempts={screen.attempts}
        practicedWeakTags={screen.practicedWeakTags}
        go={setScreen}
      />
    );
  }

  if (screen.name === "vocabIntake") {
    return <VocabIntakeScreen go={setScreen} />;
  }

  if (screen.name === "vocabSession") {
    return <VocabSessionScreen go={setScreen} />;
  }

  if (screen.name === "vocabReview") {
    return <VocabReviewScreen results={screen.results} go={setScreen} />;
  }

  if (screen.name === "combinedSession") {
    return <CombinedSessionScreen go={setScreen} />;
  }

  if (screen.name === "combinedReview") {
    return (
      <CombinedReviewScreen
        attempts={screen.attempts}
        vocabLemmasByItemId={screen.vocabLemmasByItemId}
        go={setScreen}
      />
    );
  }

  if (screen.name === "v2Units") {
    return <V2UnitListScreen go={setScreen} />;
  }

  if (screen.name === "v2Session") {
    return (
      <V2SessionScreen
        unitId={screen.unitId}
        unitTitle={screen.unitTitle}
        go={setScreen}
      />
    );
  }

  if (screen.name === "v2Review") {
    return (
      <V2ReviewScreen
        attempts={screen.attempts}
        sessionId={screen.sessionId}
        go={setScreen}
      />
    );
  }

  return <HomeScreen go={setScreen} />;
}

export default App;
