use crate::db::Db;
use rusqlite::{params, Connection};
use serde::Serialize;

const WINDOW: usize = 20;
const THRESHOLD: usize = 16; // ≥16/20 correct = ≥80%

/// Derive unit completion status from attempt_log for a single tag.
/// Rules (per mastery-threshold.md):
///   - "complete"     : the rolling window of last 20 was ever ≥80% correct
///                      (once complete, never re-locks)
///   - "in-progress"  : at least one attempt exists, but never reached threshold
///   - "not-started"  : no attempts for this tag
pub fn derive_unit_status(attempts: &[bool]) -> &'static str {
    if attempts.is_empty() {
        return "not-started";
    }
    // Scan every window of WINDOW attempts in chronological order.
    // As soon as one window hits threshold, the unit is permanently complete.
    if attempts.len() >= WINDOW {
        for start in 0..=(attempts.len() - WINDOW) {
            let window = &attempts[start..start + WINDOW];
            let correct = window.iter().filter(|&&c| c).count();
            if correct >= THRESHOLD {
                return "complete";
            }
        }
    }
    "in-progress"
}

/// Fetch ordered attempt correctness for a tag from the DB.
pub fn fetch_attempts(conn: &Connection, tag: &str) -> rusqlite::Result<Vec<bool>> {
    let mut stmt = conn.prepare_cached(
        "SELECT correct FROM attempt_log WHERE tag = ?1 ORDER BY timestamp ASC",
    )?;
    let rows = stmt.query_map(params![tag], |r| r.get::<_, i64>(0))?;
    rows.map(|r| r.map(|v| v != 0)).collect()
}

/// True if the tag is currently mastered (last 20 ≥ 80% correct).
/// Used for prerequisite warnings — does NOT apply the never-re-lock rule.
pub fn is_currently_mastered(attempts: &[bool]) -> bool {
    if attempts.len() < WINDOW {
        return false;
    }
    let last20 = &attempts[attempts.len() - WINDOW..];
    last20.iter().filter(|&&c| c).count() >= THRESHOLD
}

#[derive(Debug, Serialize)]
pub struct WeakTag {
    pub id: String,
    pub name: String,
    #[serde(rename = "wrongOf20")]
    pub wrong_of_20: u32,
}

/// Return all tags that have attempts but have not currently mastered
/// (last ≤20 window has at least one wrong), ordered by wrong_of_20 desc.
/// The `tag_name_map` maps skill_tag → human title.
pub fn compute_weak_tags(
    conn: &Connection,
    tag_name_map: &std::collections::HashMap<String, String>,
) -> rusqlite::Result<Vec<WeakTag>> {
    // Fetch the last 20 attempts for every tag that appears in attempt_log.
    let mut stmt = conn.prepare(
        "SELECT tag, correct
         FROM attempt_log
         WHERE tag IN (SELECT DISTINCT tag FROM attempt_log)
         ORDER BY tag ASC, timestamp ASC",
    )?;

    let rows: Vec<(String, bool)> = stmt
        .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?)))?
        .filter_map(|r| r.ok())
        .map(|(tag, c)| (tag, c != 0))
        .collect();

    // Group by tag.
    let mut by_tag: std::collections::HashMap<String, Vec<bool>> =
        std::collections::HashMap::new();
    for (tag, correct) in rows {
        by_tag.entry(tag).or_default().push(correct);
    }

    let mut weak: Vec<WeakTag> = by_tag
        .into_iter()
        .filter_map(|(tag, attempts)| {
            if attempts.is_empty() {
                return None;
            }
            let last20_start = attempts.len().saturating_sub(WINDOW);
            let last20 = &attempts[last20_start..];
            let wrong = last20.iter().filter(|&&c| !c).count();
            if wrong == 0 {
                return None; // perfectly correct — not weak
            }
            let name = tag_name_map
                .get(&tag)
                .cloned()
                .unwrap_or_else(|| tag.clone());
            Some(WeakTag {
                id: tag,
                name,
                wrong_of_20: wrong as u32,
            })
        })
        .collect();

    weak.sort_by(|a, b| b.wrong_of_20.cmp(&a.wrong_of_20));
    Ok(weak)
}

/// Tauri command — returns weak tags for downstream consumers (home pill,
/// deliberate practice scheduler).
#[tauri::command]
pub fn get_weak_tags(state: tauri::State<'_, Db>) -> Result<Vec<WeakTag>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;

    // Build tag → title map from units table.
    let mut stmt = conn
        .prepare("SELECT skill_tag, title FROM units WHERE unit_number IS NOT NULL")
        .map_err(|e| e.to_string())?;
    let map: std::collections::HashMap<String, String> = stmt
        .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    compute_weak_tags(&conn, &map).map_err(|e| e.to_string())
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

    fn insert_attempt(conn: &Connection, tag: &str, correct: bool, ts: i64) {
        let id = format!("{}-{}", tag, ts);
        conn.execute(
            "INSERT INTO attempt_log (id, tag, item_id, correct, learner_answer, timestamp)
             VALUES (?1, ?2, ?3, ?4, '', ?5)",
            params![id, tag, id, correct as i64, ts],
        )
        .unwrap();
    }

    // ── derive_unit_status ────────────────────────────────────────────────────

    #[test]
    fn no_attempts_is_not_started() {
        assert_eq!(derive_unit_status(&[]), "not-started");
    }

    #[test]
    fn fewer_than_20_attempts_is_in_progress() {
        let attempts: Vec<bool> = vec![true; 19];
        assert_eq!(derive_unit_status(&attempts), "in-progress");
    }

    #[test]
    fn exactly_20_correct_is_complete() {
        let attempts: Vec<bool> = vec![true; 20];
        assert_eq!(derive_unit_status(&attempts), "complete");
    }

    #[test]
    fn threshold_at_16_of_20() {
        let mut attempts = vec![false; 4];
        attempts.extend(vec![true; 16]);
        assert_eq!(derive_unit_status(&attempts), "complete");
    }

    #[test]
    fn below_threshold_15_of_20_is_in_progress() {
        let mut attempts = vec![false; 5];
        attempts.extend(vec![true; 15]);
        assert_eq!(derive_unit_status(&attempts), "in-progress");
    }

    #[test]
    fn unit_never_relocks_after_accuracy_dips() {
        // First 20 attempts: 16 correct → complete
        let mut attempts: Vec<bool> = vec![true; 16];
        attempts.extend(vec![false; 4]);
        // Then add 20 more wrong answers — current window is all wrong
        attempts.extend(vec![false; 20]);
        // Should still be "complete" because an earlier window hit the threshold
        assert_eq!(derive_unit_status(&attempts), "complete");
    }

    #[test]
    fn window_slides_correctly() {
        // 19 wrong, then 20 correct — the last window of 20 hits threshold
        let mut attempts = vec![false; 19];
        attempts.extend(vec![true; 20]);
        assert_eq!(derive_unit_status(&attempts), "complete");
    }

    // ── is_currently_mastered ─────────────────────────────────────────────────

    #[test]
    fn currently_mastered_requires_20_attempts() {
        let attempts = vec![true; 19];
        assert!(!is_currently_mastered(&attempts));
    }

    #[test]
    fn currently_mastered_true_at_threshold() {
        let mut attempts = vec![false; 4];
        attempts.extend(vec![true; 16]);
        assert!(is_currently_mastered(&attempts));
    }

    #[test]
    fn currently_mastered_false_after_dip() {
        // Was at threshold, then got 20 wrong
        let mut attempts = vec![true; 16];
        attempts.extend(vec![false; 4]);
        attempts.extend(vec![false; 20]);
        assert!(!is_currently_mastered(&attempts));
    }

    // ── fetch_attempts ────────────────────────────────────────────────────────

    #[test]
    fn fetch_attempts_returns_chronological_order() {
        let conn = setup();
        insert_attempt(&conn, "tag.a", true, 10);
        insert_attempt(&conn, "tag.a", false, 20);
        insert_attempt(&conn, "tag.a", true, 30);

        let result = fetch_attempts(&conn, "tag.a").unwrap();
        assert_eq!(result, vec![true, false, true]);
    }

    #[test]
    fn fetch_attempts_empty_for_unknown_tag() {
        let conn = setup();
        let result = fetch_attempts(&conn, "unknown.tag").unwrap();
        assert!(result.is_empty());
    }

    // ── compute_weak_tags ─────────────────────────────────────────────────────

    #[test]
    fn weak_tags_empty_when_no_attempts() {
        let conn = setup();
        let map = std::collections::HashMap::new();
        let result = compute_weak_tags(&conn, &map).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn weak_tags_excludes_perfect_tag() {
        let conn = setup();
        for ts in 0..10 {
            insert_attempt(&conn, "perfect.tag", true, ts);
        }
        let map = std::collections::HashMap::new();
        let result = compute_weak_tags(&conn, &map).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn weak_tags_includes_tag_with_wrongs() {
        let conn = setup();
        for ts in 0..8 {
            insert_attempt(&conn, "weak.tag", true, ts);
        }
        for ts in 8..10 {
            insert_attempt(&conn, "weak.tag", false, ts);
        }
        let map = std::collections::HashMap::new();
        let result = compute_weak_tags(&conn, &map).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, "weak.tag");
        assert_eq!(result[0].wrong_of_20, 2);
    }

    #[test]
    fn weak_tags_sorted_by_wrong_descending() {
        let conn = setup();
        // tag.a: 1 wrong
        insert_attempt(&conn, "tag.a", true, 1);
        insert_attempt(&conn, "tag.a", false, 2);
        // tag.b: 3 wrong
        insert_attempt(&conn, "tag.b", true, 1);
        insert_attempt(&conn, "tag.b", false, 2);
        insert_attempt(&conn, "tag.b", false, 3);
        insert_attempt(&conn, "tag.b", false, 4);

        let map = std::collections::HashMap::new();
        let result = compute_weak_tags(&conn, &map).unwrap();
        assert_eq!(result[0].id, "tag.b");
        assert_eq!(result[1].id, "tag.a");
    }

    #[test]
    fn weak_tags_uses_unit_title_from_map() {
        let conn = setup();
        insert_attempt(&conn, "unit.tag", false, 1);
        let mut map = std::collections::HashMap::new();
        map.insert("unit.tag".to_string(), "Preterite — regular verbs".to_string());
        let result = compute_weak_tags(&conn, &map).unwrap();
        assert_eq!(result[0].name, "Preterite — regular verbs");
    }

    #[test]
    fn weak_tags_only_looks_at_last_20() {
        let conn = setup();
        // 30 wrong attempts followed by 20 correct → last 20 are all correct → not weak
        for ts in 0..30 {
            insert_attempt(&conn, "tag.x", false, ts);
        }
        for ts in 30..50 {
            insert_attempt(&conn, "tag.x", true, ts);
        }
        let map = std::collections::HashMap::new();
        let result = compute_weak_tags(&conn, &map).unwrap();
        assert!(result.is_empty(), "last 20 are all correct — not weak");
    }
}
