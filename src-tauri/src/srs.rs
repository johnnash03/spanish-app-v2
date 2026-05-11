use crate::db::Db;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

/// Days of interval at which a word is considered mature (mastered).
pub const MATURITY_THRESHOLD_DAYS: i64 = 21;

/// Default SM-2 ease factor for new cards.
pub const DEFAULT_EASE_FACTOR: f64 = 2.5;

/// Seconds per day constant.
const SECS_PER_DAY: i64 = 86_400;

#[derive(Debug, PartialEq)]
pub struct Sm2Result {
    pub repetitions: i64,
    pub interval_days: i64,
    pub ease_factor: f64,
}

/// Core SM-2 scheduling function.
///
/// `quality`: 0 (wrong) or 4 (correct). We use binary recall here — multiple
/// choice / self-rated both collapse to a two-valued signal for v1 simplicity.
///
/// Returns updated (repetitions, interval_days, ease_factor).
pub fn sm2_schedule(
    repetitions: i64,
    interval_days: i64,
    ease_factor: f64,
    correct: bool,
) -> Sm2Result {
    let quality: f64 = if correct { 4.0 } else { 0.0 };

    if correct {
        let new_interval = match repetitions {
            0 => 1,
            1 => 6,
            _ => {
                let next = (interval_days as f64 * ease_factor).round() as i64;
                next.max(1)
            }
        };
        let new_ef = (ease_factor + 0.1 - (5.0 - quality) * (0.08 + (5.0 - quality) * 0.02))
            .max(1.3);
        Sm2Result {
            repetitions: repetitions + 1,
            interval_days: new_interval,
            ease_factor: new_ef,
        }
    } else {
        // Forgot — reset streak, re-review tomorrow.
        Sm2Result {
            repetitions: 0,
            interval_days: 1,
            ease_factor: (ease_factor - 0.2).max(1.3),
        }
    }
}

pub fn is_mature(interval_days: i64) -> bool {
    interval_days >= MATURITY_THRESHOLD_DAYS
}

// ── DB helpers ────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct SrsCard {
    pub lemma: String,
    pub translation: String,
    #[serde(rename = "frequencyRank")]
    pub frequency_rank: i64,
    #[serde(rename = "partOfSpeech")]
    pub part_of_speech: String,
    #[serde(rename = "pipelineState")]
    pub pipeline_state: String,
    #[serde(rename = "nextReview")]
    pub next_review: Option<i64>,
    #[serde(rename = "intervalDays")]
    pub interval_days: i64,
    pub repetitions: i64,
    #[serde(rename = "easeFactor")]
    pub ease_factor: f64,
}

#[derive(Debug, Serialize)]
pub struct ReviewResult {
    pub lemma: String,
    #[serde(rename = "newPipelineState")]
    pub new_pipeline_state: String,
    #[serde(rename = "newIntervalDays")]
    pub new_interval_days: i64,
}

/// Fetch words due for SRS review (pipeline_state IN ('new','learning') and
/// next_review <= now, or next_review IS NULL). Returns up to `limit` cards.
pub fn fetch_due_cards(conn: &Connection, now_secs: i64, limit: i64) -> rusqlite::Result<Vec<SrsCard>> {
    let mut stmt = conn.prepare(
        "SELECT lemma, translation, frequency_rank, part_of_speech, pipeline_state,
                next_review, srs_interval_days, srs_repetitions, srs_ease_factor
         FROM vocab_words
         WHERE pipeline_state IN ('new', 'learning')
           AND (next_review IS NULL OR next_review <= ?1)
         ORDER BY COALESCE(next_review, 0) ASC
         LIMIT ?2",
    )?;
    let cards = stmt
        .query_map(params![now_secs, limit], |row| {
            Ok(SrsCard {
                lemma: row.get(0)?,
                translation: row.get(1)?,
                frequency_rank: row.get(2)?,
                part_of_speech: row.get(3)?,
                pipeline_state: row.get(4)?,
                next_review: row.get(5)?,
                interval_days: row.get(6)?,
                repetitions: row.get(7)?,
                ease_factor: row.get(8)?,
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(cards)
}

/// Interval threshold (days) above which a card uses self-rated recall instead of MC.
pub const SELF_RATED_THRESHOLD_DAYS: i64 = 7;

#[derive(Debug, Serialize)]
pub struct SrsCardWithDistractors {
    pub lemma: String,
    pub translation: String,
    #[serde(rename = "frequencyRank")]
    pub frequency_rank: i64,
    #[serde(rename = "partOfSpeech")]
    pub part_of_speech: String,
    #[serde(rename = "pipelineState")]
    pub pipeline_state: String,
    #[serde(rename = "intervalDays")]
    pub interval_days: i64,
    pub repetitions: i64,
    /// Self-rated recall (true) vs multiple choice (false).
    #[serde(rename = "selfRated")]
    pub self_rated: bool,
    /// Wrong translations for MC mode (3 items). Empty for self-rated cards.
    pub distractors: Vec<String>,
}

pub fn fetch_session_cards(
    conn: &Connection,
    now_secs: i64,
    limit: i64,
) -> rusqlite::Result<Vec<SrsCardWithDistractors>> {
    let cards = fetch_due_cards(conn, now_secs, limit)?;

    let mut result = Vec::with_capacity(cards.len());
    for card in cards {
        let self_rated = card.interval_days >= SELF_RATED_THRESHOLD_DAYS;
        let distractors = if self_rated {
            vec![]
        } else {
            fetch_distractors(conn, &card.lemma, 3)?
        };
        result.push(SrsCardWithDistractors {
            lemma: card.lemma,
            translation: card.translation,
            frequency_rank: card.frequency_rank,
            part_of_speech: card.part_of_speech,
            pipeline_state: card.pipeline_state,
            interval_days: card.interval_days,
            repetitions: card.repetitions,
            self_rated,
            distractors,
        });
    }
    Ok(result)
}

fn fetch_distractors(
    conn: &Connection,
    exclude_lemma: &str,
    count: usize,
) -> rusqlite::Result<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT translation FROM vocab_words
         WHERE lemma != ?1
         ORDER BY RANDOM()
         LIMIT ?2",
    )?;
    let translations = stmt
        .query_map(params![exclude_lemma, count as i64], |row| row.get(0))?
        .collect::<rusqlite::Result<Vec<String>>>()?;
    Ok(translations)
}

/// Record a vocab review: update SRS state, advance pipeline_state if appropriate.
pub fn record_review(
    conn: &Connection,
    lemma: &str,
    correct: bool,
    now_secs: i64,
) -> rusqlite::Result<ReviewResult> {
    let (current_state, repetitions, interval_days, ease_factor): (String, i64, i64, f64) =
        conn.query_row(
            "SELECT pipeline_state, srs_repetitions, srs_interval_days, srs_ease_factor
             FROM vocab_words WHERE lemma = ?1",
            params![lemma],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
        )?;

    let result = sm2_schedule(repetitions, interval_days, ease_factor, correct);
    let next_review = now_secs + result.interval_days * SECS_PER_DAY;

    // Advance pipeline state.
    let new_state = if current_state == "new" {
        // First review (any result) moves word into learning.
        "learning".to_string()
    } else if is_mature(result.interval_days) {
        "mastered".to_string()
    } else {
        current_state
    };

    conn.execute(
        "UPDATE vocab_words
         SET srs_repetitions = ?1,
             srs_interval_days = ?2,
             srs_ease_factor = ?3,
             next_review = ?4,
             pipeline_state = ?5
         WHERE lemma = ?6",
        params![
            result.repetitions,
            result.interval_days,
            result.ease_factor,
            next_review,
            new_state,
            lemma,
        ],
    )?;

    Ok(ReviewResult {
        lemma: lemma.to_string(),
        new_pipeline_state: new_state,
        new_interval_days: result.interval_days,
    })
}

// ── Tauri commands ────────────────────────────────────────────────────────────

#[tauri::command]
pub fn get_due_vocab_cards(
    db: tauri::State<'_, Db>,
    limit: i64,
) -> Result<Vec<SrsCard>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let now = now_secs();
    fetch_due_cards(&conn, now, limit).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_vocab_session_cards(
    db: tauri::State<'_, Db>,
    limit: i64,
) -> Result<Vec<SrsCardWithDistractors>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let now = now_secs();
    fetch_session_cards(&conn, now, limit).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn record_vocab_review(
    db: tauri::State<'_, Db>,
    lemma: String,
    correct: bool,
) -> Result<ReviewResult, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let now = now_secs();
    record_review(&conn, &lemma, correct, now).map_err(|e| e.to_string())
}

fn now_secs() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        crate::db::run_migrations_for_test(&conn).unwrap();
        conn
    }

    fn insert_word(conn: &Connection, lemma: &str, state: &str) {
        conn.execute(
            "INSERT INTO vocab_words (lemma, translation, frequency_rank, part_of_speech, pipeline_state)
             VALUES (?1, 'test', 999, 'noun', ?2)
             ON CONFLICT(lemma) DO UPDATE SET pipeline_state = ?2,
               srs_repetitions = 0, srs_interval_days = 1, srs_ease_factor = 2.5,
               next_review = NULL",
            params![lemma, state],
        )
        .unwrap();
    }

    // ── sm2_schedule ──────────────────────────────────────────────────────────

    #[test]
    fn first_correct_gives_interval_one() {
        let r = sm2_schedule(0, 1, DEFAULT_EASE_FACTOR, true);
        assert_eq!(r.repetitions, 1);
        assert_eq!(r.interval_days, 1);
    }

    #[test]
    fn second_correct_gives_interval_six() {
        let r = sm2_schedule(1, 1, DEFAULT_EASE_FACTOR, true);
        assert_eq!(r.repetitions, 2);
        assert_eq!(r.interval_days, 6);
    }

    #[test]
    fn third_correct_multiplies_by_ef() {
        let r = sm2_schedule(2, 6, DEFAULT_EASE_FACTOR, true);
        assert_eq!(r.repetitions, 3);
        // 6 * 2.5 = 15 days
        assert_eq!(r.interval_days, 15);
    }

    #[test]
    fn fourth_correct_crosses_maturity_threshold() {
        let r = sm2_schedule(3, 15, DEFAULT_EASE_FACTOR, true);
        assert_eq!(r.repetitions, 4);
        // 15 * 2.5 = 37 days — well past maturity threshold
        assert!(r.interval_days >= MATURITY_THRESHOLD_DAYS);
    }

    #[test]
    fn wrong_answer_resets_streak() {
        let r = sm2_schedule(3, 15, DEFAULT_EASE_FACTOR, false);
        assert_eq!(r.repetitions, 0);
        assert_eq!(r.interval_days, 1);
    }

    #[test]
    fn wrong_answer_decreases_ease_factor() {
        let r = sm2_schedule(3, 15, DEFAULT_EASE_FACTOR, false);
        assert!(r.ease_factor < DEFAULT_EASE_FACTOR);
    }

    #[test]
    fn ease_factor_never_drops_below_1_3() {
        // Drive EF down with many wrong answers.
        let mut ef = DEFAULT_EASE_FACTOR;
        for _ in 0..20 {
            let r = sm2_schedule(0, 1, ef, false);
            ef = r.ease_factor;
        }
        assert!(ef >= 1.3 - f64::EPSILON);
    }

    #[test]
    fn correct_answer_does_not_decrease_ef() {
        // With binary quality=4, SM-2 nets zero EF change. EF should not fall further.
        let r = sm2_schedule(0, 1, 1.3, true);
        assert!(r.ease_factor >= 1.3 - f64::EPSILON);
    }

    // ── is_mature ─────────────────────────────────────────────────────────────

    #[test]
    fn not_mature_below_threshold() {
        assert!(!is_mature(MATURITY_THRESHOLD_DAYS - 1));
    }

    #[test]
    fn mature_at_threshold() {
        assert!(is_mature(MATURITY_THRESHOLD_DAYS));
    }

    #[test]
    fn mature_above_threshold() {
        assert!(is_mature(MATURITY_THRESHOLD_DAYS + 10));
    }

    // ── fetch_due_cards ───────────────────────────────────────────────────────

    #[test]
    fn new_word_with_null_next_review_is_due() {
        let conn = setup();
        insert_word(&conn, "comer", "new");

        let cards = fetch_due_cards(&conn, 1_000_000, 10).unwrap();
        assert!(cards.iter().any(|c| c.lemma == "comer"));
    }

    #[test]
    fn word_not_yet_due_is_excluded() {
        let conn = setup();
        insert_word(&conn, "salir", "learning");
        // Set next_review far in the future.
        conn.execute(
            "UPDATE vocab_words SET next_review = 9999999999 WHERE lemma = 'salir'",
            [],
        )
        .unwrap();

        let cards = fetch_due_cards(&conn, 1_000_000, 10).unwrap();
        assert!(!cards.iter().any(|c| c.lemma == "salir"));
    }

    #[test]
    fn mastered_words_are_excluded() {
        let conn = setup();
        insert_word(&conn, "hacer", "mastered");

        let cards = fetch_due_cards(&conn, 9_999_999_999, 10).unwrap();
        assert!(!cards.iter().any(|c| c.lemma == "hacer"));
    }

    #[test]
    fn limit_is_respected() {
        let conn = setup();
        // Seed already has untouched words — promote a few to 'new' with null next_review.
        conn.execute(
            "UPDATE vocab_words SET pipeline_state = 'new', next_review = NULL
             WHERE frequency_rank <= 5",
            [],
        )
        .unwrap();

        let cards = fetch_due_cards(&conn, 9_999_999_999, 3).unwrap();
        assert_eq!(cards.len(), 3);
    }

    // ── record_review ─────────────────────────────────────────────────────────

    #[test]
    fn correct_review_of_new_word_transitions_to_learning() {
        let conn = setup();
        insert_word(&conn, "venir", "new");

        let result = record_review(&conn, "venir", true, 1_000_000).unwrap();
        assert_eq!(result.new_pipeline_state, "learning");
    }

    #[test]
    fn wrong_review_of_new_word_also_transitions_to_learning() {
        let conn = setup();
        insert_word(&conn, "traer", "new");

        let result = record_review(&conn, "traer", false, 1_000_000).unwrap();
        assert_eq!(result.new_pipeline_state, "learning");
    }

    #[test]
    fn review_updates_next_review_timestamp() {
        let conn = setup();
        insert_word(&conn, "poder", "new");

        let now = 1_000_000_i64;
        record_review(&conn, "poder", true, now).unwrap();

        let next_review: i64 = conn
            .query_row(
                "SELECT next_review FROM vocab_words WHERE lemma = 'poder'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert!(next_review > now, "next_review should be in the future");
    }

    #[test]
    fn mature_interval_transitions_to_mastered() {
        let conn = setup();
        insert_word(&conn, "querer", "learning");
        // Set up state so the next correct review pushes interval past maturity threshold.
        // After 3 repetitions with interval=15, next = round(15 * 2.5) = 37 days >= 21.
        conn.execute(
            "UPDATE vocab_words SET srs_repetitions = 3, srs_interval_days = 15, srs_ease_factor = 2.5
             WHERE lemma = 'querer'",
            [],
        )
        .unwrap();

        let result = record_review(&conn, "querer", true, 1_000_000).unwrap();
        assert_eq!(result.new_pipeline_state, "mastered");
        assert!(result.new_interval_days >= MATURITY_THRESHOLD_DAYS);
    }

    #[test]
    fn wrong_review_of_learning_word_stays_learning() {
        let conn = setup();
        insert_word(&conn, "saber", "learning");
        conn.execute(
            "UPDATE vocab_words SET srs_repetitions = 2, srs_interval_days = 6
             WHERE lemma = 'saber'",
            [],
        )
        .unwrap();

        let result = record_review(&conn, "saber", false, 1_000_000).unwrap();
        assert_eq!(result.new_pipeline_state, "learning");
        assert_eq!(result.new_interval_days, 1);
    }

    // ── fetch_session_cards ───────────────────────────────────────────────────

    #[test]
    fn mc_card_has_three_distractors() {
        let conn = setup();
        insert_word(&conn, "dormir", "new");
        // interval_days defaults to 1 (< SELF_RATED_THRESHOLD_DAYS) → MC mode.

        let cards = fetch_session_cards(&conn, 9_999_999_999, 10).unwrap();
        let card = cards.iter().find(|c| c.lemma == "dormir").unwrap();
        assert!(!card.self_rated);
        assert_eq!(card.distractors.len(), 3);
    }

    #[test]
    fn mc_distractors_do_not_include_correct_translation() {
        let conn = setup();
        insert_word(&conn, "dormir", "new");
        // Overwrite seed translation to something unique.
        conn.execute(
            "UPDATE vocab_words SET translation = 'to sleep' WHERE lemma = 'dormir'",
            [],
        )
        .unwrap();

        let cards = fetch_session_cards(&conn, 9_999_999_999, 10).unwrap();
        let card = cards.iter().find(|c| c.lemma == "dormir").unwrap();
        assert!(!card.distractors.contains(&"to sleep".to_string()));
    }

    #[test]
    fn high_interval_card_is_self_rated_with_no_distractors() {
        let conn = setup();
        insert_word(&conn, "llegar", "learning");
        conn.execute(
            "UPDATE vocab_words SET srs_interval_days = 7, next_review = NULL
             WHERE lemma = 'llegar'",
            [],
        )
        .unwrap();

        let cards = fetch_session_cards(&conn, 9_999_999_999, 10).unwrap();
        let card = cards.iter().find(|c| c.lemma == "llegar").unwrap();
        assert!(card.self_rated);
        assert!(card.distractors.is_empty());
    }
}
