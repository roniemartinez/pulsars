use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A worksheet in the shape fortune-sheet expects.
#[derive(Serialize, Debug)]
pub struct Sheet {
    pub name: String,
    pub id: String,
    pub order: usize,
    pub celldata: Vec<CellData>,
}

/// A single cell, addressed by zero-based row and column.
#[derive(Serialize, Debug)]
pub struct CellData {
    pub r: u32,
    pub c: u32,
    pub v: CellAttributes,
}

/// Mirrors fortune-sheet's `Cell` type. The short names are theirs, not ours.
#[derive(Serialize, Debug)]
pub struct CellAttributes {
    /// Cell format.
    pub ct: CellFormat,
    /// Background color.
    pub bg: String,
    /// Font family, as an index into fortune-sheet's font list.
    pub ff: u32,
    /// Font color.
    pub fc: String,
    /// Bold.
    pub bl: u8,
    /// Italic.
    pub it: u8,
    /// Font size.
    pub fs: f64,
    /// Strikethrough.
    pub cl: u8,
    /// Raw value.
    pub v: Value,
    /// Displayed value.
    pub m: String,
}

#[derive(Serialize, Debug)]
pub struct CellFormat {
    /// Format definition.
    pub fa: String,
    /// Type: "g" for general, "n" for numeric.
    pub t: String,
}

/// Mirrors fortune-sheet's `Op["op"]` union.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum OpKind {
    Add,
    Remove,
    Replace,
    InsertRowCol,
    DeleteRowCol,
    AddSheet,
    DeleteSheet,
}

/// A single change emitted by the frontend. `id` is optional upstream.
#[derive(Serialize, Deserialize, Debug)]
pub struct Op {
    pub op: OpKind,
    #[serde(default)]
    pub id: String,
    pub path: Vec<Value>,
    #[serde(default)]
    pub value: Value,
}

impl Op {
    pub fn segment(&self, index: usize) -> &str {
        self.path.get(index).and_then(Value::as_str).unwrap_or_default()
    }
}
