use std::collections::HashMap;

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::payload::section6::Attachment;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
struct ValidationResponse {
    errors: HashMap<String, String>,
}

pub async fn _validate_attachment(
    attachment: &Attachment,
    auth_key_value: &str,
) -> anyhow::Result<()> {
    match attachment.attachment_type.as_ref().map(|s| s.as_str()) {
        Some("STM") => {}
        _ => return Ok(()),
    };

    let attachment = attachment.clone();
    let file_content = attachment.file_content.unwrap_or_default();
    let file_name = attachment.file_name.clone().unwrap_or_default();
    let file_mime = attachment.file_mime.unwrap_or_default();
    let part_data = reqwest::multipart::Part::bytes(file_content)
        .file_name(file_name.clone())
        .mime_str(&file_mime)
        .with_context(|| format!("Lỗi khi xác định kiểu file của {}", &file_name))?;

    let body = reqwest::multipart::Form::new().part("file", part_data);

    let api_url = "https://amlstr.sbv.gov.vn/strcreator/api/attachment/validateAttachment";

    let response = reqwest::Client::new()
        .post(api_url)
        .bearer_auth(auth_key_value)
        .multipart(body)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Lỗi khi gửi yêu cầu đến đường dẫn '{}': {}-{}",
            api_url,
            response.status(),
            response.status().canonical_reason().unwrap_or_default()
        ));
    }

    let resp_text = response
        .text()
        .await
        .with_context(|| format!("Không đọc được kết quả gửi về từ đường dẫn '{}'", api_url))?;

    let parsed_text =
        serde_json::from_str::<ValidationResponse>(&resp_text).with_context(|| {
            format!(
                "Định dạng kết quả gửi về từ đường dẫn '{}' bất thường. {}",
                api_url, resp_text
            )
        })?;

    if parsed_text.errors.len() > 0 {
        let err_message = parsed_text
            .errors
            .values()
            .fold(String::new(), |result, element| match result.is_empty() {
                true => element.clone(),
                false => format!("{}{}", result, element),
            });

        if err_message.len() > 120 {
            return Err(anyhow::anyhow!(
                "File '{}' không đúng mẫu bảng kê của NHNN. Vui lòng tải lại mẫu bảng kê và điền lại.",
                attachment.file_name.clone().unwrap_or_default()
            ));
        }

        return Err(anyhow::anyhow!("{}", err_message));
    }

    Ok(())
}

pub async fn validate_attachments(
    attachments: &Vec<Attachment>,
    auth_key_value: &str,
) -> anyhow::Result<()> {
    for attachment in attachments.iter() {
        _validate_attachment(attachment, auth_key_value)
            .await
            .with_context(|| {
                format!(
                    "Có lỗi xảy thực hiện đối chiếu file đính kèm '{}' với mẫu báo cáo của NHNN.",
                    attachment.file_name.clone().unwrap_or_default()
                )
            })?;
    }

    Ok(())
}
