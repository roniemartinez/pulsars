mod color;
mod commands;
mod error;
mod model;
mod ops;
mod serialize;
mod state;

use state::SpreadsheetManager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(SpreadsheetManager::new(umya_spreadsheet::new_file()))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::open,
            commands::save,
            commands::serialize,
            commands::apply_ops
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
