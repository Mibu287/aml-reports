use calamine::{DataType, Reader};
use std::io::{Read, Seek};

use crate::payload;

impl crate::payload::form::Form {
    pub fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> Self
    where
        RS: Seek + Read,
    {
        let mut form: payload::form::Form = Default::default();

        if let Ok(range) = workbook.worksheet_range("STR") {
            for row in range.rows().skip(1) {
                // Skip header row
                let name: String = row[0].as_string().unwrap_or_default().to_string();
                let age: u32 = row[1].as_f64().unwrap_or(0.0) as u32;
            }
        }

        form
    }
}
