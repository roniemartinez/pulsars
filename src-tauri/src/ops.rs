use crate::error::{Error, Result};
use crate::model::{Op, OpKind};
use serde_json::Value;
use std::collections::HashMap;
use umya_spreadsheet::{Workbook, Worksheet};

/// Applies frontend edits back onto the workbook. Unmodelled ops are skipped.
pub fn apply(workbook: &mut Workbook, sheet_map: &mut HashMap<String, String>, ops: Vec<Op>) -> Result<()> {
    for op in ops {
        match op.op {
            OpKind::AddSheet => add_sheet(workbook, sheet_map, &op)?,
            OpKind::Add | OpKind::Replace if op.segment(0) == "data" => {
                apply_cell_change(workbook, sheet_map, &op)?;
            }
            _ => {}
        }
    }

    Ok(())
}

fn add_sheet(workbook: &mut Workbook, sheet_map: &mut HashMap<String, String>, op: &Op) -> Result<()> {
    let value = op.value.as_object().ok_or_else(|| Error::MalformedOp("addSheet has no value object".to_owned()))?;

    let name = value
        .get("name")
        .and_then(Value::as_str)
        .ok_or_else(|| Error::MalformedOp("addSheet has no name".to_owned()))?;
    let id =
        value.get("id").and_then(Value::as_str).ok_or_else(|| Error::MalformedOp("addSheet has no id".to_owned()))?;

    workbook.new_sheet(name).map_err(|error| Error::MalformedOp(error.to_string()))?;
    sheet_map.insert(id.to_owned(), name.to_owned());

    Ok(())
}

fn apply_cell_change(workbook: &mut Workbook, sheet_map: &HashMap<String, String>, op: &Op) -> Result<()> {
    let (column, row) = coordinate(op)?;

    let sheet_name = sheet_map.get(&op.id).ok_or_else(|| Error::UnknownSheet(op.id.clone()))?;
    let worksheet = workbook.sheet_by_name_mut(sheet_name).map_err(|_| Error::UnknownSheet(sheet_name.clone()))?;

    match (op.segment(3), op.segment(4)) {
        ("ct", "") => retype_cell(worksheet, column, row, &op.value),
        ("ct", _) => {}
        ("m" | "v", _) => {
            let text = as_text(&op.value);
            worksheet.cell_mut((column, row)).set_value_string(text);
        }
        ("ff", _) => {
            worksheet
                .cell_mut((column, row))
                .style_mut()
                .font_mut()
                .font_name_mut()
                .set_val(op.value.as_str().unwrap_or_default());
        }
        ("fs", _) => {
            if let Some(size) = op.value.as_f64() {
                worksheet.cell_mut((column, row)).style_mut().font_mut().font_size_mut().set_val(size);
            }
        }
        ("bl", _) => {
            worksheet.cell_mut((column, row)).style_mut().font_mut().font_bold_mut().set_val(is_enabled(&op.value));
        }
        ("it", _) => {
            worksheet.cell_mut((column, row)).style_mut().font_mut().font_italic_mut().set_val(is_enabled(&op.value));
        }
        ("cl", _) => {
            worksheet.cell_mut((column, row)).style_mut().font_mut().font_strike_mut().set_val(is_enabled(&op.value));
        }
        ("", _) => replace_cell(worksheet, column, row, &op.value)?,
        _ => {}
    }

    Ok(())
}

/// Path segments 1 and 2 are the zero-based row and column; umya-spreadsheet is
/// one-based, so both shift by one.
fn coordinate(op: &Op) -> Result<(u32, u32)> {
    let read = |index: usize| {
        op.path
            .get(index)
            .and_then(Value::as_u64)
            .and_then(|value| u32::try_from(value.checked_add(1)?).ok())
            .ok_or_else(|| Error::MalformedOp(format!("path segment {index} is not a valid coordinate")))
    };

    Ok((read(2)?, read(1)?))
}

/// Rewrites the stored value so its type matches the cell's new format.
fn retype_cell(worksheet: &mut Worksheet, column: u32, row: u32, value: &Value) {
    let is_numeric = value.as_object().and_then(|map| map.get("t")).and_then(Value::as_str) == Some("n");

    let cell = worksheet.cell_mut((column, row));

    if is_numeric {
        let number = cell.value_number().or_else(|| cell.value().parse::<f64>().ok());

        if let Some(number) = number {
            cell.set_value_number(number);
        }
    } else {
        let text = cell.value().to_string();
        cell.set_value_string(text);
    }
}

/// Replaces a cell wholesale, as emitted when value and format change together.
fn replace_cell(worksheet: &mut Worksheet, column: u32, row: u32, value: &Value) -> Result<()> {
    let map = value.as_object().ok_or_else(|| Error::MalformedOp("cell replacement has no value object".to_owned()))?;

    // Style-only changes arrive without a format, and carry no value to store.
    let Some(cell_type) =
        map.get("ct").and_then(Value::as_object).and_then(|format| format.get("t")).and_then(Value::as_str)
    else {
        return Ok(());
    };

    let Some(new_value) = map.get("v") else {
        return Ok(());
    };

    let cell = worksheet.cell_mut((column, row));

    if cell_type == "n" {
        let number = match new_value {
            Value::Number(number) => number.as_f64(),
            Value::String(text) => text.parse::<f64>().ok(),
            _ => None,
        };

        match number {
            Some(number) => cell.set_value_number(number),
            None => cell.set_value_string(as_text(new_value)),
        };
    } else {
        cell.set_value_string(as_text(new_value));
    }

    Ok(())
}

/// fortune-sheet encodes these toggles as 1 or 0 rather than as booleans.
fn is_enabled(value: &Value) -> bool {
    value.as_u64() == Some(1)
}

/// Renders a JSON scalar as cell text.
///
/// `Value::to_string` quotes strings, so `foo` would be stored as `"foo"`.
fn as_text(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::{Error, HashMap, Op, OpKind, Value, apply, as_text};
    use serde_json::json;

    #[test]
    fn string_values_are_unquoted() {
        assert_eq!(as_text(&json!("foo")), "foo");
        assert_eq!(as_text(&json!("")), "");
    }

    #[test]
    fn numbers_survive_a_text_path() {
        assert_eq!(as_text(&json!(42)), "42");
        assert_eq!(as_text(&json!(12.5)), "12.5");
        assert_eq!(as_text(&json!(-3)), "-3");
    }

    #[test]
    fn booleans_render_without_quotes() {
        assert_eq!(as_text(&json!(true)), "true");
        assert_eq!(as_text(&json!(false)), "false");
    }

    #[test]
    fn null_becomes_empty() {
        assert_eq!(as_text(&json!(null)), "");
    }

    fn sheet_map() -> HashMap<String, String> {
        HashMap::from([("1".to_owned(), "Sheet1".to_owned())])
    }

    fn value_op(path: Vec<Value>, value: Value) -> Op {
        Op { op: OpKind::Replace, id: "1".to_owned(), path, value }
    }

    #[test]
    fn editing_a_value_keeps_the_cell_style() {
        let mut workbook = umya_spreadsheet::new_file();
        {
            let cell = workbook.sheet_by_name_mut("Sheet1").unwrap().cell_mut((1u32, 1u32));
            cell.set_value_number(42.0);
            cell.style_mut().set_background_color("FFFF0000");
            cell.style_mut().font_mut().font_bold_mut().set_val(true);
        }

        let op = value_op(vec![json!("data"), json!(0), json!(0), json!("v")], json!("hello"));
        apply(&mut workbook, &mut sheet_map(), vec![op]).unwrap();

        let cell = workbook.sheet_by_name_mut("Sheet1").unwrap().cell_mut((1u32, 1u32));
        assert_eq!(cell.value(), "hello");
        assert!(cell.style().fill().is_some(), "background was dropped");
        assert_eq!(cell.style().font().map(|font| font.bold()), Some(true), "bold was dropped");
    }

    #[test]
    fn out_of_range_coordinates_are_rejected() {
        let mut workbook = umya_spreadsheet::new_file();
        let path = vec![json!("data"), json!(u64::from(u32::MAX) + 1), json!(0), json!("v")];
        let op = value_op(path, json!("x"));

        let result = apply(&mut workbook, &mut sheet_map(), vec![op]);

        assert!(matches!(result, Err(Error::MalformedOp(_))), "expected rejection, got {result:?}");
    }

    #[test]
    fn retyping_parses_text_into_a_number() {
        let mut workbook = umya_spreadsheet::new_file();
        workbook.sheet_by_name_mut("Sheet1").unwrap().cell_mut((1u32, 1u32)).set_value_string("42");

        let op = value_op(vec![json!("data"), json!(0), json!(0), json!("ct")], json!({ "t": "n" }));
        apply(&mut workbook, &mut sheet_map(), vec![op]).unwrap();

        let cell = workbook.sheet_by_name_mut("Sheet1").unwrap().cell_mut((1u32, 1u32));
        assert_eq!(cell.value_number(), Some(42.0));
    }
}
