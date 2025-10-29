use aml::{auth::get_auth_code, launch::launch_web_automation_task, payload::form::Form};

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

    for excel_file in excel_files {
        let mut workbook: calamine::Xlsx<_> = calamine::open_workbook(excel_file.path())?;
        let form_payload = Form::from_excel(&mut workbook)?;

        let resp = reqwest::Client::new()
            .post(api_url)
            .bearer_auth(&auth_key_value)
            .json(&form_payload)
            .send()
            .await?;

        if !resp.status().is_success() {
            eprintln!("Failed to submit form for file {:?}", excel_file.path());
        } else {
            println!(
                "Successfully submitted form for file {:?}",
                excel_file.path()
            );
        }
    }

    Ok(())
}
