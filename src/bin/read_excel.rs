use aml::payload;
use calamine::{Xlsx, open_workbook};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

    for excel_file in excel_files {
        let excel_path = excel_file.path();
        let mut workbook: Xlsx<_> = open_workbook(excel_path)?;
        let form = payload::form::Form::from_excel(&mut workbook)?;
        let json_form = serde_json::to_string_pretty(&form)?;
        println!("{}", json_form);
    }

    Ok(())
}
