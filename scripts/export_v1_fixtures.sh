#!/usr/bin/env bash
# Export v1 data from the live legacy database into committed test fixtures
# (S1, issue #32). The v1 DB is read, never written.
#
# Usage: scripts/export_v1_fixtures.sh [path-to-v1-db]
set -euo pipefail

DB="${1:-$HOME/Library/Application Support/com.spanishapp.dev/spanish-app.db}"
OUT="$(cd "$(dirname "$0")/.." && pwd)/src-tauri/fixtures"

[ -f "$DB" ] || { echo "v1 db not found at: $DB" >&2; exit 1; }
mkdir -p "$OUT"

sqlite3 -readonly -json "$DB" \
  "SELECT id, source, canonical, primary_tag, stacked_tags, created_at, category
   FROM exercise_items ORDER BY created_at, id" \
  > "$OUT/v1_exercise_items.json"

sqlite3 -readonly -json "$DB" \
  "SELECT id, tag, item_id, correct, learner_answer, timestamp,
          session_id, eval_state, error_tag, remarks, explanation
   FROM attempt_log WHERE eval_state = 'evaluated' ORDER BY timestamp, id" \
  > "$OUT/v1_evaluations.json"

sqlite3 -readonly -json "$DB" \
  "SELECT id, source, canonical, grammar_tags, vocab_lemmas, created_at, served
   FROM combined_exercises ORDER BY created_at, id" \
  > "$OUT/v1_combined_exercises.json"

wc -c "$OUT"/v1_*.json
