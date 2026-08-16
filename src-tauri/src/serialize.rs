use crate::color::to_hex;
use crate::model::{CellAttributes, CellData, CellFormat, Sheet};
use serde_json::Value;
use umya_spreadsheet::{Cell, CellRawValue, Font, Workbook, drawing::Theme};

/// fortune-sheet identifies fonts by index, not by name.
const FONT_FAMILIES: &[&str] = &[
    "Times New Roman",
    "Arial",
    "Tahoma",
    "Verdana",
    "Microsoft Yahei",
    "Song",
    "ST Heiti",
    "ST Kaiti",
    "ST FangSong",
    "ST Song",
    "Chinese New Wei",
    "Chinese Xingkai",
    "Chinese Lishu",
];

struct FontStyle {
    family: u32,
    size: f64,
    color: String,
    bold: u8,
    italic: u8,
    strike: u8,
}

impl Default for FontStyle {
    fn default() -> Self {
        Self { family: 0, size: 0.0, color: String::new(), bold: 0, italic: 0, strike: 0 }
    }
}

pub fn to_sheets(workbook: &Workbook) -> Vec<Sheet> {
    let theme = workbook.theme();

    workbook
        .sheet_collection()
        .iter()
        .enumerate()
        .map(|(order, worksheet)| Sheet {
            name: worksheet.name().to_owned(),
            id: worksheet.sheet_id().to_owned(),
            order,
            celldata: worksheet.cells().into_iter().map(|cell| to_cell_data(cell, theme)).collect(),
        })
        .collect()
}

fn to_cell_data(cell: &Cell, theme: &Theme) -> CellData {
    let coordinate = cell.coordinate();
    let (value, cell_type) = to_value(cell.raw_value());
    let style = cell.style();

    let font = style.font().map(|font| to_font_style(font, theme));
    let font = font.unwrap_or_default();

    CellData {
        // fortune-sheet is zero-based; umya-spreadsheet is one-based.
        r: coordinate.row_num() - 1,
        c: coordinate.col_num() - 1,
        v: CellAttributes {
            ct: CellFormat { fa: "General".to_owned(), t: cell_type.to_owned() },
            bg: to_background(cell, theme),
            ff: font.family,
            fc: font.color,
            bl: font.bold,
            it: font.italic,
            fs: font.size,
            cl: font.strike,
            v: value,
            m: cell.value().to_string(),
        },
    }
}

/// Returns the raw value and fortune-sheet's type tag: "n" numeric, "g" general.
fn to_value(raw: &CellRawValue) -> (Value, &'static str) {
    match raw {
        CellRawValue::String(text) => (Value::from(text.to_string()), "g"),
        CellRawValue::Lazy(text) => (Value::from(text.to_string()), "g"),
        CellRawValue::RichText(rich) => (Value::from(rich.text().to_string()), "g"),
        CellRawValue::Numeric(number) => (Value::from(*number), "n"),
        CellRawValue::Bool(value) => (Value::from(*value), "g"),
        CellRawValue::Error(_) | CellRawValue::Empty => (Value::Null, "g"),
    }
}

/// An empty string means the cell has no background.
fn to_background(cell: &Cell, theme: &Theme) -> String {
    cell.style()
        .fill()
        .and_then(|fill| fill.pattern_fill())
        .and_then(|pattern| pattern.background_color())
        .map(|color| to_hex(color, theme))
        .unwrap_or_default()
}

fn to_font_style(font: &Font, theme: &Theme) -> FontStyle {
    FontStyle {
        family: FONT_FAMILIES.iter().position(|name| *name == font.font_name().val()).unwrap_or(0) as u32,
        size: font.size(),
        color: to_hex(font.color(), theme),
        bold: u8::from(font.bold()),
        italic: u8::from(font.italic()),
        strike: u8::from(font.strikethrough()),
    }
}
