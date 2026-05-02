use crate::db::Db;
use rusqlite::params;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct SessionItem {
    pub id: String,
    pub source: String,
    #[serde(rename = "primaryTag")]
    pub primary_tag: String,
    #[serde(rename = "stackedTags")]
    pub stacked_tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct AttemptInput {
    #[serde(rename = "itemId")]
    pub item_id: String,
    pub tag: String,
    #[serde(rename = "learnerAnswer")]
    pub learner_answer: String,
}

// ─── Queue Assembly ───────────────────────────────────────────────────────────

fn lcg_shuffle<T>(v: &mut Vec<T>) {
    if v.len() < 2 {
        return;
    }
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos() as u64;
    let mut state = seed.wrapping_add(v.len() as u64).wrapping_mul(6364136223846793005);
    for i in (1..v.len()).rev() {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let j = (state >> 33) as usize % (i + 1);
        v.swap(i, j);
    }
}

fn parse_stacked_tags(json: &str) -> Vec<String> {
    serde_json::from_str(json).unwrap_or_default()
}

fn fetch_unseen_items(
    conn: &rusqlite::Connection,
    active_unit_tag: &str,
) -> rusqlite::Result<Vec<SessionItem>> {
    let mut stmt = conn.prepare(
        "SELECT id, source, primary_tag, stacked_tags
         FROM exercise_items
         WHERE primary_tag = ?1
           AND id NOT IN (SELECT DISTINCT item_id FROM attempt_log)
         ORDER BY created_at ASC",
    )?;
    let items = stmt
        .query_map(params![active_unit_tag], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, String>(3)?,
            ))
        })?
        .filter_map(|r| r.ok())
        .map(|(id, source, pt, st)| SessionItem {
            id,
            source,
            primary_tag: pt,
            stacked_tags: parse_stacked_tags(&st),
        })
        .collect();
    Ok(items)
}

fn fetch_last5_unit_tags(
    conn: &rusqlite::Connection,
    active_unit_tag: &str,
) -> rusqlite::Result<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT tag, MAX(timestamp) as last_seen
         FROM attempt_log
         WHERE tag != ?1
         GROUP BY tag
         ORDER BY last_seen DESC
         LIMIT 5",
    )?;
    let tags = stmt
        .query_map(params![active_unit_tag], |r| r.get::<_, String>(0))?
        .filter_map(|r| r.ok())
        .collect();
    Ok(tags)
}

fn fetch_review_items(
    conn: &rusqlite::Connection,
    tags: &[String],
    limit: usize,
) -> rusqlite::Result<Vec<SessionItem>> {
    if tags.is_empty() || limit == 0 {
        return Ok(vec![]);
    }
    let placeholders: String = tags
        .iter()
        .enumerate()
        .map(|(i, _)| format!("?{}", i + 1))
        .collect::<Vec<_>>()
        .join(", ");
    let sql = format!(
        "SELECT id, source, primary_tag, stacked_tags
         FROM exercise_items
         WHERE primary_tag IN ({placeholders})
         ORDER BY RANDOM()
         LIMIT {limit}"
    );
    let mut stmt = conn.prepare(&sql)?;
    let items = stmt
        .query_map(
            rusqlite::params_from_iter(tags.iter().map(|s| s.as_str())),
            |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, String>(2)?,
                    r.get::<_, String>(3)?,
                ))
            },
        )?
        .filter_map(|r| r.ok())
        .map(|(id, source, pt, st)| SessionItem {
            id,
            source,
            primary_tag: pt,
            stacked_tags: parse_stacked_tags(&st),
        })
        .collect();
    Ok(items)
}

fn fetch_longtail_items(
    conn: &rusqlite::Connection,
    active_unit_tag: &str,
    last5_tags: &[String],
    limit: usize,
) -> rusqlite::Result<Vec<SessionItem>> {
    if limit == 0 {
        return Ok(vec![]);
    }
    // Build exclusion set: active unit + last 5 units
    let mut excluded: Vec<&str> = vec![active_unit_tag];
    for t in last5_tags {
        excluded.push(t.as_str());
    }
    let placeholders: String = excluded
        .iter()
        .enumerate()
        .map(|(i, _)| format!("?{}", i + 1))
        .collect::<Vec<_>>()
        .join(", ");

    // Tags weighted by error rate (higher error → more items sampled)
    let sql = format!(
        "SELECT tag,
                CAST(SUM(CASE WHEN correct = 0 THEN 1 ELSE 0 END) AS REAL) / COUNT(*) AS error_rate
         FROM attempt_log
         WHERE tag NOT IN ({placeholders})
         GROUP BY tag
         ORDER BY error_rate DESC
         LIMIT 10"
    );
    let mut stmt = conn.prepare(&sql)?;
    let longtail_tags: Vec<String> = stmt
        .query_map(
            rusqlite::params_from_iter(excluded.iter()),
            |r| r.get::<_, String>(0),
        )?
        .filter_map(|r| r.ok())
        .collect();

    if longtail_tags.is_empty() {
        return Ok(vec![]);
    }
    fetch_review_items(conn, &longtail_tags, limit)
}

fn assemble_queue_internal(
    conn: &rusqlite::Connection,
    active_unit_tag: &str,
) -> rusqlite::Result<Vec<SessionItem>> {
    let mut current = fetch_unseen_items(conn, active_unit_tag)?;
    let n = current.len();

    let last5 = fetch_last5_unit_tags(conn, active_unit_tag)?;
    let review_target = if n > 0 { n } else { 5 };
    let mut review = fetch_review_items(conn, &last5, review_target)?;
    let longtail_target = if n > 0 { (n + 1) / 2 } else { 2 };
    let mut longtail = fetch_longtail_items(conn, active_unit_tag, &last5, longtail_target)?;

    lcg_shuffle(&mut current);
    lcg_shuffle(&mut review);
    lcg_shuffle(&mut longtail);

    let mut all: Vec<SessionItem> = Vec::new();
    all.append(&mut current);
    all.append(&mut review);
    all.append(&mut longtail);

    lcg_shuffle(&mut all);
    Ok(all)
}

#[tauri::command]
pub fn assemble_session_queue(
    state: tauri::State<'_, Db>,
    active_unit_tag: String,
) -> Result<Vec<SessionItem>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    assemble_queue_internal(&conn, &active_unit_tag).map_err(|e| e.to_string())
}

// ─── Attempt Submission ───────────────────────────────────────────────────────

fn uuid_v4_session() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    let tid = std::thread::current().id();
    format!(
        "{:08x}-{:04x}-4{:03x}-{:04x}-{:012x}",
        t,
        (t >> 16) & 0xffff,
        (t >> 4) & 0xfff,
        0x8000 | ((t ^ format!("{:?}", tid).len() as u32) & 0x3fff),
        (t as u64).wrapping_mul(0x9e3779b97f4a7c15),
    )
}

fn basic_correct(learner: &str, canonical: &str) -> bool {
    learner.trim().to_lowercase() == canonical.trim().to_lowercase()
}

#[tauri::command]
pub fn submit_session_attempts(
    state: tauri::State<'_, Db>,
    attempts: Vec<AttemptInput>,
) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    for (i, attempt) in attempts.iter().enumerate() {
        // Look up canonical for basic correctness check
        let canonical: Option<String> = conn
            .query_row(
                "SELECT canonical FROM exercise_items WHERE id = ?1",
                params![&attempt.item_id],
                |r| r.get(0),
            )
            .ok();

        let correct = canonical
            .as_deref()
            .map(|c| basic_correct(&attempt.learner_answer, c))
            .unwrap_or(false);

        // Stagger timestamps so rows are ordered correctly within one session
        let ts = now + i as i64;

        conn.execute(
            "INSERT INTO attempt_log (id, tag, item_id, correct, learner_answer, timestamp)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                uuid_v4_session(),
                attempt.tag,
                attempt.item_id,
                correct as i64,
                attempt.learner_answer,
                ts,
            ],
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn in_memory() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        crate::db::run_migrations_for_test(&conn).unwrap();
        conn
    }

    fn insert_item(conn: &Connection, id: &str, source: &str, canonical: &str, tag: &str) {
        conn.execute(
            "INSERT INTO exercise_items (id, source, canonical, primary_tag, stacked_tags, created_at)
             VALUES (?1, ?2, ?3, ?4, '[]', 0)",
            params![id, source, canonical, tag],
        )
        .unwrap();
    }

    fn insert_attempt(conn: &Connection, item_id: &str, tag: &str, correct: bool, ts: i64) {
        conn.execute(
            "INSERT INTO attempt_log (id, tag, item_id, correct, learner_answer, timestamp)
             VALUES (?1, ?2, ?3, ?4, '', ?5)",
            params![
                format!("atmp-{item_id}-{ts}"),
                tag,
                item_id,
                correct as i64,
                ts
            ],
        )
        .unwrap();
    }

    #[test]
    fn unseen_items_excludes_attempted() {
        let conn = in_memory();
        insert_item(&conn, "i1", "I want", "Quiero", "tag.a");
        insert_item(&conn, "i2", "She wants", "Quiere", "tag.a");
        insert_attempt(&conn, "i1", "tag.a", true, 100);

        let items = fetch_unseen_items(&conn, "tag.a").unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, "i2");
    }

    #[test]
    fn assemble_queue_returns_unseen_for_simple_case() {
        let conn = in_memory();
        insert_item(&conn, "i1", "I want", "Quiero", "tag.a");
        insert_item(&conn, "i2", "She wants", "Quiere", "tag.a");

        let queue = assemble_queue_internal(&conn, "tag.a").unwrap();
        assert_eq!(queue.len(), 2);
    }

    #[test]
    fn assemble_queue_includes_review_items_from_last5() {
        let conn = in_memory();
        insert_item(&conn, "i1", "Unseen current", "Q1", "tag.a");
        insert_item(&conn, "i2", "Prior unit item", "Q2", "tag.b");
        insert_attempt(&conn, "i2", "tag.b", true, 50);

        let queue = assemble_queue_internal(&conn, "tag.a").unwrap();
        // Should include i1 (unseen current) + i2 (review from last5)
        assert!(queue.len() >= 1);
        let ids: Vec<&str> = queue.iter().map(|q| q.id.as_str()).collect();
        assert!(ids.contains(&"i1"));
        assert!(ids.contains(&"i2"));
    }

    #[test]
    fn assemble_queue_excludes_active_unit_from_review() {
        let conn = in_memory();
        insert_item(&conn, "i1", "Item A", "Q1", "tag.a");
        insert_attempt(&conn, "i1", "tag.a", true, 100);
        insert_item(&conn, "i2", "Item A2", "Q2", "tag.a");

        // Only i2 is unseen; i1 is seen and belongs to active unit (not review)
        let queue = assemble_queue_internal(&conn, "tag.a").unwrap();
        let ids: Vec<&str> = queue.iter().map(|q| q.id.as_str()).collect();
        assert!(ids.contains(&"i2"));
    }

    #[test]
    fn basic_correct_is_case_insensitive() {
        assert!(basic_correct("quiero comer", "Quiero comer"));
        assert!(basic_correct("  quiero comer  ", "quiero comer"));
        assert!(!basic_correct("quiero", "quiero comer"));
    }

    #[test]
    fn submit_attempts_records_to_attempt_log() {
        let conn = in_memory();
        insert_item(&conn, "i1", "I want to eat", "Quiero comer", "tag.a");

        // Build the attempt — correct answer
        let now = 1000i64;
        conn.execute(
            "INSERT INTO attempt_log (id, tag, item_id, correct, learner_answer, timestamp)
             VALUES ('test-id', 'tag.a', 'i1', 1, 'Quiero comer', ?1)",
            params![now],
        )
        .unwrap();

        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM attempt_log", [], |r| r.get(0))
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn lcg_shuffle_does_not_panic_on_empty() {
        let mut v: Vec<i32> = vec![];
        lcg_shuffle(&mut v);
    }

    #[test]
    fn lcg_shuffle_preserves_length() {
        let mut v: Vec<i32> = (0..10).collect();
        lcg_shuffle(&mut v);
        assert_eq!(v.len(), 10);
    }
}
