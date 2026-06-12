//! Persistence for v2 practice sessions (S6, #37). The attempt log is the
//! single source of truth: every submitted item writes one row eagerly,
//! and the end-of-session review is read back from the log — a session is
//! always reconstructable from the database alone.

use crate::v2::eval;
use crate::v2::generator::ValidatedVariant;
use rusqlite::{params, Connection, OptionalExtension};
use serde::Serialize;
use std::sync::atomic::{AtomicU64, Ordering};

fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

/// Unique-enough id for a single-user local log; the counter breaks ties
/// within one clock reading.
fn fresh_id(prefix: &str) -> String {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("{prefix}-{nanos:x}-{n}")
}

/// One queue entry as the session screen consumes it. The canonical and
/// variants deliberately stay server-side: the UI never holds answers.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueItem {
    pub id: String,
    pub source: String,
    pub target_skill: String,
}

/// The Tier 0 verdict of one submitted attempt, as returned to the UI the
/// moment it is persisted. `correct` carries its deterministic remarks;
/// `pending` waits for the Tier 1 evaluator (S7, #38).
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttemptVerdict {
    pub attempt_id: String,
    pub item_id: String,
    pub status: String,
    pub remarks: Vec<String>,
}

/// One attempt as the end-of-session review shows it, read back from the
/// attempt log joined with the bank.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReviewAttempt {
    pub item_id: String,
    pub source: String,
    pub answer: String,
    pub status: String,
    pub remarks: Vec<String>,
    pub canonical: String,
    pub target_skill: String,
}

pub fn start_session(conn: &Connection, unit_id: &str) -> rusqlite::Result<String> {
    let id = fresh_id("ses");
    conn.execute(
        "INSERT INTO sessions (id, unit_id, started_at) VALUES (?1, ?2, ?3)",
        params![id, unit_id, now()],
    )?;
    Ok(id)
}

/// The unit's banked items in randomized serving order.
pub fn session_queue(conn: &Connection, unit_id: &str) -> rusqlite::Result<Vec<QueueItem>> {
    let mut stmt = conn.prepare(
        "SELECT id, source, tags FROM bank_items WHERE unit_id = ?1 ORDER BY RANDOM()",
    )?;
    let items = stmt
        .query_map(params![unit_id], |r| {
            let tags: String = r.get(2)?;
            Ok(QueueItem {
                id: r.get(0)?,
                source: r.get(1)?,
                target_skill: serde_json::from_str::<serde_json::Value>(&tags)
                    .ok()
                    .and_then(|t| t["target_skill"].as_str().map(String::from))
                    .unwrap_or_default(),
            })
        })?
        .collect::<Result<_, _>>()?;
    Ok(items)
}

/// Runs Tier 0 on a submitted answer and writes the attempt to the log in
/// the same call — deterministic, instant, no network. Returns an error
/// for unknown items: an attempt against nothing must not enter the log.
pub fn submit_attempt(
    conn: &Connection,
    session_id: &str,
    item_id: &str,
    answer: &str,
) -> Result<AttemptVerdict, String> {
    let (unit_id, source, canonical, variants_json, tags_json): (
        String,
        String,
        String,
        String,
        String,
    ) = conn
        .query_row(
            "SELECT unit_id, source, canonical, variants, tags FROM bank_items WHERE id = ?1",
            params![item_id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)),
        )
        .optional()
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("unknown item `{item_id}`"))?;

    let variants: Vec<ValidatedVariant> =
        serde_json::from_str(&variants_json).map_err(|e| e.to_string())?;
    let variant_texts: Vec<String> = variants.into_iter().map(|v| v.text).collect();
    let target_skill = serde_json::from_str::<serde_json::Value>(&tags_json)
        .ok()
        .and_then(|t| t["target_skill"].as_str().map(String::from))
        .unwrap_or_default();

    let (status, tier, remarks) = match eval::match_answer(answer, &canonical, &variant_texts) {
        Some(m) => ("correct", Some(0i64), m.remarks),
        None => ("pending", None, vec![]),
    };

    let attempt_id = fresh_id("att");
    conn.execute(
        "INSERT INTO attempts
         (id, session_id, item_id, unit_id, target_skill, source, answer, status, tier, remarks, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![
            attempt_id,
            session_id,
            item_id,
            unit_id,
            target_skill,
            source,
            answer,
            status,
            tier,
            serde_json::to_string(&remarks).unwrap(),
            now(),
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(AttemptVerdict {
        attempt_id,
        item_id: item_id.to_string(),
        status: status.to_string(),
        remarks,
    })
}

pub fn end_session(conn: &Connection, session_id: &str) -> rusqlite::Result<()> {
    conn.execute(
        "UPDATE sessions SET ended_at = ?2 WHERE id = ?1 AND ended_at IS NULL",
        params![session_id, now()],
    )?;
    Ok(())
}

/// The session's attempts in submission order, joined with the bank for
/// the canonical answer (review's "Correct: …" line).
pub fn session_attempts(
    conn: &Connection,
    session_id: &str,
) -> rusqlite::Result<Vec<ReviewAttempt>> {
    let mut stmt = conn.prepare(
        "SELECT a.item_id, a.source, a.answer, a.status, a.remarks, a.target_skill,
                COALESCE(b.canonical, '')
         FROM attempts a LEFT JOIN bank_items b ON b.id = a.item_id
         WHERE a.session_id = ?1
         ORDER BY a.created_at, a.id",
    )?;
    let attempts = stmt
        .query_map(params![session_id], |r| {
            let remarks: String = r.get(4)?;
            Ok(ReviewAttempt {
                item_id: r.get(0)?,
                source: r.get(1)?,
                answer: r.get(2)?,
                status: r.get(3)?,
                remarks: serde_json::from_str(&remarks).unwrap_or_default(),
                target_skill: r.get(5)?,
                canonical: r.get(6)?,
            })
        })?
        .collect::<Result<_, _>>()?;
    Ok(attempts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2::db::run_migrations;
    use crate::v2::generator::bank::persist_item;
    use crate::v2::generator::plan::ItemTags;
    use crate::v2::generator::{BankItem, ValidatedVariant};
    use crate::v2::validator::{ItemAnalysis, SlotSpec};

    fn in_memory() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        run_migrations(&conn).unwrap();
        conn
    }

    fn seed_item(conn: &Connection, id: &str, source: &str, canonical: &str, variants: &[&str]) {
        persist_item(
            conn,
            &BankItem {
                id: id.into(),
                unit_id: "opener.quiero".into(),
                source: source.into(),
                canonical: canonical.into(),
                variants: variants
                    .iter()
                    .map(|t| ValidatedVariant {
                        text: t.to_string(),
                        analysis: ItemAnalysis::default(),
                    })
                    .collect(),
                slot: SlotSpec::default(),
                tags: ItemTags {
                    target_skill: "opener.quiero".into(),
                    stacked: vec![],
                },
                analysis: ItemAnalysis::default(),
            },
        )
        .unwrap();
    }

    #[test]
    fn queue_serves_the_units_banked_items() {
        let conn = in_memory();
        seed_item(&conn, "a", "I want to eat.", "Quiero comer.", &[]);
        seed_item(&conn, "b", "I want to dance.", "Quiero bailar.", &[]);

        let queue = session_queue(&conn, "opener.quiero").unwrap();
        assert_eq!(queue.len(), 2);
        let mut ids: Vec<_> = queue.iter().map(|q| q.id.as_str()).collect();
        ids.sort();
        assert_eq!(ids, vec!["a", "b"]);
        assert!(queue.iter().all(|q| q.target_skill == "opener.quiero"));
        assert!(session_queue(&conn, "opener.puedo").unwrap().is_empty());
    }

    #[test]
    fn submit_attempt_resolves_tier0_and_persists_to_the_log() {
        let conn = in_memory();
        seed_item(
            &conn,
            "a",
            "You can see them.",
            "Puedes verlos.",
            &["Los puedes ver."],
        );
        let session = start_session(&conn, "opener.quiero").unwrap();

        // Clean variant match: instant correct, no remarks.
        let v = submit_attempt(&conn, &session, "a", "Los puedes ver.").unwrap();
        assert_eq!(v.status, "correct");
        assert!(v.remarks.is_empty());

        // Accent/orthography slip: correct with a deterministic remark.
        let v = submit_attempt(&conn, &session, "a", "puedes verlos").unwrap();
        assert_eq!(v.status, "correct");
        assert_eq!(v.remarks.len(), 1);

        // No match: pending until Tier 1 (S7).
        let v = submit_attempt(&conn, &session, "a", "Puedo verlos.").unwrap();
        assert_eq!(v.status, "pending");

        // The log holds all three, reconstructable in order with verdicts.
        let attempts = session_attempts(&conn, &session).unwrap();
        assert_eq!(attempts.len(), 3);
        assert_eq!(attempts[0].answer, "Los puedes ver.");
        assert_eq!(attempts[0].status, "correct");
        assert_eq!(attempts[1].remarks.len(), 1);
        assert_eq!(attempts[2].status, "pending");
        assert!(attempts.iter().all(|a| a.canonical == "Puedes verlos."));
        assert!(attempts.iter().all(|a| a.source == "You can see them."));
    }

    #[test]
    fn submit_attempt_rejects_unknown_items() {
        let conn = in_memory();
        let session = start_session(&conn, "opener.quiero").unwrap();
        let err = submit_attempt(&conn, &session, "nope", "Quiero comer.").unwrap_err();
        assert!(err.contains("nope"));
        assert!(session_attempts(&conn, &session).unwrap().is_empty());
    }

    #[test]
    fn end_session_stamps_ended_at_once() {
        let conn = in_memory();
        let session = start_session(&conn, "opener.quiero").unwrap();
        let ended: Option<i64> = conn
            .query_row(
                "SELECT ended_at FROM sessions WHERE id = ?1",
                params![session],
                |r| r.get(0),
            )
            .unwrap();
        assert!(ended.is_none(), "a fresh session is unended");

        end_session(&conn, &session).unwrap();
        let ended: Option<i64> = conn
            .query_row(
                "SELECT ended_at FROM sessions WHERE id = ?1",
                params![session],
                |r| r.get(0),
            )
            .unwrap();
        assert!(ended.is_some());
    }

    #[test]
    fn sessions_are_isolated_in_the_log() {
        let conn = in_memory();
        seed_item(&conn, "a", "I want to eat.", "Quiero comer.", &[]);
        let s1 = start_session(&conn, "opener.quiero").unwrap();
        let s2 = start_session(&conn, "opener.quiero").unwrap();
        submit_attempt(&conn, &s1, "a", "Quiero comer.").unwrap();
        submit_attempt(&conn, &s2, "a", "quiero comer").unwrap();

        assert_eq!(session_attempts(&conn, &s1).unwrap().len(), 1);
        assert_eq!(session_attempts(&conn, &s2).unwrap().len(), 1);
    }
}
