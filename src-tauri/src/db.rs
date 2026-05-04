use rusqlite::{params, Connection};
use std::path::PathBuf;
use std::sync::Mutex;

const UNITS_SEED: &str = include_str!("units_seed.json");

pub struct Db(pub Mutex<Connection>);

impl Db {
    pub fn open(path: &PathBuf) -> rusqlite::Result<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        let conn = Connection::open(path)?;
        run_migrations(&conn)?;
        Ok(Self(Mutex::new(conn)))
    }
}

/// Exposed for tests in other modules.
#[cfg(test)]
pub fn run_migrations_for_test(conn: &Connection) -> rusqlite::Result<()> {
    run_migrations(conn)
}

fn run_migrations(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS attempt_log (
            id              TEXT    PRIMARY KEY,
            tag             TEXT    NOT NULL,
            item_id         TEXT    NOT NULL,
            correct         INTEGER NOT NULL,
            learner_answer  TEXT    NOT NULL DEFAULT '',
            timestamp       INTEGER NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_attempt_log_tag
            ON attempt_log(tag, timestamp);
        CREATE INDEX IF NOT EXISTS idx_attempt_log_item
            ON attempt_log(item_id, timestamp);

        CREATE TABLE IF NOT EXISTS exercise_items (
            id          TEXT    PRIMARY KEY,
            source      TEXT    NOT NULL,
            canonical   TEXT    NOT NULL,
            primary_tag TEXT    NOT NULL,
            stacked_tags TEXT   NOT NULL DEFAULT '[]',
            created_at  INTEGER NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_exercise_items_tag
            ON exercise_items(primary_tag);

        CREATE TABLE IF NOT EXISTS units (
            skill_tag        TEXT    PRIMARY KEY,
            title            TEXT    NOT NULL,
            phase            INTEGER NOT NULL,
            prerequisites    TEXT    NOT NULL DEFAULT '[]',
            unit_number      INTEGER,
            generation_state TEXT    NOT NULL DEFAULT 'idle'
        );
        CREATE INDEX IF NOT EXISTS idx_units_phase
            ON units(phase);

        CREATE TABLE IF NOT EXISTS vocab_words (
            lemma           TEXT    PRIMARY KEY,
            translation     TEXT    NOT NULL,
            frequency_rank  INTEGER NOT NULL,
            part_of_speech  TEXT    NOT NULL,
            pipeline_state  TEXT    NOT NULL DEFAULT 'untouched',
            next_review     INTEGER
        );
        ",
    )?;

    // v2: add columns to existing DBs that were created before this migration.
    // Errors are ignored — "duplicate column name" means the column already exists.
    let _ = conn.execute_batch("ALTER TABLE units ADD COLUMN unit_number INTEGER");
    let _ = conn.execute_batch(
        "ALTER TABLE units ADD COLUMN generation_state TEXT NOT NULL DEFAULT 'idle'",
    );

    // Create the ordering index only after unit_number is guaranteed to exist.
    conn.execute_batch(
        "CREATE INDEX IF NOT EXISTS idx_units_order ON units(unit_number)",
    )?;

    // v3: evaluation columns on attempt_log.
    let _ = conn.execute_batch("ALTER TABLE attempt_log ADD COLUMN session_id TEXT");
    let _ = conn.execute_batch(
        "ALTER TABLE attempt_log ADD COLUMN eval_state TEXT NOT NULL DEFAULT 'unevaluated'",
    );
    let _ = conn.execute_batch("ALTER TABLE attempt_log ADD COLUMN error_tag TEXT");
    let _ = conn.execute_batch(
        "ALTER TABLE attempt_log ADD COLUMN remarks TEXT NOT NULL DEFAULT '[]'",
    );
    let _ = conn.execute_batch("ALTER TABLE attempt_log ADD COLUMN explanation TEXT");

    // Unique index prevents re-inserting the same item within a session (retry safety).
    conn.execute_batch(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_attempt_log_session_item
             ON attempt_log(session_id, item_id)
             WHERE session_id IS NOT NULL",
    )?;

    seed_units(conn)
}

fn seed_units(conn: &Connection) -> rusqlite::Result<()> {
    #[derive(serde::Deserialize)]
    struct UnitRow {
        #[serde(rename = "skillTag")]
        skill_tag: String,
        title: String,
        phase: i64,
        prerequisites: Vec<String>,
    }

    let units: Vec<UnitRow> =
        serde_json::from_str(UNITS_SEED).expect("units_seed.json is valid JSON");

    for (idx, u) in units.iter().enumerate() {
        let prereqs_json = serde_json::to_string(&u.prerequisites).expect("prereqs serialize");
        let unit_number = (idx + 1) as i64;
        conn.execute(
            "INSERT INTO units (skill_tag, title, phase, prerequisites, unit_number, generation_state)
             VALUES (?1, ?2, ?3, ?4, ?5, 'idle')
             ON CONFLICT(skill_tag) DO UPDATE SET unit_number = excluded.unit_number",
            params![u.skill_tag, u.title, u.phase, prereqs_json, unit_number],
        )?;
    }

    Ok(())
}

/// Dev utility: wipe all generated exercise items and reset generation state.
#[tauri::command]
pub fn wipe_exercise_items(state: tauri::State<'_, Db>) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM exercise_items", [])
        .map_err(|e| e.to_string())?;
    conn.execute("UPDATE units SET generation_state = 'idle'", [])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn db_health(state: tauri::State<'_, Db>) -> Result<String, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let n: i64 = conn
        .query_row("SELECT 1", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    let unit_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM units", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    let attempt_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM attempt_log", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    Ok(format!(
        "ok: ping={}, units={}, attempts={}",
        n, unit_count, attempt_count
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn in_memory() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        run_migrations(&conn).unwrap();
        conn
    }

    #[test]
    fn migration_creates_all_tables() {
        let conn = in_memory();
        for table in &["attempt_log", "exercise_items", "units", "vocab_words"] {
            let count: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?1",
                    params![table],
                    |r| r.get(0),
                )
                .unwrap();
            assert_eq!(count, 1, "table {table} should exist");
        }
    }

    #[test]
    fn migration_is_idempotent() {
        let conn = in_memory();
        run_migrations(&conn).unwrap();
    }

    #[test]
    fn units_seeded_with_correct_count() {
        let conn = in_memory();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM units", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 203, "all 203 units must be seeded");
    }

    #[test]
    fn unit_seed_has_all_phases() {
        let conn = in_memory();
        let phase_count: i64 = conn
            .query_row("SELECT COUNT(DISTINCT phase) FROM units", [], |r| r.get(0))
            .unwrap();
        assert_eq!(phase_count, 43, "units must span phases 0 through 42");
    }

    #[test]
    fn unit_prerequisites_are_valid_json_arrays() {
        let conn = in_memory();
        let mut stmt = conn
            .prepare("SELECT skill_tag, prerequisites FROM units")
            .unwrap();
        let rows = stmt
            .query_map([], |r| {
                Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?))
            })
            .unwrap();
        for row in rows {
            let (tag, prereqs) = row.unwrap();
            let parsed: Result<Vec<String>, _> = serde_json::from_str(&prereqs);
            assert!(parsed.is_ok(), "unit {tag} has invalid prerequisites JSON");
        }
    }

    #[test]
    fn all_units_have_sequential_unit_numbers() {
        let conn = in_memory();
        let null_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM units WHERE unit_number IS NULL",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(null_count, 0, "all units must have a unit_number");
    }

    #[test]
    fn unit_numbers_are_unique() {
        let conn = in_memory();
        let distinct: i64 = conn
            .query_row("SELECT COUNT(DISTINCT unit_number) FROM units", [], |r| {
                r.get(0)
            })
            .unwrap();
        let total: i64 = conn
            .query_row("SELECT COUNT(*) FROM units", [], |r| r.get(0))
            .unwrap();
        assert_eq!(distinct, total, "unit_numbers must be unique");
    }

    #[test]
    fn attempt_log_has_eval_columns() {
        let conn = in_memory();
        // These columns must exist — inserting with them should succeed.
        conn.execute(
            "INSERT INTO attempt_log
             (id, tag, item_id, correct, learner_answer, timestamp,
              session_id, eval_state, error_tag, remarks, explanation)
             VALUES ('x', 't', 'i', 0, 'ans', 1,
                     'sid-1', 'unevaluated', NULL, '[]', NULL)",
            [],
        )
        .expect("attempt_log must have eval columns");
    }

    #[test]
    fn attempt_log_session_item_unique_index_rejects_duplicate() {
        let conn = in_memory();
        conn.execute(
            "INSERT INTO attempt_log
             (id, tag, item_id, correct, learner_answer, timestamp, session_id, eval_state)
             VALUES ('a1', 't', 'i1', 0, 'ans', 1, 'sess-1', 'unevaluated')",
            [],
        )
        .unwrap();
        let result = conn.execute(
            "INSERT OR IGNORE INTO attempt_log
             (id, tag, item_id, correct, learner_answer, timestamp, session_id, eval_state)
             VALUES ('a2', 't', 'i1', 0, 'ans2', 2, 'sess-1', 'unevaluated')",
            [],
        );
        assert!(result.is_ok());
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM attempt_log WHERE session_id='sess-1'", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 1, "duplicate (session_id, item_id) must be ignored");
    }

    #[test]
    fn units_default_generation_state_is_idle() {
        let conn = in_memory();
        let non_idle: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM units WHERE generation_state != 'idle'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(non_idle, 0, "all units must start with idle generation_state");
    }
}
