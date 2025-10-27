use std::io::{Read, Seek};

use crate::payload::{
    self,
    form::{Form, Payload},
    info::GeneralInfo,
};
use crate::utils::excel::read_cell_value;

impl Form {
    pub fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        Ok(payload::form::Form {
            id: None,
            internal_number: internal_number(workbook)?,
            report_type: "M1".to_string(),
            creation_status: payload::form::CreationStatus::InProgress,
            payload: Default::default(),
        })
    }
}

impl Payload {
    pub fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        Ok(Payload {
            general_info: GeneralInfo::from_excel(workbook)?,
            section_1: Default::default(),
            section_2: Default::default(),
            section_3: Default::default(),
            section_4: Default::default(),
            section_5: Default::default(),
            section_6: Default::default(),
        })
    }
}

impl GeneralInfo {
    pub fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        Ok(GeneralInfo {
            report_date: Default::default(),
            report_number: Default::default(),
            amendment_supplement: Default::default(),
            reporting_entity_name: Default::default(),
            reporting_entity_code: Default::default(),
            report_form: Default::default(),
        })
    }
}

fn report_date<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<String>
where
    RS: Seek + Read,
{
    let sheet_name = "STR";
    let cell_name = "B3";
    let cell_value = read_cell_value(workbook, sheet_name, cell_name)?;
    regex::Regex::new(r"(\d{2})-(\d{2})-(\d{4})")
        .unwrap()
        .captures(&cell_value)
        .ok_or_else(|| {
            anyhow::anyhow!("Invalid date format in cell {}: {}", cell_name, cell_value)
        })?;
    Ok(cell_value)
}

fn internal_number<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<String>
where
    RS: Seek + Read,
{
    let sheet_name = "STR";
    let cell_name = "G3";
    let cell_value = read_cell_value(workbook, sheet_name, cell_name)?;
    Ok(cell_value)
}
