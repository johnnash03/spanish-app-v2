mod db;
mod openai;

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
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            db::db_health,
            openai::openai_ping
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
