use aml::payload;
use calamine::{Xlsx, open_workbook};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let path = "input/[2025.10.07] STR_Template.xlsx";
    let mut workbook: Xlsx<_> = open_workbook(path)?;

    let form = payload::form::Form::from_excel(&mut workbook)?;
    let json_form = serde_json::to_string_pretty(&form)?;
    println!("{}", json_form);

    Ok(())
}
