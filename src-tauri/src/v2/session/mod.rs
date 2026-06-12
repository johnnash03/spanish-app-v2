//! V2 practice session loop (S6, #37): one-at-a-time serving with eager
//! per-item resolution. Tier 0 verdicts resolve in-process — instant and
//! offline — and every attempt lands in the v2 attempt log the moment it
//! is submitted, so an interrupted session loses nothing (user stories 8,
//! 9, 53, 54, 55).

pub mod store;

use crate::v2::curriculum::{Curriculum, CurriculumState};
use crate::v2::db::DbV2;
use crate::v2::generator::bank;
use rusqlite::Connection;
use serde::Serialize;
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

/// Eager per-item resolution: Tier 0 runs and the attempt persists before
/// this returns. No network is touched on this path.
#[tauri::command]
pub fn v2_submit_attempt(
    db: tauri::State<'_, DbV2>,
    session_id: String,
    item_id: String,
    answer: String,
) -> Result<AttemptVerdict, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    store::submit_attempt(&conn, &session_id, &item_id, &answer)
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
