import { useState } from "react";
import "./App.css";
import { HomeScreen } from "./screens/HomeScreen";
import { UnitListScreen } from "./screens/UnitListScreen";
import { UnitDetailScreen } from "./screens/UnitDetailScreen";
import { SessionScreen } from "./screens/SessionScreen";
import { SessionReviewScreen } from "./screens/SessionReviewScreen";
import { PracticeEntryScreen } from "./screens/PracticeEntryScreen";
import { PracticeSessionScreen } from "./screens/PracticeSessionScreen";
import { PracticeReviewScreen } from "./screens/PracticeReviewScreen";
import { VocabIntakeScreen } from "./screens/VocabIntakeScreen";
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

  return <HomeScreen go={setScreen} />;
}

export default App;
