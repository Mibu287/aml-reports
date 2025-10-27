use calamine::{DataType, Reader, Xlsx, open_workbook};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let path = "input/[2025.10.07] STR_Template.xlsx";
    let mut workbook: Xlsx<_> = open_workbook(path)?;

    if let Ok(range) = workbook.worksheet_range("Sheet1") {
        for row in range.rows().skip(1) {
            // Skip header row
            let name: String = row[0].get_string().unwrap_or_default().to_string();
            let age: u32 = row[1].get_float().unwrap_or(0.0) as u32;
            println!("Name: {}, Age: {}", name, age);
        }
    }

    Ok(())
}
