use aml::{
    auth::get_auth_code,
    launch::launch_web_automation_task,
    payload::{form::Form, section6::Section6},
    response::{ErrorResponse, SuccessResponse},
};
use anyhow::Context;
use duration_extender::DurationExt;
use indicatif::{ProgressBar, ProgressStyle};
use indicatif_log_bridge::LogWrapper;
use std::{
    fs::DirEntry,
    io::{self, BufRead},
};

#[tokio::main]
async fn main() {
    if let Err(err) = _main().await {
        log::error!("Ứng dụng kết thúc với lỗi: {}", err);
    }
}

async fn create_report_from_excel(
    excel_file: &DirEntry,
    api_url: &str,
    auth_key_value: &str,
) -> anyhow::Result<i64> {
    let mut workbook: calamine::Xlsx<_> = calamine::open_workbook(excel_file.path())?;
    let form_payload = Form::from_excel(&mut workbook)?;

    let response = reqwest::Client::new()
        .post(api_url)
        .bearer_auth(&auth_key_value)
        .json(&form_payload)
        .send()
        .await
        .with_context(|| {
            format!(
                "Có lỗi xảy ra khi tải file {:?} lên website NHNN",
                excel_file.path()
            )
        })?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Có lỗi xảy ra khi tải file {:?} lên website NHNN. Mã lỗi: {} - {}: {}",
            excel_file.path(),
            response.status(),
            response.status().canonical_reason().unwrap_or_default(),
            response.text().await.unwrap_or_default()
        ));
    }

    let resp_text = {
        let error_fn = || {
            format!(
                "Có lỗi xảy ra khi tải file {:#?} lên website NHNN - {}.",
                excel_file.path(),
                "Không nhận được phản hồi từ website"
            )
        };
        response.text().await.with_context(error_fn)
    }?;

    if let Ok(err_resp) = serde_json::from_str::<ErrorResponse>(&resp_text) {
        return Err(anyhow::anyhow!(
            "Có lỗi xảy ra khi tải file {:#?} lên website NHNN {}-{}",
            excel_file.path(),
            err_resp.status,
            err_resp.message
        ));
    }

    let report_id = {
        let error_fn = || {
            format!(
                "Có lỗi xảy ra khi tải file {:#?} lên website NHNN - {}",
                excel_file.path(),
                "Không tìm được thông tin mã báo cáo trong phản hồi từ website."
            )
        };

        serde_json::from_str::<SuccessResponse>(&resp_text)
            .with_context(error_fn)?
            .id
            .unwrap_or_default()
    };

    Ok(report_id)
}

async fn save_attachments(
    excel_file: &DirEntry,
    auth_key_value: &str,
    report_id: i64,
) -> anyhow::Result<()> {
    let mut workbook: calamine::Xlsx<_> = calamine::open_workbook(excel_file.path())
        .with_context(|| format!("Lỗi khi mở file {:#?}", excel_file.path()))?;

    let mut attachments = Section6::from_excel(&mut workbook)
        .with_context(|| {
            format!(
                "Lỗi khi đọc/xử lý dữ liệu từ file {:#?} để lưu các file đính kèm",
                excel_file.path()
            )
        })?
        .attachments;

    for attachment in attachments.iter_mut() {
        attachment.str_id = report_id.into();
    }

    let mut body = reqwest::multipart::Form::new()
        .text("strId", report_id.to_string())
        .part(
            "attachments",
            reqwest::multipart::Part::text(serde_json::to_string(&attachments)?)
                .file_name("blob")
                .mime_str("application/json")?,
        );

    for attachment in attachments.into_iter() {
        let file_content = attachment.file_content.unwrap_or_default().clone();
        let file_name = attachment.file_name.unwrap_or_default().clone();
        let file_mime = attachment.file_mime.unwrap_or_default().clone();
        let part_data = reqwest::multipart::Part::bytes(file_content)
            .file_name(file_name.clone())
            .mime_str(&file_mime)
            .with_context(|| format!("Lỗi khi xác định kiểu file của {}", &file_name))?;

        body = body.part("files", part_data);
    }

    let response = reqwest::Client::new()
        .post("https://amlstr.sbv.gov.vn/strcreator/api/attachment/saveAttachment")
        .bearer_auth(&auth_key_value)
        .multipart(body)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Có lỗi xảy ra khi gửi các file đính kèm lên website NHNN file `{:?}` với lỗi: {}-{}",
            excel_file.path(),
            response.status(),
            response.status().canonical_reason().unwrap_or_default()
        ));
    }

    Ok(())
}

fn initial_setup() -> anyhow::Result<ProgressBar> {
    let logger =
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).build();
    let multi_progress = indicatif::MultiProgress::new();
    LogWrapper::new(multi_progress.clone(), logger).try_init()?;

    let progress_bar = multi_progress
        .add(indicatif::ProgressBar::new_spinner())
        .with_message("Processing file...")
        .with_style(ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:60.green/white} {pos:>7}/{len:7} {msg}",
        )?);

    Ok(progress_bar)
}

fn get_input_excel_files() -> anyhow::Result<Vec<DirEntry>> {
    let excel_files = std::fs::read_dir("input")?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().and_then(|s| s.to_str()) == Some("xlsx"))
        .filter(|entry| {
            !entry
                .file_name()
                .to_str()
                .unwrap_or_default()
                .starts_with("~$")
        })
        .collect::<Vec<_>>();

    Ok(excel_files)
}

async fn _main() -> anyhow::Result<()> {
    let progress_bar = initial_setup()?;

    let excel_files = get_input_excel_files()?;
    if excel_files.is_empty() {
        log::warn!(
            "{}. {}",
            "Không tìm thấy file Excel nào trong thư mục `input/`.",
            "Vui lòng thêm file Excel và thử lại."
        );
        return Ok(());
    }
    progress_bar.set_length(excel_files.iter().len() as u64);

    let port = 9515;
    let (_auth_key_name, auth_key_value) = launch_web_automation_task(get_auth_code, port)
        .await
        .with_context(|| format!("Lỗi khi thực hiện mở Chrome để đăng nhập"))?;

    let api_url = "https://amlstr.sbv.gov.vn/strcreator/api/str-creator/saveStrModel?tabNo=0";

    for excel_file in excel_files {
        let report_id = create_report_from_excel(&excel_file, api_url, &auth_key_value)
            .await
            .with_context(|| format!("Lỗi khi tạo báo cáo từ file {:?}", excel_file.path()))?;

        log::info!(
            "Đã nộp biểu mẫu thành công cho file `{:?}`. Mã báo cáo: '{}'.",
            excel_file.path(),
            report_id
        );

        save_attachments(&excel_file, &auth_key_value, report_id)
            .await
            .with_context(|| {
                format!(
                    "Lỗi khi lưu các file đính kèm từ file {:?} cho mã báo cáo '{}'",
                    excel_file.path(),
                    report_id
                )
            })?;

        log::info!(
            "Đã lưu các file đính kèm thành công cho file `{:?}`",
            excel_file.path()
        );

        tokio::time::sleep(1.seconds()).await;
        progress_bar.inc(1);
        progress_bar.set_message(format!(
            "Processing file: {:?}",
            excel_file.path().file_name().unwrap_or_default()
        ));
    }

    progress_bar.finish_with_message("DONE!!!");

    println!("Press Enter to exit...");
    let stdin = io::stdin();
    let _ = stdin.lock().lines().next();

    Ok(())
}
