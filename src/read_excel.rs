use aml::payload;
use calamine::{DataType, Reader, Xlsx, open_workbook};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let path = "input/[2025.10.07] STR_Template.xlsx";
    let mut workbook: Xlsx<_> = open_workbook(path)?;

    let form = payload::form::Form::from_excel(&mut workbook);
    println!("{:#?}", form);

    Ok(())
}
