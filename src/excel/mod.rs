mod section1;
mod section2;
mod section3;
mod section4;
mod section5;
mod section6;

use std::io::{Read, Seek};

use crate::{
    payload::{
        self,
        form::{Form, Payload},
        info::{Amendment, GeneralInfo},
        section1::Section1,
        section2::Section2,
        section3::Section3,
        section4::Section4,
        section5::Section5,
        section6::Section6,
    },
    template::cell_value_from_key,
};

impl Form {
    pub fn from_excel<RS>(workbook: &mut calamine::Xlsx<RS>) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        let payload = Payload::from_excel(workbook)?;
        let detection_date = payload.section_4.detection_date.clone().unwrap_or_default();
        let others_info = [("ngay_phat_hien".to_string(), detection_date.clone())]
            .into_iter()
            .collect();

        Ok(Form {
            id: None,
            internal_number: internal_number(workbook)?,
            report_type: "M1".to_string(),
            creation_status: payload::form::CreationStatus::InProgress,
            payload: payload.into(),
            others: others_info,
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
            section_4: Section4::from_excel(workbook)?,
            section_5: Section5::from_excel(workbook)?,
            section_6: Section6::from_excel(workbook)?,
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
            reporting_entity_name: cell_value_from_key(
                "Phần I.1: Thông tin đối tượng báo cáo - Tên",
                workbook,
            )?
            .into(),
            reporting_entity_code: cell_value_from_key(
                "Phần I.1: Thông tin đối tượng báo cáo - Mã",
                workbook,
            )?
            .into(),
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
