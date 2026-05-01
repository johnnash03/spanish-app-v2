use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Mutex;

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

fn run_migrations(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS attempt_log (
            id TEXT PRIMARY KEY,
            tag TEXT NOT NULL,
            item_id TEXT NOT NULL,
            correct INTEGER NOT NULL,
            learner_answer TEXT,
            timestamp INTEGER NOT NULL
        );",
    )
}

#[tauri::command]
pub fn db_health(state: tauri::State<'_, Db>) -> Result<String, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let n: i64 = conn
        .query_row("SELECT 1", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    let count: i64 = conn
        .query_row("SELECT COUNT(*) FROM attempt_log", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    Ok(format!("SELECT 1 = {}, attempt_log rows = {}", n, count))
}
