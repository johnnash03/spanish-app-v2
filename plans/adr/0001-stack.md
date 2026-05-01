# ADR 0001 — Foundation Stack

Status: accepted
Date: 2026-05-01
Issue: #1

## Context

Need to pick the desktop runtime, frontend framework, persistence layer, and LLM client for a single-user, local-first Spanish learning app.

## Decision

- **Tauri 2.x** as the desktop runtime. Native webview, small binary, Rust backend, idiomatic for local-first apps.
- **React 19 + TypeScript + Vite** for the frontend. Matches the existing JSX prototypes in `design_handoff/`.
- **pnpm** as the package manager.
- **`rusqlite` (bundled feature)** for SQLite, called from Rust-side Tauri commands. Database file lives in the platform app-data dir (`~/Library/Application Support/com.spanishapp.dev/spanish-app.db` on macOS).
- **`async-openai`** for the OpenAI client, called from a Tauri command. The API key is loaded from a `.env` file in `src-tauri/` via `dotenvy`.

## Alternatives considered

**Frontend runtime:** Electron rejected — larger binary, ships full Node, and we'd be paying for a runtime we don't need.

**SQLite library:**
- `tauri-plugin-sql` — frontend-driven; SQL strings ship from JS. Fine for prototypes, but session queue assembly, mastery derivation, and SRS scheduling all want to live in Rust against the DB. Would have outgrown it within 2–3 issues.
- `better-sqlite3` — Node-only; not loadable in a Tauri webview without a Node sidecar, which defeats the point of Tauri.
- `rusqlite` (chosen) — Rust-native, lets domain logic live close to the data with end-to-end serde types.

**OpenAI key handling:** OS keychain (via `keyring` crate) + in-app settings screen was the originally proposed end-user-grade design. Deferred for bootstrap. Current approach: plaintext `.env` read at startup. Insecure — the key sits in plaintext and is loaded into the process env. Acceptable for a single-developer local-first app pre-distribution. **Revisit before any external distribution** (issue TBD): move to OS keychain + in-app settings UI, drop the `.env` path.

## Consequences

- All persistence and LLM calls go through Tauri commands. Frontend never touches files, the DB, or the OpenAI API directly.
- Domain logic (mastery, queue assembly, SRS) will live in `src-tauri/src/` Rust modules, testable with `cargo test`.
- API key rotation requires editing `.env` and restarting the app. Acceptable until the settings screen is built.
