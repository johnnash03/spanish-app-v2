mod combined;
mod db;
mod deliberate_practice;
mod generate;
mod mastery;
mod openai;
mod session;
mod srs;
mod units;
mod vocab;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Load .env from the src-tauri dir if present (dev only; ignored in release).
    let _ = dotenvy::dotenv();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let mut db_path = app.path().app_data_dir()?;
            db_path.push("spanish-app.db");
            let db = db::Db::open(&db_path)
                .map_err(|e| format!("failed to open db at {:?}: {}", db_path, e))?;
            app.manage(db);

            // App-open pre-warm disabled during testing — generate on visit only.
            // let app_handle = app.handle().clone();
            // tauri::async_runtime::spawn(async move {
            //     let _ = generate::prewarm_units_internal(app_handle).await;
            // });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            db::db_health,
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
