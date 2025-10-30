use aml::{
    auth::get_auth_code, launch::launch_web_automation_task, payload::form::Form,
    response::ErrorResponse,
};
use duration_extender::DurationExt;
use indicatif::ProgressStyle;
use indicatif_log_bridge::LogWrapper;
use std::io::{self, BufRead};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let multi_progress = {
        let logger =
            env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
                .build();
        let multi_progress = indicatif::MultiProgress::new();
        LogWrapper::new(multi_progress.clone(), logger).try_init()?;
        multi_progress
    };

    let port = 9515;
    let (_auth_key_name, auth_key_value) = launch_web_automation_task(get_auth_code, port).await?;

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

    let api_url = "https://amlstr.sbv.gov.vn/strcreator/api/str-creator/saveStrModel?tabNo=0";

    let progress_bar = multi_progress
        .add(indicatif::ProgressBar::new_spinner())
        .with_message("Processing file...")
        .with_style(ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:60.green/white} {pos:>7}/{len:7} {msg}",
        )?);

    progress_bar.set_length(excel_files.iter().len() as u64);

    for excel_file in excel_files {
        let mut workbook: calamine::Xlsx<_> = calamine::open_workbook(excel_file.path())?;
        let form_payload = Form::from_excel(&mut workbook)?;

        let status = reqwest::Client::new()
            .post(api_url)
            .bearer_auth(&auth_key_value)
            .json(&form_payload)
            .send()
            .await;

        let _resp = match status {
            Err(err) => {
                log::error!(
                    "Có lỗi xảy ra khi xử lý file {:?}: {}",
                    excel_file.path(),
                    err
                );
                continue;
            }
            Ok(r) => match r.status().is_success() {
                false => {
                    let status = r.status();
                    let resp_text = r.text().await.unwrap_or_default();
                    let error = serde_json::from_str::<ErrorResponse>(&resp_text);
                    match error {
                        Ok(e) => log::error!(
                            "Có lỗi xảy ra khi xử lý file {:?}. Mã lỗi {}: {}",
                            excel_file.path(),
                            e.status,
                            e.message
                        ),
                        Err(_) => log::error!(
                            "Có lỗi xảy ra khi xử lý file {:?}. Mã lỗi {}: {}",
                            excel_file.path(),
                            status,
                            resp_text
                        ),
                    };
                    continue;
                }
                true => r,
            },
        };

        log::info!(
            "Successfully submitted form for file {:?}.",
            excel_file.path(),
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
