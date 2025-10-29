use calamine::Reader;
use serde::{Deserialize, Serialize};
use std::io::{Read, Seek};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CellAddress {
    pub sheet: String,
    pub cell: String,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ExcelCoord {
    pub row: u32,
    pub col: u32,
    pub base: (u32, u32),
}

impl From<ExcelCoord> for (u32, u32) {
    fn from(coord: ExcelCoord) -> Self {
        (coord.row + coord.base.0, coord.col + coord.base.1)
    }
}

impl From<(u32, u32)> for ExcelCoord {
    fn from((row, col): (u32, u32)) -> Self {
        Self {
            row,
            col,
            base: (0, 0),
        }
    }
}

impl std::fmt::Display for ExcelCoord {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "({}, {})", self.row, self.col)
    }
}

impl ExcelCoord {
    pub fn new(row: u32, col: u32) -> Self {
        Self {
            row,
            col,
            base: (0, 0),
        }
    }

    pub fn from_a1_style(coord: &str) -> Option<Self> {
        if coord.len() < 2 {
            return None;
        }

        let mut col = 0;
        let mut row = 0;
        for c in coord.chars() {
            if c.is_ascii_alphabetic() {
                col = col * 26 + (c.to_ascii_uppercase() as u32 - ('A' as u32 - 1));
            } else if c.is_ascii_digit() {
                row = row * 10 + (c as u32 - '0' as u32);
            } else {
                return None;
            }
        }

        Some(Self {
            row: row - 1,
            col: col - 1,
            base: (0, 0),
        })
    }

    pub fn from_relative_a1_style(base: (u32, u32), relative_coord: &str) -> Option<Self> {
        if relative_coord.len() < 2 {
            return None;
        }

        let mut col_offset = 0;
        let mut row_offset = 0;
        for c in relative_coord.chars() {
            if c.is_ascii_alphabetic() {
                col_offset = col_offset * 26 + (c.to_ascii_uppercase() as u32 - ('A' as u32 - 1));
            } else if c.is_ascii_digit() {
                row_offset = row_offset * 10 + (c as u32 - '0' as u32);
            } else {
                return None;
            }
        }

        Some(Self {
            row: row_offset - base.0 - 1,
            col: col_offset - base.1 - 1,
            base: (base.0, base.1),
        })
    }

    pub fn to_a1_style(&self) -> String {
        let mut col = self.col + self.base.1 + 1;
        let mut col_str = String::new();
        while col > 0 {
            let rem = (col - 1) % 26;
            col_str.insert(0, (rem as u8 + b'A') as char);
            col = (col - 1) / 26;
        }
        format!("{}{}", col_str, self.row)
    }

    pub fn to_a1_with_base(&self, base: ExcelCoord) -> String {
        let abs_row = self.row + base.row + 1;
        let abs_col = self.col + base.col + 1;

        let mut col = abs_col;
        let mut col_str = String::new();
        while col > 0 {
            let rem = (col - 1) % 26;
            col_str.insert(0, (rem as u8 + b'A') as char);
            col = (col - 1) / 26;
        }
        format!("{}{}", col_str, abs_row)
    }
}

pub fn col_name_to_index(col_name: &str, base: Option<(u32, u32)>) -> Option<u32> {
    let mut col_index = 0;
    for c in col_name.chars() {
        if c.is_ascii_alphabetic() {
            col_index = col_index * 26 + (c.to_ascii_uppercase() as u32 - ('A' as u32 - 1));
        } else {
            return None;
        }
    }
    Some(col_index - base.unwrap_or_default().1 - 1)
}

pub fn from_a1_to_coord(cell_name: &str, base: (u32, u32)) -> Option<(u32, u32)> {
    ExcelCoord::from_relative_a1_style(base, cell_name).map(|c| c.into())
}

pub fn read_cell_value<RS>(
    workbook: &mut calamine::Xlsx<RS>,
    sheet_name: &str,
    cell_name: &str,
) -> anyhow::Result<String>
where
    RS: Seek + Read,
{
    let range = workbook.worksheet_range(sheet_name)?;
    let base_coord = range.start().unwrap_or((0, 0));
    let cell_coord = from_a1_to_coord(cell_name, base_coord).unwrap_or_default();
    let cell_value = range
        .get_value((cell_coord.0, cell_coord.1))
        .map(|v| v.to_string())
        .unwrap_or_default();
    Ok(cell_value)
}
