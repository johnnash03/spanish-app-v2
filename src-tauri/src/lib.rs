mod db;
mod generate;
mod openai;
mod session;

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

            // App-open pre-warm: generate banks for the nearest idle units in background.
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let _ = generate::prewarm_units_internal(app_handle).await;
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            db::db_health,
            openai::openai_ping,
            generate::trigger_generation,
            generate::get_unit_generation_state,
            generate::retry_generation,
            generate::prewarm_units,
            session::assemble_session_queue,
            session::submit_session_attempts,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
