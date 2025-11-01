use std::io::BufRead;

use aml::{
    payload,
    utils::setup::{get_input_excel_files, initial_setup},
};
use anyhow::Context;
use calamine::{Xlsx, open_workbook};

#[tokio::main]
async fn main() {
    if let Err(err) = _main().await {
        log::error!("Đã xảy ra lỗi khi đọc các file Excel: {:?}", err);
    }

    println!("Press Enter to exit...");
    let stdin = std::io::stdin();
    let _ = stdin.lock().lines().next();
}

async fn _main() -> anyhow::Result<()> {
    let progress_bar = initial_setup()?;
    let excel_files = get_input_excel_files()?;
    progress_bar.set_length(excel_files.len() as u64);

    for excel_file in excel_files {
        let excel_path = excel_file.path();

        let mut workbook: Xlsx<_> = open_workbook(excel_path.clone())
            .with_context(|| format!("Không thể mở file {:#?}", excel_path))?;

        let form = payload::form::Form::from_excel(&mut workbook)
            .with_context(|| format!("Lỗi khi đọc và xử lý dữ liệu từ file {:#?}", excel_path))?;

        let _ = serde_json::to_string_pretty(&form).with_context(|| {
            format!(
                "Lỗi khi chuyển đổi dữ liệu thành file {:#?} thành định dạng JSON",
                excel_path
            )
        })?;

        progress_bar.inc(1);
        log::info!("Đã xử lý xong file {:#?}", excel_path);
    }

    progress_bar.finish_with_message("DONE!!!");

    Ok(())
}
