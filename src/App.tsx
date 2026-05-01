import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

type Status =
  | { kind: "pending" }
  | { kind: "ok"; detail: string }
  | { kind: "error"; detail: string };

const initial: Status = { kind: "pending" };

function statusIcon(s: Status): string {
  if (s.kind === "ok") return "✅";
  if (s.kind === "error") return "❌";
  return "…";
}

function statusText(s: Status): string {
  if (s.kind === "pending") return "checking…";
  return s.detail;
}

function App() {
  const [frontend] = useState<Status>({ kind: "ok", detail: "rendered" });
  const [sqlite, setSqlite] = useState<Status>(initial);
  const [openai, setOpenai] = useState<Status>(initial);

  useEffect(() => {
    invoke<string>("db_health")
      .then((detail) => setSqlite({ kind: "ok", detail }))
      .catch((e) => setSqlite({ kind: "error", detail: String(e) }));

    invoke<string>("openai_ping")
      .then((detail) => setOpenai({ kind: "ok", detail }))
      .catch((e) => setOpenai({ kind: "error", detail: String(e) }));
  }, []);

  return (
    <main className="container">
      <h1>Spanish App — stack check</h1>
      <ul style={{ listStyle: "none", padding: 0, textAlign: "left" }}>
        <li>
          {statusIcon(frontend)} <strong>Frontend:</strong> {statusText(frontend)}
        </li>
        <li>
          {statusIcon(sqlite)} <strong>SQLite:</strong> {statusText(sqlite)}
        </li>
        <li>
          {statusIcon(openai)} <strong>OpenAI:</strong> {statusText(openai)}
        </li>
      </ul>
    </main>
  );
}

export default App;
