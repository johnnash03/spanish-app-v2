//! Tauri entry points for background bank generation (S5, #36).
//! Generation is never invoked mid-session: visiting a unit triggers it,
//! and the adjacent unit is prefetched silently (user story 48, v1
//! behavior carried over).

use super::bank::{self, SqliteBankSink};
use super::pipeline::{generate_unit_bank, PipelineConfig};
use super::plan::LearnerSnapshot;
use super::source::OpenAiItemSource;
use crate::v2::curriculum::{Curriculum, CurriculumState};
use crate::v2::db::DbV2;
use crate::v2::validator::OpenAiAnalyzer;
use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};
use tauri::{AppHandle, Manager};

/// In-process guard against double generation of one unit (v1 behavior).
static GENERATING_UNITS: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

fn generating_units() -> &'static Mutex<HashSet<String>> {
    GENERATING_UNITS.get_or_init(|| Mutex::new(HashSet::new()))
}

/// The unit after `unit_id` in authored curriculum order — the prefetch
/// target.
pub fn next_unit_id(c: &Curriculum, unit_id: &str) -> Option<String> {
    let pos = c.units.iter().position(|u| u.id == unit_id)?;
    c.units.get(pos + 1).map(|u| u.id.clone())
}

/// Runs the full pipeline for one unit in the background. Prefetch
/// failures are silent (state returns to idle so a direct visit retries);
/// direct failures surface as the "failed" state.
async fn run_generation(app: AppHandle, unit_id: String, is_prefetch: bool) {
    if !generating_units().lock().unwrap().insert(unit_id.clone()) {
        return;
    }
    let outcome = run_generation_inner(&app, &unit_id).await;
    generating_units().lock().unwrap().remove(&unit_id);

    let db = app.state::<DbV2>();
    let conn = db.0.lock().unwrap();
    match outcome {
        Ok(banked) if banked > 0 => {
            let _ = bank::set_generation_state(&conn, &unit_id, "ready");
        }
        Ok(_) | Err(_) => {
            let state = if is_prefetch { "idle" } else { "failed" };
            if let Err(e) = &outcome {
                eprintln!("[gen {unit_id}] generation failed: {e}");
            }
            let _ = bank::set_generation_state(&conn, &unit_id, state);
        }
    }
}

async fn run_generation_inner(app: &AppHandle, unit_id: &str) -> Result<usize, String> {
    let source = OpenAiItemSource::from_env().map_err(|e| e.to_string())?;
    let analyzer = OpenAiAnalyzer::from_env().map_err(|e| e.to_string())?;
    let curriculum = app.state::<CurriculumState>().0.clone();

    let (existing, existing_sources) = {
        let db = app.state::<DbV2>();
        let conn = db.0.lock().map_err(|e| e.to_string())?;
        (
            bank::existing_items(&conn, unit_id).map_err(|e| e.to_string())?,
            bank::existing_sources(&conn, unit_id).map_err(|e| e.to_string())?,
        )
    };

    let db = app.state::<DbV2>();
    let sink = SqliteBankSink(&db);
    // Learner state wiring arrives with the Words track (S10/S11); until
    // then every item drills under the structure-unknown partition.
    let learner = LearnerSnapshot::default();
    let outcome = generate_unit_bank(
        &source,
        &analyzer,
        &sink,
        &curriculum,
        unit_id,
        &learner,
        existing,
        existing_sources,
        &PipelineConfig::default(),
    )
    .await
    .map_err(|e| e.to_string())?;
    eprintln!(
        "[gen {unit_id}] done: {} banked, {} abandoned, {} round(s)",
        outcome.banked,
        outcome.abandoned.len(),
        outcome.rounds
    );
    Ok(outcome.banked)
}

/// Checks the unit's bank; if empty and idle (or failed), kicks off
/// background generation, and prefetches the adjacent unit. Returns the
/// current generation state.
#[tauri::command]
pub async fn v2_trigger_generation(
    app: AppHandle,
    unit_id: String,
) -> Result<String, String> {
    let curriculum = app.state::<CurriculumState>().0.clone();
    if curriculum.unit(&unit_id).is_none() {
        return Err(format!("unknown unit `{unit_id}`"));
    }

    let (state, count) = {
        let db = app.state::<DbV2>();
        let conn = db.0.lock().map_err(|e| e.to_string())?;
        (
            bank::generation_state(&conn, &unit_id),
            bank::bank_count(&conn, &unit_id).map_err(|e| e.to_string())?,
        )
    };

    if (state == "idle" && count == 0) || state == "failed" {
        {
            let db = app.state::<DbV2>();
            let conn = db.0.lock().map_err(|e| e.to_string())?;
            bank::set_generation_state(&conn, &unit_id, "generating").map_err(|e| e.to_string())?;
        }
        let app_clone = app.clone();
        let unit = unit_id.clone();
        tauri::async_runtime::spawn(async move {
            run_generation(app_clone, unit, false).await;
        });
    }

    // Adjacent-unit prefetch (silent, only when untouched).
    if let Some(next) = next_unit_id(&curriculum, &unit_id) {
        let should_prefetch = {
            let db = app.state::<DbV2>();
            let conn = db.0.lock().map_err(|e| e.to_string())?;
            bank::generation_state(&conn, &next) == "idle"
                && bank::bank_count(&conn, &next).map_err(|e| e.to_string())? == 0
        };
        if should_prefetch {
            {
                let db = app.state::<DbV2>();
                let conn = db.0.lock().map_err(|e| e.to_string())?;
                bank::set_generation_state(&conn, &next, "generating")
                    .map_err(|e| e.to_string())?;
            }
            let app_clone = app.clone();
            tauri::async_runtime::spawn(async move {
                run_generation(app_clone, next, true).await;
            });
        }
    }

    let db = app.state::<DbV2>();
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    Ok(bank::generation_state(&conn, &unit_id))
}

/// Poll the generation state of a unit.
#[tauri::command]
pub fn v2_generation_state(
    state: tauri::State<'_, DbV2>,
    unit_id: String,
) -> Result<String, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    Ok(bank::generation_state(&conn, &unit_id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2::curriculum;

    #[test]
    fn next_unit_follows_authored_curriculum_order() {
        let c = curriculum::load_embedded().unwrap();
        assert_eq!(
            next_unit_id(&c, "opener.quiero").as_deref(),
            Some("opener.quiero.neg")
        );
        assert_eq!(
            next_unit_id(&c, "opener.mixed").as_deref(),
            Some("clitic.do.sg.attach"),
            "prefetch crosses phase boundaries"
        );
        let last = &c.units.last().unwrap().id;
        assert_eq!(next_unit_id(&c, last), None);
        assert_eq!(next_unit_id(&c, "nope"), None);
    }
}
