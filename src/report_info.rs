use std::io::{Read, Seek};

use anyhow::Context;

use crate::template::cell_value_from_key;

async fn get_report_info(
    workbook: &mut calamine::Xlsx<impl Seek + Read>,
    auth_key_value: &str,
) -> anyhow::Result<()> {
    _get_report_info(workbook, auth_key_value)
        .await
        .with_context(
            || "Có lỗi xảy ra khi tải file lấy thông tin các báo cáo hiện hữu từ website NHNN",
        )
}

async fn _get_report_info(
    workbook: &mut calamine::Xlsx<impl Seek + Read>,
    auth_key_value: &str,
) -> anyhow::Result<()> {
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

    Ok(())
}
