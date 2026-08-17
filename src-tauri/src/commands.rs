use crate::error::{Error, Result};
use crate::model::{Op, Sheet};
use crate::state::SpreadsheetManager;
use crate::{ops, serialize};
use std::path::Path;
use tauri::{AppHandle, Emitter, Manager, State, Wry};

/// Loads a workbook from disk and tells the frontend to reload.
#[tauri::command]
pub fn open(
    file_path: &Path,
    spreadsheet_manager: State<SpreadsheetManager>,
    app_handle: AppHandle<Wry>,
) -> Result<()> {
    let workbook = umya_spreadsheet::reader::xlsx::read(file_path).map_err(|error| Error::Read(error.to_string()))?;

    spreadsheet_manager.state.lock().unwrap().replace(workbook);

    app_handle
        .get_webview_window("main")
        .ok_or_else(|| Error::Read("main window is gone".to_owned()))?
        .emit("reload", ())
        .map_err(|error| Error::Read(error.to_string()))
}

/// Writes the current workbook to disk.
#[tauri::command]
pub fn save(file_path: &Path, spreadsheet_manager: State<SpreadsheetManager>) -> Result<()> {
    let state = spreadsheet_manager.state.lock().unwrap();

    umya_spreadsheet::writer::xlsx::write(&state.workbook, file_path).map_err(|error| Error::Write(error.to_string()))
}

/// Returns the workbook in the shape fortune-sheet renders.
#[tauri::command]
pub fn serialize(spreadsheet_manager: State<SpreadsheetManager>) -> Vec<Sheet> {
    let state = spreadsheet_manager.state.lock().unwrap();

    serialize::to_sheets(&state.workbook)
}

/// Applies edits made in the frontend back onto the workbook.
#[tauri::command]
pub fn apply_ops(ops: Vec<Op>, spreadsheet_manager: State<SpreadsheetManager>) -> Result<()> {
    let mut state = spreadsheet_manager.state.lock().unwrap();
    let state = &mut *state;

    ops::apply(&mut state.workbook, &mut state.sheet_map, ops)
}
