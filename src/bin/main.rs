use aml::{auth::get_auth_code, launch::launch_web_automation_task, payload::form::Form};
use duration_extender::DurationExt;
use indicatif::ProgressStyle;
use std::io::{self, BufRead};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

    let progress_bar = indicatif::ProgressBar::new_spinner()
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
                progress_bar.println(format!(
                    "Error submitting form for file {:?}: {}",
                    excel_file.path(),
                    err
                ));
                continue;
            }
            Ok(r) => match r.status().is_success() {
                false => {
                    progress_bar.println(format!(
                        "Failed to submit form for file {:?}: HTTP {}",
                        excel_file.path(),
                        r.status()
                    ));
                    continue;
                }
                true => r,
            },
        };

        progress_bar.println(format!(
            "Successfully submitted form for file {:?}",
            excel_file.path()
        ));

        tokio::time::sleep(500.milliseconds()).await;
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
