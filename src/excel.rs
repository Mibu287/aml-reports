use calamine::Reader;
use std::io::{Read, Seek};

use crate::payload;
use crate::utils::excel::from_a1_to_coord;

impl crate::payload::form::Form {
    pub fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        let range = workbook.worksheet_range("STR")?;
        let cell_name = "G3";
        let base_coord = range.start().unwrap_or((0, 0));
        let cell_coord = from_a1_to_coord(cell_name, base_coord).unwrap_or_default();
        let cell_value = range
            .get_value((cell_coord.0, cell_coord.1))
            .map(|v| v.to_string())
            .unwrap_or("NONE".to_string());
        println!("Cell {} value: {}", cell_name, cell_value);

        Ok(payload::form::Form {
            id: None,
            internal_number: internal_number(workbook)?,
            report_type: "M1".to_string(),
            creation_status: payload::form::CreationStatus::InProgress,
            payload: Default::default(),
        })
    }
}

fn internal_number<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<String>
where
    RS: Seek + Read,
{
    let range = workbook.worksheet_range("STR")?;
    let cell_name = "G3";
    let base_coord = range.start().unwrap_or((0, 0));
    let cell_coord = from_a1_to_coord(cell_name, base_coord).unwrap_or_default();
    let cell_value = range
        .get_value((cell_coord.0, cell_coord.1))
        .map(|v| v.to_string())
        .unwrap_or_default();
    Ok(cell_value)
}
