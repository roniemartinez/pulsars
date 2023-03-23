// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use itertools::Itertools;
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;
use tauri::{Manager, State};
use umya_spreadsheet::Spreadsheet;

struct SpreadsheetManager {
    spreadsheet: Mutex<Spreadsheet>,
}

#[derive(serde::Serialize)]
struct Sheet {
    name: String,
    rows: HashMap<u32, HashMap<String, HashMap<u32, HashMap<String, String>>>>,
}

#[tauri::command]
fn serialize(spreadsheet_manager: State<SpreadsheetManager>) -> Vec<Sheet> {
    let spreadsheet = spreadsheet_manager.spreadsheet.lock().unwrap();

    // TODO: better serialization to x-spreadsheet format
    spreadsheet
        .get_sheet_collection()
        .into_iter()
        // .map(|worksheet| worksheet.get_name())
        .map(|worksheet| {
            let grouped_by_row = worksheet
                .get_cell_collection()
                .into_iter()
                .map(|cell| {
                    let coordinate = cell.get_coordinate();
                    (
                        coordinate.get_row_num().to_owned() - 1,
                        (
                            coordinate.get_col_num().to_owned() - 1,
                            HashMap::from([("text".to_owned(), cell.get_value().to_string())]),
                        ),
                    )
                })
                .into_group_map()
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        HashMap::from([(
                            "cells".to_owned(),
                            v.into_iter()
                                .into_group_map()
                                .into_iter()
                                .map(|(k, v)| (k, v.into_iter().next().unwrap()))
                                .collect::<HashMap<u32, HashMap<String, String>>>(),
                        )]),
                    )
                })
                .collect::<HashMap<u32, HashMap<String, HashMap<u32, HashMap<String, String>>>>>();
            Sheet {
                name: worksheet.get_name().to_owned(),
                rows: grouped_by_row,
            }
        })
        .collect::<Vec<Sheet>>()
}

#[tauri::command]
fn open(file_path: &str, spreadsheet_manager: State<SpreadsheetManager>) -> Vec<Sheet> {
    let path = Path::new(file_path);
    spreadsheet_manager
        .spreadsheet
        .lock()
        .unwrap()
        .clone_from(&umya_spreadsheet::reader::xlsx::read(path).unwrap());

    serialize(spreadsheet_manager)
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
                    if cell.as_object().unwrap().is_empty() {
                        worksheet
                            .get_cell_mut((
                                column.parse::<u32>().unwrap() + 1,
                                row.parse::<u32>().unwrap() + 1,
                            ))
                            .set_value_string("");
                        continue;
                    }
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
        // .setup(|app| {
        //     #[cfg(debug_assertions)] // only include this code on debug builds
        //     {
        //         let window = app.get_window("main").unwrap();
        //         window.open_devtools();
        //         window.close_devtools();
        //     }
        //     Ok(())
        // })
        .invoke_handler(tauri::generate_handler![serialize, open, save, save_cell])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
