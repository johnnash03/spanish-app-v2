//! V2 practice session loop (S6, #37): one-at-a-time serving with eager
//! per-item resolution. Tier 0 verdicts resolve in-process — instant and
//! offline — and every attempt lands in the v2 attempt log the moment it
//! is submitted, so an interrupted session loses nothing (user stories 8,
//! 9, 53, 54, 55). Answers Tier 0 cannot match fire a background Tier 1
//! evaluation as they are submitted (S7, #38), so the batched review is
//! ready at session end.

pub mod store;

use crate::v2::curriculum::{Curriculum, CurriculumState};
use crate::v2::db::DbV2;
use crate::v2::eval::{self, EvalInput, Evaluator, OpenAiEvaluator};
use crate::v2::generator::bank;
use rusqlite::Connection;
use serde::Serialize;
use tauri::Manager;
pub use store::{AttemptVerdict, QueueItem, ReviewAttempt};

/// One curriculum unit as the v2 unit picker lists it.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitRow {
    pub id: String,
    pub title: String,
    pub phase: u32,
    pub bank_count: i64,
    pub generation_state: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartSessionResponse {
    pub session_id: String,
    pub items: Vec<QueueItem>,
}

pub fn list_units(conn: &Connection, curriculum: &Curriculum) -> rusqlite::Result<Vec<UnitRow>> {
    curriculum
        .units
        .iter()
        .map(|u| {
            Ok(UnitRow {
                id: u.id.clone(),
                title: u.title.clone(),
                phase: u.phase,
                bank_count: bank::bank_count(conn, &u.id)?,
                generation_state: bank::generation_state(conn, &u.id),
            })
        })
        .collect()
}

/// Units in authored curriculum order, with bank readiness for the picker.
#[tauri::command]
pub fn v2_list_units(
    db: tauri::State<'_, DbV2>,
    curriculum: tauri::State<'_, CurriculumState>,
) -> Result<Vec<UnitRow>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    list_units(&conn, &curriculum.0).map_err(|e| e.to_string())
}

/// Opens a session row and returns the unit's queue in serving order.
#[tauri::command]
pub fn v2_start_session(
    db: tauri::State<'_, DbV2>,
    curriculum: tauri::State<'_, CurriculumState>,
    unit_id: String,
) -> Result<StartSessionResponse, String> {
    if curriculum.0.unit(&unit_id).is_none() {
        return Err(format!("unknown unit `{unit_id}`"));
    }
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let items = store::session_queue(&conn, &unit_id).map_err(|e| e.to_string())?;
    if items.is_empty() {
        return Err(format!("unit `{unit_id}` has no banked items"));
    }
    let session_id = store::start_session(&conn, &unit_id).map_err(|e| e.to_string())?;
    Ok(StartSessionResponse { session_id, items })
}

/// Runs one pending attempt through the Tier 1 evaluator and writes the
/// resolution. Any failure — transport, parse, resolution — leaves the
/// attempt pending (fail-safe: no verdict is ever invented), to be retried
/// on a later firing. The database lock is never held across the call.
pub async fn evaluate_pending<E: Evaluator>(
    db: &DbV2,
    curriculum: &Curriculum,
    evaluator: &E,
    attempt_id: &str,
) -> Result<(), String> {
    let ctx = {
        let conn = db.0.lock().map_err(|e| e.to_string())?;
        store::eval_context(&conn, attempt_id)?
    };
    // Unknown or already resolved: nothing to evaluate.
    let Some(ctx) = ctx else { return Ok(()) };

    let target_description = eval::target_description(curriculum, &ctx.target_skill)
        .unwrap_or_else(|| ctx.target_skill.clone());
    let target_title = curriculum
        .unit(&ctx.target_skill)
        .map(|u| u.title.clone())
        .unwrap_or_else(|| ctx.target_skill.clone());

    let analysis = evaluator
        .evaluate(&EvalInput {
            cue: ctx.cue.clone(),
            answer: ctx.answer.clone(),
            target_description,
        })
        .await
        .map_err(|e| e.to_string())?;
    let outcome = eval::resolve(
        &analysis,
        &ctx.target_skill,
        &target_title,
        &ctx.stacked,
        curriculum,
    )
    .map_err(|e| e.to_string())?;

    let conn = db.0.lock().map_err(|e| e.to_string())?;
    store::resolve_attempt(&conn, attempt_id, &analysis, &outcome)
}

/// Eager per-item resolution: Tier 0 runs and the attempt persists before
/// this returns; no network on that path. An unmatched answer fires its
/// Tier 1 evaluation in the background as it is submitted (PRD #31:
/// batched review with eager evaluation) — the learner types on
/// uninterrupted.
#[tauri::command]
pub async fn v2_submit_attempt(
    app: tauri::AppHandle,
    session_id: String,
    item_id: String,
    answer: String,
) -> Result<AttemptVerdict, String> {
    let verdict = {
        let db = app.state::<DbV2>();
        let conn = db.0.lock().map_err(|e| e.to_string())?;
        store::submit_attempt(&conn, &session_id, &item_id, &answer)?
    };
    if verdict.status == "pending" {
        let attempt_id = verdict.attempt_id.clone();
        tauri::async_runtime::spawn(async move {
            match OpenAiEvaluator::from_env() {
                Ok(evaluator) => {
                    let db = app.state::<DbV2>();
                    let curriculum = app.state::<CurriculumState>().0.clone();
                    if let Err(e) =
                        evaluate_pending(&db, &curriculum, &evaluator, &attempt_id).await
                    {
                        eprintln!("[tier1 {attempt_id}] evaluation failed, attempt stays pending: {e}");
                    }
                }
                // Offline / unconfigured: the attempt stays pending in the
                // log (PRD: evaluation runs on reconnect).
                Err(e) => eprintln!("[tier1 {attempt_id}] evaluator unavailable: {e}"),
            }
        });
    }
    Ok(verdict)
}

/// Read-only snapshot of the session's attempts — the review screen polls
/// this while background Tier 1 evaluations land.
#[tauri::command]
pub fn v2_session_review(
    db: tauri::State<'_, DbV2>,
    session_id: String,
) -> Result<Vec<ReviewAttempt>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    store::session_attempts(&conn, &session_id).map_err(|e| e.to_string())
}

/// Stamps the session ended and returns the batched review, read back
/// from the attempt log.
#[tauri::command]
pub fn v2_end_session(
    db: tauri::State<'_, DbV2>,
    session_id: String,
) -> Result<Vec<ReviewAttempt>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    store::end_session(&conn, &session_id).map_err(|e| e.to_string())?;
    store::session_attempts(&conn, &session_id).map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2::curriculum;
    use crate::v2::db::run_migrations;
    use crate::v2::eval::tier1::{Judgment, Tier1Analysis};
    use crate::v2::eval::Tier1Error;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Mutex;

    /// Stub evaluator: canned analysis, counts its calls, no network.
    struct StubEvaluator {
        result: Result<Tier1Analysis, &'static str>,
        calls: AtomicUsize,
    }

    impl Evaluator for StubEvaluator {
        async fn evaluate(&self, _input: &EvalInput) -> Result<Tier1Analysis, Tier1Error> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            self.result
                .clone()
                .map_err(|e| Tier1Error::Transport(e.into()))
        }
    }

    fn stub(a: bool, b: bool, c: bool) -> StubEvaluator {
        let judgment = |v| Judgment { verdict: v, evidence: "stub".into() };
        StubEvaluator {
            result: Ok(Tier1Analysis {
                accent_restored_answer: "restored".into(),
                grammatical: judgment(a),
                conveys_meaning: judgment(b),
                uses_target_structure: judgment(c),
                error: None,
                hint: None,
                explanation: None,
            }),
            calls: AtomicUsize::new(0),
        }
    }

    fn db_with_item() -> DbV2 {
        use crate::v2::generator::bank::persist_item;
        use crate::v2::generator::plan::ItemTags;
        use crate::v2::generator::BankItem;
        use crate::v2::validator::{ItemAnalysis, SlotSpec};
        let conn = Connection::open_in_memory().unwrap();
        run_migrations(&conn).unwrap();
        persist_item(
            &conn,
            &BankItem {
                id: "a".into(),
                unit_id: "opener.quiero".into(),
                source: "I want to eat.".into(),
                canonical: "Quiero comer.".into(),
                variants: vec![],
                slot: SlotSpec::default(),
                tags: ItemTags {
                    target_skill: "opener.quiero".into(),
                    stacked: vec![],
                },
                analysis: ItemAnalysis::default(),
            },
        )
        .unwrap();
        DbV2(Mutex::new(conn))
    }

    fn pending_attempt(db: &DbV2) -> String {
        let conn = db.0.lock().unwrap();
        let session = store::start_session(&conn, "opener.quiero").unwrap();
        let v = store::submit_attempt(&conn, &session, "a", "Deseo comer.").unwrap();
        assert_eq!(v.status, "pending");
        v.attempt_id
    }

    fn status_of(db: &DbV2, attempt_id: &str) -> String {
        let conn = db.0.lock().unwrap();
        conn.query_row(
            "SELECT status FROM attempts WHERE id = ?1",
            rusqlite::params![attempt_id],
            |r| r.get(0),
        )
        .unwrap()
    }

    #[tokio::test]
    async fn evaluate_pending_resolves_a_dodge_end_to_end() {
        let db = db_with_item();
        let attempt = pending_attempt(&db);
        let c = curriculum::load_embedded().unwrap();

        // Grammatical, conveys meaning, avoids the target structure.
        evaluate_pending(&db, &c, &stub(true, true, false), &attempt)
            .await
            .unwrap();
        assert_eq!(status_of(&db, &attempt), "dodge");
        let conn = db.0.lock().unwrap();
        let remarks: String = conn
            .query_row(
                "SELECT remarks FROM attempts WHERE id = ?1",
                rusqlite::params![attempt],
                |r| r.get(0),
            )
            .unwrap();
        assert!(remarks.contains("Quiero + infinitive"), "nudge names the unit: {remarks}");
    }

    #[tokio::test]
    async fn evaluator_failure_leaves_the_attempt_pending() {
        let db = db_with_item();
        let attempt = pending_attempt(&db);
        let c = curriculum::load_embedded().unwrap();

        let failing = StubEvaluator {
            result: Err("model unreachable"),
            calls: AtomicUsize::new(0),
        };
        let err = evaluate_pending(&db, &c, &failing, &attempt).await.unwrap_err();
        assert!(err.contains("model unreachable"));
        assert_eq!(status_of(&db, &attempt), "pending", "no verdict is invented");
    }

    #[tokio::test]
    async fn resolved_attempts_never_reach_the_evaluator() {
        let db = db_with_item();
        let attempt = pending_attempt(&db);
        let c = curriculum::load_embedded().unwrap();

        let evaluator = stub(true, true, true);
        evaluate_pending(&db, &c, &evaluator, &attempt).await.unwrap();
        assert_eq!(status_of(&db, &attempt), "correct");

        // A second firing finds nothing pending and never calls the model.
        evaluate_pending(&db, &c, &evaluator, &attempt).await.unwrap();
        assert_eq!(evaluator.calls.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn list_units_covers_the_authored_curriculum_in_order() {
        let conn = Connection::open_in_memory().unwrap();
        run_migrations(&conn).unwrap();
        let c = curriculum::load_embedded().unwrap();

        let rows = list_units(&conn, &c).unwrap();
        assert_eq!(rows.len(), c.units.len());
        assert_eq!(rows[0].id, c.units[0].id);
        assert!(rows.iter().all(|r| r.bank_count == 0));
        assert!(rows.iter().all(|r| r.generation_state == "idle"));
    }
}
