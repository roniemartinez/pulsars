use std::collections::HashMap;
use std::sync::Mutex;
use umya_spreadsheet::Workbook;

/// The frontend addresses sheets by id, umya-spreadsheet by name.
pub struct WorkbookState {
    pub workbook: Workbook,
    pub sheet_map: HashMap<String, String>,
}

impl WorkbookState {
    pub fn new(workbook: Workbook) -> Self {
        let sheet_map = sheet_map_of(&workbook);
        Self { workbook, sheet_map }
    }

    pub fn replace(&mut self, workbook: Workbook) {
        self.sheet_map = sheet_map_of(&workbook);
        self.workbook = workbook;
    }
}

/// One mutex: separate locks would let a reader pair a new sheet map with an
/// old workbook.
pub struct SpreadsheetManager {
    pub state: Mutex<WorkbookState>,
}

impl SpreadsheetManager {
    pub fn new(workbook: Workbook) -> Self {
        Self { state: Mutex::new(WorkbookState::new(workbook)) }
    }
}

fn sheet_map_of(workbook: &Workbook) -> HashMap<String, String> {
    workbook
        .sheet_collection()
        .iter()
        .map(|worksheet| (worksheet.sheet_id().to_string(), worksheet.name().to_string()))
        .collect()
}
