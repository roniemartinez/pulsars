use crate::error::{Error, Result};
use crate::model::{Op, Sheet};
use crate::state::{SpreadsheetManager, sheet_map_of};
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

    *spreadsheet_manager.sheet_map.lock().unwrap() = sheet_map_of(&workbook);
    *spreadsheet_manager.workbook.lock().unwrap() = workbook;

    app_handle
        .get_webview_window("main")
        .ok_or_else(|| Error::Read("main window is gone".to_owned()))?
        .emit("reload", ())
        .map_err(|error| Error::Read(error.to_string()))
}

/// Writes the current workbook to disk.
#[tauri::command]
pub fn save(file_path: &Path, spreadsheet_manager: State<SpreadsheetManager>) -> Result<()> {
    let workbook = spreadsheet_manager.workbook.lock().unwrap();

    umya_spreadsheet::writer::xlsx::write(&workbook, file_path).map_err(|error| Error::Write(error.to_string()))
}

/// Returns the workbook in the shape fortune-sheet renders.
#[tauri::command]
pub fn serialize(spreadsheet_manager: State<SpreadsheetManager>) -> Vec<Sheet> {
    let workbook = spreadsheet_manager.workbook.lock().unwrap();

    serialize::to_sheets(&workbook)
}

/// Applies edits made in the frontend back onto the workbook.
#[tauri::command]
pub fn apply_ops(ops: Vec<Op>, spreadsheet_manager: State<SpreadsheetManager>) -> Result<()> {
    let mut workbook = spreadsheet_manager.workbook.lock().unwrap();
    let mut sheet_map = spreadsheet_manager.sheet_map.lock().unwrap();

    ops::apply(&mut workbook, &mut sheet_map, ops)
}
