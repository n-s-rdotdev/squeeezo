mod commands;
mod state;

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let persistent_state = state::PersistentStateHandle::load(app.handle())?;
            app.manage(persistent_state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::analyze_pdf,
            commands::compress_pdf,
            commands::get_recent_jobs,
            commands::clear_recent_jobs,
            commands::reveal_in_folder,
            commands::open_file,
            commands::get_settings,
            commands::update_settings,
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Squeeezo desktop app")
}
