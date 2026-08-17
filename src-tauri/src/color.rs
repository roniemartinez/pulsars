use umya_spreadsheet::{Color, drawing::Theme};

/// Converts a workbook color to the `#RRGGBB` string fortune-sheet expects.
///
/// `argb_with_theme` handles indexed, themed and literal colors, but returns
/// 8-char `AARRGGBB` for the first and last and 6-char for the themed case.
pub fn to_hex(color: &Color, theme: &Theme) -> String {
    let hex = color.argb_with_theme(theme);
    let rgb = if hex.len() == 8 { &hex[2..] } else { &hex[..] };
    format!("#{rgb}")
}
