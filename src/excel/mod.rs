mod section1;
mod section2;
mod section3;
mod section4;
mod section5;
mod section6;

use std::{
    collections::HashMap,
    io::{Read, Seek},
};

use anyhow::Context;
use calamine::{DataType, Reader};

use crate::{
    codes::amendment::AmendmentTypeCode,
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
    template::{cell_value_from_key, table_config_from_key},
    utils::excel::col_name_to_index,
};

impl Form {
    pub fn from_excel<RS>(
        workbook: &mut calamine::Xlsx<RS>,
        file_path: &std::path::Path,
    ) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        let payload = Payload::from_excel(workbook, file_path)?;
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
    pub fn from_excel<RS>(
        workbook: &mut calamine::Xlsx<RS>,
        file_path: &std::path::Path,
    ) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        Ok(Payload {
            general_info: GeneralInfo::from_excel(workbook, file_path)?,
            section_1: Section1::from_excel(workbook, file_path)?,
            section_2: Section2::from_excel(workbook, file_path)?,
            section_3: Section3::from_excel(workbook, file_path)?,
            section_4: Section4::from_excel(workbook, file_path)?,
            section_5: Section5::from_excel(workbook, file_path)?,
            section_6: Section6::from_excel(workbook, file_path)?,
        })
    }
}

impl GeneralInfo {
    pub fn from_excel<RS>(
        workbook: &mut calamine::Xlsx<RS>,
        _file_path: &std::path::Path,
    ) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        Self::_from_excel(workbook, _file_path)
            .with_context(|| format!("Lỗi dữ liệu khi xử lý phần Thông tin chung"))
    }

    fn _from_excel<RS>(
        workbook: &mut calamine::Xlsx<RS>,
        _file_path: &std::path::Path,
    ) -> anyhow::Result<Self>
    where
        RS: Seek + Read,
    {
        let amendment = {
            let change_type = cell_value_from_key(
                "Báo cáo này có bổ sung/thay thế báo cáo nào trước không?",
                workbook,
            )?
            .to_amendment_type_code()
            .with_context(|| format!("Lỗi dữ liệu phần thông tin báo cáo bổ sung/thay thế"))?;

            let report_number = match change_type.as_str() {
                "0" => Default::default(),
                _ => cell_value_from_key("Nếu có, bổ sung/thay thế cho Báo cáo", workbook)?,
            };

            Amendment {
                change_type,
                report_number,
                report_date: String::new(),
            }
        };

        Ok(GeneralInfo {
            report_date: report_date(workbook)?,
            report_number: None,
            amendment: amendment,
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

pub fn read_table_from_sheet<RS>(
    workbook: &mut calamine::Xlsx<RS>,
    sheet_key: &str,
) -> anyhow::Result<(Vec<Vec<String>>, HashMap<String, String>, (u32, u32))>
where
    RS: Seek + Read,
{
    let table_config = table_config_from_key(sheet_key)?;
    let range = workbook.worksheet_range(&table_config.sheet)?;

    // Check table columns matched with config
    table_config
        .columns
        .iter()
        .map(|(expected_header, col_name)| -> anyhow::Result<()> {
            let col_idx = col_name_to_index(col_name, range.start());
            let actual_header = match col_idx {
                None => None,
                Some(idx) => range.get(((table_config.header_row - 1) as usize, idx as usize)),
            }
            .map(|v| v.as_string())
            .flatten();

            let checked_header = match &actual_header {
                None => false,
                Some(actual_header) => {
                    let expected = expected_header.trim().to_lowercase();
                    let actual = actual_header.trim().to_lowercase();
                    let checked = expected.as_str().eq_ignore_ascii_case(&actual);
                    checked
                }
            };

            if !checked_header {
                return anyhow::Result::<()>::Err(anyhow::anyhow!(
                    "Tiêu đề cột {} không đúng. Kỳ vọng: '{}' - Thực tế: '{}'",
                    col_name,
                    expected_header,
                    actual_header.unwrap_or_default()
                ));
            };

            Ok(())
        })
        .fold(anyhow::Result::<()>::Ok(()), |result, element| {
            let _ = result?;
            let _ = element?;
            Ok(())
        })
        .with_context(|| format!("Lỗi kiểm tra định dạng bảng tại sheet '{}'", sheet_key))?;

    let header_row_idx = table_config.header_row - range.start().unwrap_or_default().0 - 1;
    let end_row_idx = {
        let mut end_row_idx = 0;
        for (row_idx, row_content) in range.rows().enumerate() {
            if row_idx as u32 <= header_row_idx {
                end_row_idx = row_idx;
                continue;
            }

            if row_content.iter().all(|value| value.is_empty()) {
                break;
            }

            end_row_idx = row_idx;
        }
        end_row_idx
    };

    let rows: Vec<Vec<String>> = range
        .rows()
        .enumerate()
        .filter(|(row_idx, _)| {
            row_idx.clone() as u32 > header_row_idx && row_idx.clone() <= end_row_idx
        })
        .map(|(_, row_content)| {
            row_content
                .iter()
                .map(|v| v.as_string().unwrap_or_default())
                .collect::<Vec<String>>()
        })
        .collect();

    Ok((
        rows,
        table_config.columns,
        range.start().unwrap_or_default(),
    ))
}

pub fn get_cell_value(
    col_name: &str,
    col_map: &HashMap<String, String>,
    base_coord: (u32, u32),
    curr_row: &Vec<String>,
) -> anyhow::Result<Option<String>> {
    let col_no = col_map
        .get(col_name)
        .cloned()
        .with_context(|| format!("Không tìm thấy cột {}", col_name))?;

    let col_idx = col_name_to_index(&col_no, base_coord.into())
        .with_context(|| format!("Không tìm thấy cột {} {}", col_no, col_name))?;

    let value = curr_row
        .get(col_idx as usize)
        .map(|s| match s.is_empty() {
            true => None,
            false => Some(s.clone()),
        })
        .flatten();

    Ok(value)
}
