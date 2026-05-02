import { useState } from "react";
import "./App.css";
import { HomeScreen } from "./screens/HomeScreen";
import { UnitListScreen } from "./screens/UnitListScreen";
import { UnitDetailScreen } from "./screens/UnitDetailScreen";
import type { Screen } from "./types";

function App() {
  const [screen, setScreen] = useState<Screen>({ name: "home" });

  if (screen.name === "units") {
    return <UnitListScreen go={setScreen} />;
  }

  if (screen.name === "unitDetail") {
    return <UnitDetailScreen unitN={screen.unitN} go={setScreen} />;
  }

  return <HomeScreen go={setScreen} />;
}

export default App;
