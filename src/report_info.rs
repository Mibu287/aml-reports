use std::io::{Read, Seek};

use anyhow::Context;
use chrono::Local;

use crate::template::cell_value_from_key;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportInfos {
    pub content: Vec<ReportItem>,
    pub pageable: Pageable,
    pub last: bool,
    pub total_pages: usize,
    pub total_elements: usize,
    pub first: bool,
    pub size: usize,
    pub number: usize,
    pub sort: Sort,
    pub number_of_elements: usize,
    pub empty: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReportItem {
    #[serde(rename = "ngay_bao_cao")]
    pub report_date: chrono::DateTime<Local>,

    #[serde(rename = "so_bao_cao")]
    pub report_number: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pageable {
    pub page_number: usize,
    pub page_size: usize,
    pub sort: Sort,
    pub offset: usize,
    pub paged: bool,
    pub unpaged: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Sort {
    pub empty: bool,
    pub sorted: bool,
    pub unsorted: bool,
}

pub async fn get_report_info(
    workbook: &mut calamine::Xlsx<impl Seek + Read>,
    auth_key_value: &str,
) -> anyhow::Result<ReportInfos> {
    _get_report_info(workbook, auth_key_value)
        .await
        .with_context(
            || "Có lỗi xảy ra khi tải file lấy thông tin các báo cáo hiện hữu từ website NHNN",
        )
}

async fn _get_report_info(
    workbook: &mut calamine::Xlsx<impl Seek + Read>,
    auth_key_value: &str,
) -> anyhow::Result<ReportInfos> {
    let report_entity_code =
        cell_value_from_key("Phần I.1: Thông tin đối tượng báo cáo - Mã", workbook)?;

    let api_url = format!(
        "https://amlstr.sbv.gov.vn/strcreator/api/str-creator/infoReport?report_entity_code={}",
        report_entity_code
    );

    let response = reqwest::Client::new()
        .get(&api_url)
        .bearer_auth(&auth_key_value)
        .send()
        .await?;

    let resp_status = response.status();
    if !resp_status.is_success() {
        return Err(anyhow::anyhow!(
            "Lỗi khi truy vấn thông tin báo cáo hiện hữu tại đường dẫn {}: {} - {}",
            &api_url,
            resp_status.as_u16(),
            resp_status.canonical_reason().unwrap_or_default()
        ));
    }

    let resp_text = response.text().await.with_context(|| {
        format!(
            "Lỗi khi truy vấn thông tin báo cáo hiện hữu tại đường dẫn {}",
            api_url
        )
    })?;

    let report_infos = {
        let context_fn = || {
            format!(
                "Thông tin về báo cáo hiện hữu từ đường dẫn {} không đúng định dạng như kỳ vọng. {}",
                api_url, resp_text
            )
        };

        serde_json::from_str::<ReportInfos>(&resp_text).with_context(context_fn)?
    };

    Ok(report_infos)
}
