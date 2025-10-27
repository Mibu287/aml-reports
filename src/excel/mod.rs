mod section1;
mod section2;
mod section3;

use std::io::{Read, Seek};

use crate::{
    payload::{
        self,
        form::{Form, Payload},
        info::{Amendment, GeneralInfo},
        section1::Section1,
        section2::Section2,
        section3::Section3,
    },
    template::cell_value_from_key,
};

impl Form {
    pub fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        Ok(Form {
            id: None,
            internal_number: internal_number(workbook)?,
            report_type: "M1".to_string(),
            creation_status: payload::form::CreationStatus::InProgress,
            payload: Payload::from_excel(workbook)?,
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
            section_1: Section1::from_excel(workbook)?,
            section_2: Section2::from_excel(workbook)?,
            section_3: Section3::from_excel(workbook)?,
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
            report_date: report_date(workbook)?,
            report_number: None,
            amendment: Amendment {
                change_type: 0,
                report_number: String::new(),
                report_date: String::new(),
            },
            reporting_entity_name: Some("Ngân hàng TMCP Ngoại thương Việt Nam".to_string()),
            reporting_entity_code: Some("01203001".to_string()),
            report_form: Some("M1".to_string()),
        })
    }
}

fn report_date<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Option<String>>
where
    RS: Seek + Read,
{
    const CELL_KEY: &str = "Ngày báo cáo";
    let cell_value = cell_value_from_key(CELL_KEY, workbook)?;

    let date_value = regex::Regex::new(r"(?ms)(\d{2}).+(\d{2}).+(\d{4})")?
        .captures(&cell_value)
        .map(|caps| format!("{}-{}-{}", &caps[3], &caps[2], &caps[1]));
    Ok(date_value)
}

fn internal_number<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<String>
where
    RS: Seek + Read,
{
    const CELL_KEY: &str = "Mã báo cáo nội bộ";
    let cell_value = cell_value_from_key(CELL_KEY, workbook)?;
    Ok(cell_value)
}
