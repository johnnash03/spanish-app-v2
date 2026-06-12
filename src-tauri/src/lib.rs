mod legacy;
mod openai;
// Public so dev binaries (`cargo run --bin dump_licensing`) can reuse the
// curriculum loader.
pub mod v2;

use legacy::{combined, db, deliberate_practice, generate, mastery, session, srs, units, vocab};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Load .env from the src-tauri dir if present (dev only; ignored in release).
    let _ = dotenvy::dotenv();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let data_dir = app.path().app_data_dir()?;

            // Legacy v1 database — untouched schema, still serves the v1 UI
            // until the v2 home lands (S14).
            let mut db_path = data_dir.clone();
            db_path.push("spanish-app.db");
            let db = db::Db::open(&db_path)
                .map_err(|e| format!("failed to open db at {:?}: {}", db_path, e))?;
            app.manage(db);

            // V2 database — separate file, separate schema. Both DBs are live
            // side by side for the duration of the strangler-fig rewrite.
            let mut db_v2_path = data_dir;
            db_v2_path.push("spanish-app-v2.db");
            let db_v2 = v2::db::DbV2::open(&db_v2_path)
                .map_err(|e| format!("failed to open v2 db at {:?}: {}", db_v2_path, e))?;

            // Load and validate the v2 curriculum. A validation failure
            // (DAG cycle, non-monotonic licensing, bad reference) is fatal
            // by design: guarantees live in code, and the app must not run
            // on a curriculum that fails them.
            let curriculum = v2::curriculum::load_embedded()
                .map_err(|e| format!("v2 curriculum failed validation: {}", e))?;
            {
                let mut conn = db_v2.0.lock().map_err(|e| e.to_string())?;
                v2::curriculum::store::persist(&mut conn, &curriculum)
                    .map_err(|e| format!("failed to persist v2 curriculum: {}", e))?;
            }
            app.manage(v2::curriculum::CurriculumState(std::sync::Arc::new(
                curriculum,
            )));
            app.manage(db_v2);

            // App-open pre-warm disabled during testing — generate on visit only.
            // let app_handle = app.handle().clone();
            // tauri::async_runtime::spawn(async move {
            //     let _ = generate::prewarm_units_internal(app_handle).await;
            // });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            db::db_health,
            v2::db::db_v2_health,
            v2::curriculum::dump_effective_licensing,
            v2::generator::commands::v2_trigger_generation,
            v2::generator::commands::v2_generation_state,
            v2::session::v2_list_units,
            v2::session::v2_start_session,
            v2::session::v2_submit_attempt,
            v2::session::v2_end_session,
            db::wipe_exercise_items,
            openai::openai_ping,
            generate::trigger_generation,
            generate::get_unit_generation_state,
            generate::retry_generation,
            generate::prewarm_units,
            session::assemble_session_queue,
            session::submit_session_attempts,
            session::evaluate_session,
            units::list_units,
            units::get_unit_by_n,
            units::get_current_unit_number,
            mastery::get_weak_tags,
            deliberate_practice::assemble_dp_queue,
            deliberate_practice::trigger_dp_generation,
            session::get_pending_session,
            vocab::get_next_untouched_words,
            vocab::commit_intake_batch,
            vocab::get_pipeline_health,
            vocab::mark_vocab_word_mastered,
            srs::get_due_vocab_cards,
            srs::get_vocab_session_cards,
            srs::record_vocab_review,
            combined::get_combined_exercises,
            combined::trigger_combined_replenishment,
            combined::get_combined_pool_size,
            combined::submit_combined_exercise_result,
            combined::assemble_combined_queue,
            combined::record_combined_session_reviews,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
