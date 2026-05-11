use crate::db::Db;
use rusqlite::{params, Connection};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct VocabWord {
    pub lemma: String,
    pub translation: String,
    #[serde(rename = "frequencyRank")]
    pub frequency_rank: i64,
    #[serde(rename = "partOfSpeech")]
    pub part_of_speech: String,
}

#[derive(Debug, Serialize)]
pub struct PipelineHealth {
    #[serde(rename = "activeCount")]
    pub active_count: i64,
    pub band: String,
}

pub fn compute_band(active: i64) -> &'static str {
    match active {
        0..=10 => "light",
        11..=25 => "healthy",
        26..=40 => "full",
        _ => "overloaded",
    }
}

pub fn fetch_next_untouched(conn: &Connection, count: i64) -> rusqlite::Result<Vec<VocabWord>> {
    let mut stmt = conn.prepare(
        "SELECT lemma, translation, frequency_rank, part_of_speech
         FROM vocab_words
         WHERE pipeline_state = 'untouched'
         ORDER BY frequency_rank ASC
         LIMIT ?1",
    )?;
    let words = stmt
        .query_map(params![count], |row| {
            Ok(VocabWord {
                lemma: row.get(0)?,
                translation: row.get(1)?,
                frequency_rank: row.get(2)?,
                part_of_speech: row.get(3)?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(words)
}

pub fn promote_to_new(conn: &Connection, lemmas: &[String]) -> rusqlite::Result<()> {
    for lemma in lemmas {
        conn.execute(
            "UPDATE vocab_words SET pipeline_state = 'new'
             WHERE lemma = ?1 AND pipeline_state = 'untouched'",
            params![lemma],
        )?;
    }
    Ok(())
}

pub fn mark_word_mastered(conn: &Connection, lemma: &str) -> rusqlite::Result<bool> {
    let rows = conn.execute(
        "UPDATE vocab_words SET pipeline_state = 'mastered'
         WHERE lemma = ?1 AND pipeline_state != 'mastered'",
        params![lemma],
    )?;
    Ok(rows > 0)
}

pub fn fetch_pipeline_health(conn: &Connection) -> rusqlite::Result<PipelineHealth> {
    let active_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM vocab_words WHERE pipeline_state IN ('new', 'learning')",
        [],
        |r| r.get(0),
    )?;
    Ok(PipelineHealth {
        active_count,
        band: compute_band(active_count).to_string(),
    })
}

#[tauri::command]
pub fn get_next_untouched_words(
    db: tauri::State<'_, Db>,
    count: i64,
) -> Result<Vec<VocabWord>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    fetch_next_untouched(&conn, count).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn commit_intake_batch(
    db: tauri::State<'_, Db>,
    lemmas: Vec<String>,
) -> Result<(), String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    promote_to_new(&conn, &lemmas).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_pipeline_health(db: tauri::State<'_, Db>) -> Result<PipelineHealth, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    fetch_pipeline_health(&conn).map_err(|e| e.to_string())
}

/// Power-user: skip SRS and mark a word as mastered immediately.
/// Returns true if the word was updated, false if it was already mastered or not found.
#[tauri::command]
pub fn mark_vocab_word_mastered(
    db: tauri::State<'_, Db>,
    lemma: String,
) -> Result<bool, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    mark_word_mastered(&conn, &lemma).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        crate::db::run_migrations_for_test(&conn).unwrap();
        conn
    }

    fn insert_word(conn: &Connection, lemma: &str, rank: i64, state: &str) {
        conn.execute(
            "INSERT INTO vocab_words (lemma, translation, frequency_rank, part_of_speech, pipeline_state)
             VALUES (?1, 'test', ?2, 'noun', ?3)
             ON CONFLICT(lemma) DO UPDATE SET pipeline_state = ?3",
            params![lemma, rank, state],
        )
        .unwrap();
    }

    // ── compute_band ─────────────────────────────────────────────────────────

    #[test]
    fn band_light_at_zero() {
        assert_eq!(compute_band(0), "light");
    }

    #[test]
    fn band_light_at_ten() {
        assert_eq!(compute_band(10), "light");
    }

    #[test]
    fn band_healthy_at_eleven() {
        assert_eq!(compute_band(11), "healthy");
    }

    #[test]
    fn band_healthy_at_twenty_five() {
        assert_eq!(compute_band(25), "healthy");
    }

    #[test]
    fn band_full_at_twenty_six() {
        assert_eq!(compute_band(26), "full");
    }

    #[test]
    fn band_full_at_forty() {
        assert_eq!(compute_band(40), "full");
    }

    #[test]
    fn band_overloaded_at_forty_one() {
        assert_eq!(compute_band(41), "overloaded");
    }

    #[test]
    fn band_overloaded_at_large_count() {
        assert_eq!(compute_band(200), "overloaded");
    }

    // ── fetch_next_untouched ─────────────────────────────────────────────────

    #[test]
    fn returns_words_ordered_by_frequency_rank() {
        let conn = setup();
        // Promote all seed words so only our test words are untouched.
        conn.execute_batch("UPDATE vocab_words SET pipeline_state = 'new'")
            .unwrap();
        insert_word(&conn, "word_b", 20, "untouched");
        insert_word(&conn, "word_a", 5, "untouched");
        insert_word(&conn, "word_c", 50, "untouched");

        let words = fetch_next_untouched(&conn, 3).unwrap();
        assert_eq!(words[0].lemma, "word_a");
        assert_eq!(words[1].lemma, "word_b");
        assert_eq!(words[2].lemma, "word_c");
    }

    #[test]
    fn skips_non_untouched_words() {
        let conn = setup();
        insert_word(&conn, "new_word", 1, "new");
        insert_word(&conn, "learning_word", 2, "learning");
        insert_word(&conn, "mastered_word", 3, "mastered");
        insert_word(&conn, "untouched_word", 4, "untouched");

        let words = fetch_next_untouched(&conn, 10).unwrap();
        // Only the untouched word should appear (plus any from the seed)
        assert!(words.iter().all(|w| w.lemma != "new_word"));
        assert!(words.iter().all(|w| w.lemma != "learning_word"));
        assert!(words.iter().all(|w| w.lemma != "mastered_word"));
    }

    #[test]
    fn respects_count_limit() {
        let conn = setup();
        // Seed already has many untouched words; just check count limit
        let words = fetch_next_untouched(&conn, 5).unwrap();
        assert_eq!(words.len(), 5);
    }

    #[test]
    fn returns_empty_when_no_untouched_words() {
        let conn = setup();
        // Mark all as new (use a raw update on everything from seed)
        conn.execute_batch(
            "UPDATE vocab_words SET pipeline_state = 'new'",
        )
        .unwrap();
        let words = fetch_next_untouched(&conn, 5).unwrap();
        assert!(words.is_empty());
    }

    // ── promote_to_new ───────────────────────────────────────────────────────

    #[test]
    fn promotes_untouched_words_to_new() {
        let conn = setup();
        insert_word(&conn, "probar", 999, "untouched");

        promote_to_new(&conn, &["probar".to_string()]).unwrap();

        let state: String = conn
            .query_row(
                "SELECT pipeline_state FROM vocab_words WHERE lemma = 'probar'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(state, "new");
    }

    #[test]
    fn does_not_downgrade_non_untouched_words() {
        let conn = setup();
        insert_word(&conn, "already_learning", 999, "learning");

        promote_to_new(&conn, &["already_learning".to_string()]).unwrap();

        let state: String = conn
            .query_row(
                "SELECT pipeline_state FROM vocab_words WHERE lemma = 'already_learning'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(state, "learning");
    }

    #[test]
    fn promotes_multiple_words_atomically() {
        let conn = setup();
        insert_word(&conn, "alpha", 998, "untouched");
        insert_word(&conn, "beta", 999, "untouched");

        promote_to_new(&conn, &["alpha".to_string(), "beta".to_string()]).unwrap();

        for lemma in &["alpha", "beta"] {
            let state: String = conn
                .query_row(
                    "SELECT pipeline_state FROM vocab_words WHERE lemma = ?1",
                    params![lemma],
                    |r| r.get(0),
                )
                .unwrap();
            assert_eq!(state, "new");
        }
    }

    // ── fetch_pipeline_health ─────────────────────────────────────────────────

    #[test]
    fn pipeline_health_counts_new_and_learning() {
        let conn = setup();
        insert_word(&conn, "w1", 901, "new");
        insert_word(&conn, "w2", 902, "new");
        insert_word(&conn, "w3", 903, "learning");
        insert_word(&conn, "w4", 904, "mastered");
        insert_word(&conn, "w5", 905, "untouched");

        let health = fetch_pipeline_health(&conn).unwrap();
        // 2 new + 1 learning = 3 active
        assert_eq!(health.active_count, 3);
        assert_eq!(health.band, "light");
    }

    #[test]
    fn pipeline_health_excludes_mastered_and_untouched() {
        let conn = setup();
        // All words in seed start as untouched; mark some as mastered
        conn.execute_batch(
            "UPDATE vocab_words SET pipeline_state = 'mastered' WHERE frequency_rank <= 5",
        )
        .unwrap();

        let health = fetch_pipeline_health(&conn).unwrap();
        assert_eq!(health.active_count, 0);
        assert_eq!(health.band, "light");
    }

    // ── mark_word_mastered ────────────────────────────────────────────────────

    #[test]
    fn marks_untouched_word_as_mastered() {
        let conn = setup();
        insert_word(&conn, "correr", 500, "untouched");
        let changed = mark_word_mastered(&conn, "correr").unwrap();
        assert!(changed);
        let state: String = conn
            .query_row(
                "SELECT pipeline_state FROM vocab_words WHERE lemma = 'correr'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(state, "mastered");
    }

    #[test]
    fn marks_learning_word_as_mastered() {
        let conn = setup();
        insert_word(&conn, "beber", 501, "learning");
        let changed = mark_word_mastered(&conn, "beber").unwrap();
        assert!(changed);
        let state: String = conn
            .query_row(
                "SELECT pipeline_state FROM vocab_words WHERE lemma = 'beber'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(state, "mastered");
    }

    #[test]
    fn already_mastered_word_returns_false() {
        let conn = setup();
        insert_word(&conn, "vivir", 502, "mastered");
        let changed = mark_word_mastered(&conn, "vivir").unwrap();
        assert!(!changed);
    }

    #[test]
    fn unknown_lemma_returns_false() {
        let conn = setup();
        let changed = mark_word_mastered(&conn, "nonexistent_word_xyz").unwrap();
        assert!(!changed);
    }
}
