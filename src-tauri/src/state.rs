use std::collections::HashMap;
use std::sync::Mutex;
use umya_spreadsheet::Workbook;

/// The frontend addresses sheets by id, umya-spreadsheet by name.
pub struct SpreadsheetManager {
    pub workbook: Mutex<Workbook>,
    pub sheet_map: Mutex<HashMap<String, String>>,
}

impl SpreadsheetManager {
    pub fn new(workbook: Workbook) -> Self {
        let sheet_map = sheet_map_of(&workbook);
        Self { workbook: Mutex::new(workbook), sheet_map: Mutex::new(sheet_map) }
    }
}

pub fn sheet_map_of(workbook: &Workbook) -> HashMap<String, String> {
    workbook
        .sheet_collection()
        .iter()
        .map(|worksheet| (worksheet.sheet_id().to_string(), worksheet.name().to_string()))
        .collect()
}
