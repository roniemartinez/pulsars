// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;
use tauri::{Emitter, Manager, State, Wry};
use umya_spreadsheet::{CellRawValue, Color, Spreadsheet};

// https://github.com/ClosedXML/ClosedXML/wiki/Excel-Indexed-Colors
const INDEXED_COLORS: &[&str] = &[
    // Color	Color Index	Color Name
    "FF000000", // 0	Black,
    "FFFFFFFF", // 1	White,
    "FFFF0000", // 2	Red,
    "FF00FF00", // 3	Bright Green,
    "FF0000FF", // 4	Blue,
    "FFFFFF00", // 5	Yellow,
    "FFFF00FF", // 6	Pink,
    "FF00FFFF", // 7	Turquoise,
    "FF000000", // 8	Black,
    "FFFFFFFF", // 9	White,
    "FFFF0000", // 10	Red,
    "FF00FF00", // 11	Bright Green,
    "FF0000FF", // 12	Blue,
    "FFFFFF00", // 13	Yellow,
    "FFFF00FF", // 14	Pink,
    "FF00FFFF", // 15	Turquoise,
    "FF800000", // 16	Dark Red,
    "FF008000", // 17	Green,
    "FF000080", // 18	Dark Blue,
    "FF808000", // 19	Dark Yellow,
    "FF800080", // 20	Violet,
    "FF008080", // 21	Teal,
    "FFC0C0C0", // 22	Gray-25%,
    "FF808080", // 23	Gray-50%,
    "FF9999FF", // 24	Periwinkle,
    "FF993366", // 25	Plum,
    "FFFFFFCC", // 26	Ivory,
    "FFCCFFFF", // 27	Light Turquoise,
    "FF660066", // 28	Dark Purple,
    "FFFF8080", // 29	Coral,
    "FF0066CC", // 30	Ocean Blue,
    "FFCCCCFF", // 31	Ice Blue,
    "FF000080", // 32	Dark Blue,
    "FFFF00FF", // 33	Pink,
    "FFFFFF00", // 34	Yellow,
    "FF00FFFF", // 35	Turquoise,
    "FF800080", // 36	Violet,
    "FF800000", // 37	Dark Red,
    "FF008080", // 38	Teal,
    "FF0000FF", // 39	Blue,
    "FF00CCFF", // 40	Sky Blue,
    "FFCCFFFF", // 41	Light Turquoise,
    "FFCCFFCC", // 42	Light Green,
    "FFFFFF99", // 43	Light Yellow,
    "FF99CCFF", // 44	Pale Blue,
    "FFFF99CC", // 45	Rose,
    "FFCC99FF", // 46	Lavender,
    "FFFFCC99", // 47	Tan,
    "FF3366FF", // 48	Light Blue,
    "FF33CCCC", // 49	Aqua,
    "FF99CC00", // 50	Lime,
    "FFFFCC00", // 51	Gold,
    "FFFF9900", // 52	Light Orange,
    "FFFF6600", // 53	Orange,
    "FF666699", // 54	Blue-Gray,
    "FF969696", // 55	Gray-Gray40%,
    "FF003366", // 56	Dark Teal,
    "FF339966", // 57	Sea Green,
    "FF003300", // 58	Dark Green,
    "FF333300", // 59	Olive Green,
    "FF993300", // 60	Brown,
    "FF993366", // 61	Plum,
    "FF333399", // 62	Indigo,
    "FF333333", // 63	Gray-80%,
];

struct SpreadsheetManager {
    spreadsheet: Mutex<Spreadsheet>,
    sheet_map: Mutex<HashMap<String, String>>,
}

#[derive(serde::Serialize, Debug)]
struct Sheet {
    name: String,
    id: String,
    order: usize,
    celldata: Vec<CellData>,
}

#[derive(serde::Serialize, Debug)]
struct CellData {
    r: u32,
    c: u32,
    v: CellAttributes,
}

#[derive(serde::Serialize, Debug)]
struct CellAttributes {
    #[serde(default)]
    ct: CellFormat,
    #[serde(default)]
    bg: String,
    #[serde(default)]
    ff: i32,
    #[serde(default)]
    fc: String,
    #[serde(default)]
    bl: u32,
    #[serde(default)]
    it: u32,
    #[serde(default)]
    fs: f64,
    #[serde(default)]
    cl: u32,
    // #[serde(default)]
    // un: u32,
    // #[serde(default)]
    // vt: u32,
    // #[serde(default)]
    // ht: u32,
    // #[serde(default)]
    // mc: MergeCell,
    // #[serde(default)]
    // tr: u32,
    // #[serde(default)]
    // tb: u32,
    // #[serde(default)]
    v: Value,
    #[serde(default)]
    m: String,
    // #[serde(default)]
    // f: String,
}

#[derive(serde::Serialize, Debug)]
struct CellFormat {
    fa: String,
    t: String,
}

#[derive(serde::Serialize, Debug)]
struct MergeCell {
    r: u32,
    c: u32,
    rs: u32,
    cs: u32,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Op {
    op: String,
    id: String,
    path: Vec<Value>,
    #[serde(default)]
    value: Value,
}

#[tauri::command]
fn open(
    file_path: &Path,
    spreadsheet_manager: State<SpreadsheetManager>,
    app_handle: tauri::AppHandle<Wry>,
) {
    let mut spreadsheet = spreadsheet_manager.spreadsheet.lock().unwrap();
    let mut sheet_map = spreadsheet_manager.sheet_map.lock().unwrap();

    spreadsheet.clone_from(&umya_spreadsheet::reader::xlsx::read(file_path).unwrap());

    for worksheet in spreadsheet.get_sheet_collection() {
        sheet_map.insert(
            worksheet.get_sheet_id().to_string(),
            worksheet.get_name().to_string(),
        );
    }

    app_handle
        .get_webview_window("main")
        .unwrap()
        .emit("reload", {})
        .unwrap();
}

#[tauri::command]
fn save(file_path: &Path, spreadsheet_manager: State<SpreadsheetManager>) {
    let spreadsheet = spreadsheet_manager.spreadsheet.lock().unwrap();

    umya_spreadsheet::writer::xlsx::write(&spreadsheet, file_path).unwrap();
}

#[tauri::command]
fn serialize(spreadsheet_manager: State<SpreadsheetManager>) -> Vec<Sheet> {
    let spreadsheet = spreadsheet_manager.spreadsheet.lock().unwrap();
    let theme = spreadsheet.get_theme();

    spreadsheet
        .get_sheet_collection()
        .into_iter()
        .enumerate()
        .map(|(index, worksheet)| Sheet {
            name: worksheet.get_name().to_owned(),
            id: worksheet.get_sheet_id().to_owned(),
            order: index,
            celldata: worksheet
                .get_cell_collection()
                .into_iter()
                .map(|cell| {
                    let coordinate = cell.get_coordinate();

                    let m = cell.get_value();

                    let (value, format_definition, cell_type) = match cell.get_raw_value() {
                        CellRawValue::String(s) => (
                            Value::from(s.to_string()),
                            "General".to_string(),
                            "g".to_string(),
                        ),
                        CellRawValue::RichText(r) => (
                            Value::from(r.get_text()),
                            "General".to_string(),
                            "g".to_string(),
                        ),
                        CellRawValue::Numeric(n) => (
                            Value::from(n.to_owned()),
                            "General".to_string(),
                            "n".to_string(),
                        ),
                        _ => (Value::Null, "General".to_string(), "g".to_string()),
                    };

                    let background_color = match cell.get_style().get_fill() {
                        Some(fill) => match fill.get_pattern_fill() {
                            Some(pattern_fill) => match pattern_fill.get_background_color() {
                                Some(color) => {
                                    let argb = color.get_argb();
                                    if argb.is_empty() {
                                        color.get_argb_with_theme(theme).to_string()
                                    } else {
                                        "#".to_string() + &argb[2..]
                                    }
                                }
                                None => "".to_string(),
                            },
                            None => "".to_string(),
                        },
                        None => "".to_string(),
                    };

                    let (font_family, font_size, font_color, bold, italic, strike) =
                        match cell.get_style().get_font() {
                            Some(font) => {
                                // FIXME: When xlsx file is written by Numbers (Mac), the colors are different
                                let color = font.get_color();
                                let indexed_color = color.get_indexed();
                                let themed_color = color.get_theme_index();
                                let argb_color = color.get_argb();

                                let color_string: &str;
                                if indexed_color > &0 {
                                    color_string = INDEXED_COLORS[*indexed_color as usize]
                                } else if themed_color > &0 || argb_color == "" {
                                    color_string = Color::COLOR_BLACK;
                                } else {
                                    color_string = argb_color;
                                };

                                (
                                    match font.get_font_name().get_val() {
                                        "Times New Roman" => 0,
                                        "Arial" => 1,
                                        "Tahoma" => 2,
                                        "Verdana" => 3,
                                        "Microsoft Yahei" => 4,
                                        "Song" => 5,
                                        "ST Heiti" => 6,
                                        "ST Kaiti" => 7,
                                        "ST FangSong" => 8,
                                        "ST Song" => 9,
                                        "Chinese New Wei" => 10,
                                        "Chinese Xingkai" => 11,
                                        "Chinese Lishu" => 12,
                                        _ => 0,
                                    },
                                    font.get_size().to_owned(),
                                    // Remove Alpha
                                    "#".to_string()
                                        + if color_string.len() == 8 {
                                            &color_string[2..]
                                        } else {
                                            &color_string
                                        },
                                    u32::from(font.get_bold().to_owned()),
                                    u32::from(font.get_italic().to_owned()),
                                    u32::from(font.get_strikethrough().to_owned()),
                                )
                            }
                            None => (0, 0f64, "".to_string(), 0u32, 0u32, 0u32),
                        };

                    CellData {
                        r: coordinate.get_row_num().to_owned() - 1,
                        c: coordinate.get_col_num().to_owned() - 1,
                        v: CellAttributes {
                            ct: CellFormat {
                                fa: format_definition,
                                t: cell_type,
                            },
                            bg: background_color,
                            ff: font_family,
                            fc: font_color,
                            bl: bold,
                            it: italic,
                            fs: font_size,
                            cl: strike,
                            // un: 0,
                            // vt: 0,
                            // ht: 0,
                            // mc: MergeCell {
                            //     r: coordinate.get_row_num().to_owned() - 1,
                            //     c: coordinate.get_col_num().to_owned() - 1,
                            //     rs: 0,
                            //     cs: 0,
                            // },
                            // tr: 0,
                            // tb: 0,
                            v: value,
                            m: m.to_string(),
                            // f: "".to_string(),
                        },
                    }
                })
                .collect(),
        })
        .collect()
}

#[tauri::command]
fn apply_ops(ops: Vec<Op>, spreadsheet_manager: State<SpreadsheetManager>) {
    let mut spreadsheet = spreadsheet_manager.spreadsheet.lock().unwrap();
    let mut sheet_map = spreadsheet_manager.sheet_map.lock().unwrap();

    for op in ops {
        println!("{:#?}", op);
        if op.op == "addSheet" {
            let value = op.value.as_object().unwrap();
            let sheet_title = value.get("name").unwrap();
            let sheet_id = value.get("id").unwrap();

            spreadsheet.new_sheet(sheet_title.to_string()).unwrap();

            sheet_map.insert(sheet_id.to_string(), sheet_title.to_string());
        } else if (op.op == "add" || op.op == "replace") && op.path[0] == "data" {
            let row = (op.path[1].to_owned().as_u64().unwrap() + 1) as u32;
            let column = (op.path[2].to_owned().as_u64().unwrap() + 1) as u32;

            let field1 = match op.path.get(3) {
                Some(value) => value.as_str().unwrap(),
                None => "",
            };
            let field2 = match op.path.get(4) {
                Some(value) => value.as_str().unwrap(),
                None => "",
            };

            let sheet_name = sheet_map.get(&op.id).unwrap();
            let worksheet = spreadsheet.get_sheet_by_name_mut(sheet_name).unwrap();

            match (field1, field2) {
                ("ct", "fa") => {}
                ("ct", "") => {
                    let value_map = op.value.as_object().unwrap();
                    let cell_type = value_map.get("t").unwrap().to_string();

                    let cell = worksheet.get_cell_mut((column, row));

                    match cell_type.as_str() {
                        "n" => {
                            let value = cell.get_value_number().unwrap();
                            cell.set_value_number(value);
                        }
                        _ => {
                            let value = cell.get_value().to_string();
                            cell.set_value_string(value);
                        }
                    };
                }
                ("ct", _) => {}
                ("m" | "v", _) => {
                    worksheet.remove_cell((column, row));
                    worksheet
                        .get_cell_mut((column, row))
                        .set_value_string(op.value.as_str().unwrap());
                }
                ("ff", _) => {
                    worksheet
                        .get_cell_mut((column, row))
                        .get_style_mut()
                        .get_font_mut()
                        .get_font_name_mut()
                        .set_val(op.value.as_str().unwrap());
                }
                ("fs", _) => {
                    worksheet
                        .get_cell_mut((column, row))
                        .get_style_mut()
                        .get_font_mut()
                        .get_font_size_mut()
                        .set_val(op.value.as_f64().unwrap());
                }
                ("bl", _) => {
                    worksheet
                        .get_cell_mut((column, row))
                        .get_style_mut()
                        .get_font_mut()
                        .get_font_bold_mut()
                        .set_val(number_to_bool(op.value));
                }
                ("it", _) => {
                    worksheet
                        .get_cell_mut((column, row))
                        .get_style_mut()
                        .get_font_mut()
                        .get_font_italic_mut()
                        .set_val(number_to_bool(op.value));
                }
                ("cl", _) => {
                    worksheet
                        .get_cell_mut((column, row))
                        .get_style_mut()
                        .get_font_mut()
                        .get_font_strike_mut()
                        .set_val(number_to_bool(op.value));
                }
                ("", _) => {
                    let value_map = op.value.as_object().unwrap();

                    let cell_type = value_map
                        .get("ct")
                        .unwrap() // fc change will crash here
                        .as_object()
                        .unwrap()
                        .get("t")
                        .unwrap()
                        .as_str()
                        .unwrap();
                    let value = value_map.get("v").unwrap();

                    worksheet.remove_cell((column, row));

                    let cell = worksheet.get_cell_mut((column, row));

                    match cell_type {
                        "n" => {
                            let number = if value.is_number() {
                                value.as_f64().unwrap()
                            } else {
                                value.as_str().unwrap().parse::<f64>().unwrap()
                            };
                            cell.set_value_number(number);
                        }
                        _ => {
                            cell.set_value_string(value.as_str().unwrap());
                        }
                    };
                }
                _ => {}
            };
        }
    }
}

fn number_to_bool(value: Value) -> bool {
    match value {
        Value::Number(n) => match n.as_u64() {
            Some(1u64) => true,
            _ => false,
        },
        _ => false,
    }
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let spreadsheet = umya_spreadsheet::new_file();
    let mut sheet_map = HashMap::new();

    for worksheet in spreadsheet.get_sheet_collection() {
        sheet_map.insert(
            worksheet.get_sheet_id().to_string(),
            worksheet.get_name().to_string(),
        );
    }

    let state = SpreadsheetManager {
        spreadsheet: Mutex::from(spreadsheet),
        sheet_map: Mutex::from(sheet_map),
    };

    tauri::Builder::default()
        .manage(state)
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        // .setup(|app| {
        //     #[cfg(debug_assertions)] // only include this code on debug builds
        //     {
        //         let window = app.get_window("main").unwrap();
        //         window.open_devtools();
        //         window.close_devtools();
        //     }
        //
        //     Ok(())
        // })
        .invoke_handler(tauri::generate_handler![open, save, serialize, apply_ops])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
