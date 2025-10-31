use std::fs::DirEntry;

use indicatif::{ProgressBar, ProgressStyle};
use indicatif_log_bridge::LogWrapper;

pub fn initial_setup() -> anyhow::Result<ProgressBar> {
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

pub fn get_input_excel_files() -> anyhow::Result<Vec<DirEntry>> {
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
