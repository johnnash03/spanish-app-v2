//! Persistence for v2 practice sessions (S6, #37). The attempt log is the
//! single source of truth: every submitted item writes one row eagerly,
//! and the end-of-session review is read back from the log — a session is
//! always reconstructable from the database alone.

use crate::v2::eval::{self, Tier1Analysis, Tier1Outcome};
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
/// attempt log joined with the bank. Tier 1 resolution fields are null
/// until the background evaluation lands (S7, #38).
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
    pub error_category: Option<String>,
    pub hint: Option<String>,
    pub explanation: Option<String>,
}

pub fn start_session(conn: &Connection, unit_id: &str) -> rusqlite::Result<String> {
    let id = fresh_id("ses");
    conn.execute(
        "INSERT INTO sessions (id, unit_id, started_at) VALUES (?1, ?2, ?3)",
        params![id, unit_id, now()],
    )?;
    Ok(id)
}

/// The unit's banked items in randomized serving order — except that
/// skills owed a re-serve (a structure dodge not yet followed by a genuine
/// correct, user story 16) come first.
pub fn session_queue(conn: &Connection, unit_id: &str) -> rusqlite::Result<Vec<QueueItem>> {
    let reserve = skills_needing_reserve(conn, unit_id)?;
    let mut stmt = conn.prepare(
        "SELECT id, source, tags FROM bank_items WHERE unit_id = ?1 ORDER BY RANDOM()",
    )?;
    let mut items: Vec<QueueItem> = stmt
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
    // Stable partition keeps the random order within each half.
    items.sort_by_key(|q| !reserve.contains(&q.target_skill));
    Ok(items)
}

/// Skills whose latest dodge has no later genuine correct: the learner
/// produced good Spanish around the structure but has not yet demonstrated
/// it, so the skill re-serves at the head of the next queue.
pub fn skills_needing_reserve(
    conn: &Connection,
    unit_id: &str,
) -> rusqlite::Result<std::collections::BTreeSet<String>> {
    let mut stmt = conn.prepare(
        "SELECT DISTINCT a.target_skill FROM attempts a
         WHERE a.unit_id = ?1 AND a.status = 'dodge'
           AND NOT EXISTS (
             SELECT 1 FROM attempts b
             WHERE b.target_skill = a.target_skill
               AND b.status = 'correct' AND b.rowid > a.rowid
           )",
    )?;
    let skills = stmt
        .query_map(params![unit_id], |r| r.get(0))?
        .collect::<Result<_, _>>()?;
    Ok(skills)
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

    // An empty submission is wrong by code, never by model: handed to the
    // Tier 1 evaluator it has nothing to judge and the model invents an
    // answer from the cue (v1 logged empty answers as correct).
    let (status, tier, remarks) = if eval::normalize(answer, eval::Leniency::FULL).is_empty() {
        ("wrong", Some(0i64), vec!["No answer given.".to_string()])
    } else {
        match eval::match_answer(answer, &canonical, &variant_texts) {
            Some(m) => ("correct", Some(0i64), m.remarks),
            None => ("pending", None, vec![]),
        }
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

/// What the Tier 1 evaluator needs to know about one pending attempt: the
/// cue, the answer, and the item's skill tags. The canonical answer is
/// deliberately absent — the evaluator never sees it.
#[derive(Debug, Clone)]
pub struct EvalContext {
    pub cue: String,
    pub answer: String,
    pub target_skill: String,
    pub stacked: Vec<String>,
}

/// Loads the evaluation context of a pending attempt. Returns `None` when
/// the attempt is unknown or no longer pending (a verdict must never be
/// overwritten by a late evaluation).
pub fn eval_context(
    conn: &Connection,
    attempt_id: &str,
) -> Result<Option<EvalContext>, String> {
    let row: Option<(String, String, String, String)> = conn
        .query_row(
            "SELECT a.source, a.answer, a.target_skill, COALESCE(b.tags, '{}')
             FROM attempts a LEFT JOIN bank_items b ON b.id = a.item_id
             WHERE a.id = ?1 AND a.status = 'pending'",
            params![attempt_id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
        )
        .optional()
        .map_err(|e| e.to_string())?;
    Ok(row.map(|(cue, answer, target_skill, tags)| {
        let stacked = serde_json::from_str::<serde_json::Value>(&tags)
            .ok()
            .and_then(|t| serde_json::from_value::<Vec<String>>(t["stacked"].clone()).ok())
            .unwrap_or_default();
        EvalContext {
            cue,
            answer,
            target_skill,
            stacked,
        }
    }))
}

/// Writes a Tier 1 resolution onto its pending attempt — the only path
/// from 'pending' to a Tier 1 verdict, and a no-op if the attempt has
/// already been resolved. The full decomposed analysis is kept on the row
/// for inspection and the appeal flow.
pub fn resolve_attempt(
    conn: &Connection,
    attempt_id: &str,
    analysis: &Tier1Analysis,
    outcome: &Tier1Outcome,
) -> Result<(), String> {
    let judgments = serde_json::to_string(analysis).map_err(|e| e.to_string())?;
    let (status, remarks, category, evidence, skills, hint, explanation) = match outcome {
        Tier1Outcome::Correct => ("correct", vec![], None, None, None, None, None),
        Tier1Outcome::Dodge { nudge } => {
            ("dodge", vec![nudge.clone()], None, None, None, None, None)
        }
        Tier1Outcome::Wrong {
            category,
            evidence,
            hint,
            explanation,
            skills,
        } => (
            "wrong",
            vec![],
            Some(category.wire_name()),
            Some(evidence.clone()),
            Some(serde_json::to_string(skills).map_err(|e| e.to_string())?),
            hint.clone(),
            explanation.clone(),
        ),
    };
    let updated = conn
        .execute(
            "UPDATE attempts SET
               status = ?2, tier = 1, remarks = ?3, judgments = ?4,
               error_category = ?5, error_evidence = ?6, error_skills = ?7,
               hint = ?8, explanation = ?9
             WHERE id = ?1 AND status = 'pending'",
            params![
                attempt_id,
                status,
                serde_json::to_string(&remarks).map_err(|e| e.to_string())?,
                judgments,
                category,
                evidence,
                skills,
                hint,
                explanation,
            ],
        )
        .map_err(|e| e.to_string())?;
    if updated == 0 {
        eprintln!("[tier1 {attempt_id}] resolution dropped: attempt missing or already resolved");
    }
    Ok(())
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
                COALESCE(b.canonical, ''), a.error_category, a.hint, a.explanation
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
                error_category: r.get(7)?,
                hint: r.get(8)?,
                explanation: r.get(9)?,
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
    fn empty_answers_are_wrong_by_code_and_never_reach_tier1() {
        // V1 marked empty submissions correct; v2 settles them
        // deterministically — no pending status, so no evaluator call.
        let conn = in_memory();
        seed_item(&conn, "a", "I want to eat.", "Quiero comer.", &[]);
        let session = start_session(&conn, "opener.quiero").unwrap();
        for empty in ["", "   ", "¿?"] {
            let v = submit_attempt(&conn, &session, "a", empty).unwrap();
            assert_eq!(v.status, "wrong", "{empty:?}");
            assert_eq!(v.remarks, vec!["No answer given.".to_string()]);
            assert!(eval_context(&conn, &v.attempt_id).unwrap().is_none());
        }
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

    fn pending_attempt(conn: &Connection, session: &str, answer: &str) -> String {
        let v = submit_attempt(conn, session, "a", answer).unwrap();
        assert_eq!(v.status, "pending");
        v.attempt_id
    }

    fn stub_analysis() -> Tier1Analysis {
        use crate::v2::eval::tier1::Judgment;
        Tier1Analysis {
            accent_restored_answer: "restored".into(),
            grammatical: Judgment { verdict: true, evidence: "".into() },
            conveys_meaning: Judgment { verdict: true, evidence: "".into() },
            uses_target_structure: Judgment { verdict: false, evidence: "".into() },
            error: None,
            hint: None,
            explanation: None,
        }
    }

    #[test]
    fn resolving_wrong_writes_classification_and_review_reads_it_back() {
        let conn = in_memory();
        seed_item(&conn, "a", "I want to eat.", "Quiero comer.", &[]);
        let session = start_session(&conn, "opener.quiero").unwrap();
        let attempt = pending_attempt(&conn, &session, "Quieromos comer.");

        resolve_attempt(
            &conn,
            &attempt,
            &stub_analysis(),
            &Tier1Outcome::Wrong {
                category: crate::v2::eval::ErrorCategory::VerbForm,
                evidence: "Quieromos".into(),
                hint: Some("Check the nosotros form.".into()),
                explanation: Some("The 1pl of querer is queremos.".into()),
                skills: vec!["opener.quiero".into()],
            },
        )
        .unwrap();

        let attempts = session_attempts(&conn, &session).unwrap();
        assert_eq!(attempts[0].status, "wrong");
        assert_eq!(attempts[0].error_category.as_deref(), Some("verb-form"));
        assert_eq!(attempts[0].hint.as_deref(), Some("Check the nosotros form."));
        assert!(attempts[0].explanation.as_deref().unwrap().contains("queremos"));
        // The full decomposed analysis is on the row for inspection.
        let judgments: String = conn
            .query_row(
                "SELECT judgments FROM attempts WHERE id = ?1",
                params![attempt],
                |r| r.get(0),
            )
            .unwrap();
        assert!(serde_json::from_str::<Tier1Analysis>(&judgments).is_ok());
    }

    #[test]
    fn resolving_a_dodge_writes_the_nudge_and_marks_the_skill_for_reserve() {
        let conn = in_memory();
        seed_item(&conn, "a", "I want to eat.", "Quiero comer.", &[]);
        let session = start_session(&conn, "opener.quiero").unwrap();
        let attempt = pending_attempt(&conn, &session, "Me gustaría comer.");

        resolve_attempt(
            &conn,
            &attempt,
            &stub_analysis(),
            &Tier1Outcome::Dodge { nudge: "Correct Spanish — try it with quiero.".into() },
        )
        .unwrap();

        let attempts = session_attempts(&conn, &session).unwrap();
        assert_eq!(attempts[0].status, "dodge");
        assert_eq!(attempts[0].remarks, vec!["Correct Spanish — try it with quiero.".to_string()]);
        assert!(attempts[0].error_category.is_none(), "a dodge is not an error");

        // The skill is owed a re-serve until a genuine correct lands.
        let reserve = skills_needing_reserve(&conn, "opener.quiero").unwrap();
        assert!(reserve.contains("opener.quiero"));
        let queue = session_queue(&conn, "opener.quiero").unwrap();
        assert_eq!(queue[0].target_skill, "opener.quiero");

        // A later genuine correct clears the debt.
        let v = submit_attempt(&conn, &session, "a", "Quiero comer.").unwrap();
        assert_eq!(v.status, "correct");
        assert!(skills_needing_reserve(&conn, "opener.quiero").unwrap().is_empty());
    }

    #[test]
    fn resolution_never_overwrites_an_already_resolved_attempt() {
        let conn = in_memory();
        seed_item(&conn, "a", "I want to eat.", "Quiero comer.", &[]);
        let session = start_session(&conn, "opener.quiero").unwrap();
        let attempt = pending_attempt(&conn, &session, "Quieromos comer.");

        resolve_attempt(&conn, &attempt, &stub_analysis(), &Tier1Outcome::Correct).unwrap();
        // A late second resolution (e.g. a retried call) must not clobber.
        resolve_attempt(
            &conn,
            &attempt,
            &stub_analysis(),
            &Tier1Outcome::Dodge { nudge: "late".into() },
        )
        .unwrap();
        let attempts = session_attempts(&conn, &session).unwrap();
        assert_eq!(attempts[0].status, "correct");
    }

    #[test]
    fn eval_context_serves_pending_attempts_without_the_canonical() {
        let conn = in_memory();
        seed_item(&conn, "a", "I want to eat.", "Quiero comer.", &[]);
        let session = start_session(&conn, "opener.quiero").unwrap();
        let attempt = pending_attempt(&conn, &session, "Deseo comer.");

        let ctx = eval_context(&conn, &attempt).unwrap().unwrap();
        assert_eq!(ctx.cue, "I want to eat.");
        assert_eq!(ctx.answer, "Deseo comer.");
        assert_eq!(ctx.target_skill, "opener.quiero");
        assert!(ctx.stacked.is_empty());

        // Resolved attempts no longer offer a context — no re-evaluation.
        resolve_attempt(&conn, &attempt, &stub_analysis(), &Tier1Outcome::Correct).unwrap();
        assert!(eval_context(&conn, &attempt).unwrap().is_none());
        assert!(eval_context(&conn, "nope").unwrap().is_none());
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
