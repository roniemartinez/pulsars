// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde_json::{json, Value};
use std::ops::Deref;
use std::path::Path;
use std::sync::Mutex;
use tauri::{Manager, State};
use umya_spreadsheet::Spreadsheet;

struct SpreadsheetManager {
    spreadsheet: Mutex<Spreadsheet>,
}

#[tauri::command]
fn serialize(spreadsheet_manager: State<SpreadsheetManager>) -> Value {
    // TODO: serialize
    let spreadsheet = spreadsheet_manager.spreadsheet.lock().unwrap();
    println!("spreadsheet {:#?}", spreadsheet.deref());

    json!([{ "name": "sheet1" }])
}

#[tauri::command]
fn save_cell(payload: &str, spreadsheet_manager: State<SpreadsheetManager>) {
    let data: Value = serde_json::from_str(payload).unwrap();
    println!("data {}", data);

    let mut spreadsheet = spreadsheet_manager.spreadsheet.lock().unwrap();

    let worksheet;
    if let Ok(sheet) = spreadsheet.get_sheet_by_name_mut(data["name"].as_str().unwrap()) {
        worksheet = sheet;
    } else {
        worksheet = spreadsheet
            .new_sheet(data["name"].as_str().unwrap())
            .unwrap();
    }

    for (row, value) in data["rows"].as_object().unwrap() {
        if row == "len" {
            continue;
        }
        println!("rows: {}, {}", row, value);
        for (key, columns) in value.as_object().unwrap() {
            if key == "cells" {
                for (column, cell) in columns.as_object().unwrap() {
                    for (cell_type, value) in cell.as_object().unwrap() {
                        if cell_type == "text" {
                            worksheet
                                .get_cell_mut((
                                    column.parse::<u32>().unwrap() + 1,
                                    row.parse::<u32>().unwrap() + 1,
                                ))
                                .set_value_string(value.as_str().unwrap());
                        }
                    }
                }
            }
        }
    }
}

#[tauri::command]
fn save(file_path: &str, spreadsheet_manager: State<SpreadsheetManager>) {
    let path = Path::new(file_path);
    let mut spreadsheet = spreadsheet_manager.spreadsheet.lock().unwrap();

    umya_spreadsheet::writer::xlsx::write(&spreadsheet, path).unwrap();
}

fn main() {
    tauri::Builder::default()
        .manage(SpreadsheetManager {
            spreadsheet: Mutex::from(umya_spreadsheet::new_file_empty_worksheet()),
        })
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![serialize, save, save_cell])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
