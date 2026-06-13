use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Mutex;

/// The v2 database. A separate SQLite file from the legacy v1 database —
/// the two coexist side by side and never share tables. V1 attempt history
/// is deliberately not migrated (see PRD #31, Foundation): it lives on as
/// committed fixtures, and v2 mastery starts clean.
pub struct DbV2(pub Mutex<Connection>);

impl DbV2 {
    pub fn open(path: &PathBuf) -> rusqlite::Result<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        let conn = Connection::open(path)?;
        run_migrations(&conn)?;
        Ok(Self(Mutex::new(conn)))
    }
}

pub fn run_migrations(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS meta (
            key   TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );
        INSERT OR IGNORE INTO meta (key, value) VALUES ('schema_version', '1');
        ",
    )?;

    let version: i64 = conn.query_row(
        "SELECT CAST(value AS INTEGER) FROM meta WHERE key = 'schema_version'",
        [],
        |row| row.get(0),
    )?;

    // v2 (S2, #33): curriculum storage. Licensing sets are stored, versioned,
    // inspectable; rows are rewritten from the validated load on every startup.
    if version < 2 {
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS curriculum_artifacts (
                kind    TEXT PRIMARY KEY,
                version INTEGER NOT NULL,
                json    TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS curriculum_units (
                id       TEXT PRIMARY KEY,
                position INTEGER NOT NULL,
                phase    INTEGER NOT NULL,
                title    TEXT NOT NULL,
                json     TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS licensing_sets (
                unit_id            TEXT PRIMARY KEY,
                curriculum_version INTEGER NOT NULL,
                ambient_version    INTEGER NOT NULL,
                json               TEXT NOT NULL
            );
            UPDATE meta SET value = '2' WHERE key = 'schema_version';
            ",
        )?;
    }

    // v3 (S5, #36): the generated exercise bank. Every row passed the
    // validator before insertion; items stream in as they pass. Generation
    // state lives per unit (idle/generating/ready/failed, v1 behavior).
    if version < 3 {
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS bank_items (
                id         TEXT PRIMARY KEY,
                unit_id    TEXT NOT NULL,
                source     TEXT NOT NULL,
                canonical  TEXT NOT NULL,
                variants   TEXT NOT NULL,
                slot_spec  TEXT NOT NULL,
                tags       TEXT NOT NULL,
                analysis   TEXT NOT NULL,
                created_at INTEGER NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_bank_items_unit ON bank_items(unit_id);
            CREATE TABLE IF NOT EXISTS unit_generation (
                unit_id    TEXT PRIMARY KEY,
                state      TEXT NOT NULL,
                updated_at INTEGER NOT NULL
            );
            UPDATE meta SET value = '3' WHERE key = 'schema_version';
            ",
        )?;
    }

    // v4 (S6, #37): the v2 attempt log — single source of truth for all
    // learner evidence; sessions are reconstructable from it. Attempts are
    // written eagerly, one per submitted item, with their Tier 0 verdict
    // ('correct') or 'pending' until the Tier 1 evaluator (S7) resolves
    // them.
    if version < 4 {
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS sessions (
                id         TEXT PRIMARY KEY,
                unit_id    TEXT NOT NULL,
                started_at INTEGER NOT NULL,
                ended_at   INTEGER
            );
            CREATE TABLE IF NOT EXISTS attempts (
                id           TEXT PRIMARY KEY,
                session_id   TEXT NOT NULL,
                item_id      TEXT NOT NULL,
                unit_id      TEXT NOT NULL,
                target_skill TEXT NOT NULL,
                source       TEXT NOT NULL,
                answer       TEXT NOT NULL,
                status       TEXT NOT NULL,
                tier         INTEGER,
                remarks      TEXT NOT NULL,
                created_at   INTEGER NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_attempts_session ON attempts(session_id);
            UPDATE meta SET value = '4' WHERE key = 'schema_version';
            ",
        )?;
    }

    // v5 (S7, #38): Tier 1 resolution lands on the attempt row. The full
    // decomposed analysis is kept for inspection (and the appeal flow,
    // S8); error fields hold the closed-enum classification with its
    // code-attributed, registry-validated skill tags.
    if version < 5 {
        conn.execute_batch(
            "
            ALTER TABLE attempts ADD COLUMN judgments TEXT;
            ALTER TABLE attempts ADD COLUMN error_category TEXT;
            ALTER TABLE attempts ADD COLUMN error_evidence TEXT;
            ALTER TABLE attempts ADD COLUMN error_skills TEXT;
            ALTER TABLE attempts ADD COLUMN hint TEXT;
            ALTER TABLE attempts ADD COLUMN explanation TEXT;
            UPDATE meta SET value = '5' WHERE key = 'schema_version';
            ",
        )?;
    }
    Ok(())
}

#[tauri::command]
pub fn db_v2_health(state: tauri::State<'_, DbV2>) -> Result<String, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let version: String = conn
        .query_row(
            "SELECT value FROM meta WHERE key = 'schema_version'",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;
    Ok(format!("ok: schema_version={}", version))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::params;

    fn in_memory() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        run_migrations(&conn).unwrap();
        conn
    }

    #[test]
    fn migration_creates_meta_table_with_schema_version() {
        let conn = in_memory();
        let version: String = conn
            .query_row(
                "SELECT value FROM meta WHERE key = 'schema_version'",
                params![],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(version, "5");
    }

    #[test]
    fn migration_v5_adds_tier1_resolution_columns_to_attempts() {
        let conn = in_memory();
        conn.execute(
            "INSERT INTO attempts
             (id, session_id, item_id, unit_id, target_skill, source, answer,
              status, tier, remarks, created_at, judgments, error_category,
              error_evidence, error_skills, hint, explanation)
             VALUES ('a', 's', 'i', 'u', 't', 'src', 'ans', 'wrong', 1, '[]', 0,
                     '{}', 'verb-form', 'quieromos', '[\"opener.quiero\"]',
                     'h', 'e')",
            params![],
        )
        .unwrap();
        let category: String = conn
            .query_row(
                "SELECT error_category FROM attempts WHERE id = 'a'",
                params![],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(category, "verb-form");
    }

    #[test]
    fn migration_v4_creates_attempt_log_tables() {
        let conn = in_memory();
        for table in ["sessions", "attempts"] {
            let count: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = ?1",
                    params![table],
                    |r| r.get(0),
                )
                .unwrap();
            assert_eq!(count, 1, "missing table {table}");
        }
    }

    #[test]
    fn migration_v3_creates_bank_tables() {
        let conn = in_memory();
        for table in ["bank_items", "unit_generation"] {
            let count: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = ?1",
                    params![table],
                    |r| r.get(0),
                )
                .unwrap();
            assert_eq!(count, 1, "missing table {table}");
        }
    }

    #[test]
    fn migration_v2_creates_curriculum_tables() {
        let conn = in_memory();
        for table in ["curriculum_artifacts", "curriculum_units", "licensing_sets"] {
            let count: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name = ?1",
                    params![table],
                    |r| r.get(0),
                )
                .unwrap();
            assert_eq!(count, 1, "missing table {table}");
        }
    }

    #[test]
    fn migration_is_idempotent() {
        let conn = in_memory();
        run_migrations(&conn).unwrap();
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM meta WHERE key = 'schema_version'",
                params![],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn open_creates_separate_file_from_v1() {
        let dir = std::env::temp_dir().join(format!("v2-db-test-{}", std::process::id()));
        let path = dir.join("spanish-app-v2.db");
        let _db = DbV2::open(&path).unwrap();
        assert!(path.exists(), "v2 db file must be created at its own path");
        std::fs::remove_dir_all(&dir).ok();
    }
}
