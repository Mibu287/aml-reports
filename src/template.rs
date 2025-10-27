use std::{
    collections::HashMap,
    io::{Read, Seek},
    sync::LazyLock,
};

use crate::utils::excel::CellAddress;

const TEMPLATE_FILE: &str = "report_template.json";

pub fn load_template() -> anyhow::Result<HashMap<String, CellAddress>> {
    let template = std::fs::read_to_string(TEMPLATE_FILE)?;
    let parsed_result: HashMap<String, CellAddress> = serde_json::from_str(&template)?;
    Ok(parsed_result)
}

pub static REPORT_TEMPLATE: LazyLock<HashMap<String, CellAddress>> =
    LazyLock::new(|| load_template().expect("Failed to load report template"));

pub fn cell_value_from_key(
    key: &str,
    workbook: &mut calamine::Xlsx<impl Seek + Read>,
) -> anyhow::Result<String> {
    let cell_addr = REPORT_TEMPLATE.get(key).expect("Cell address not found");
    let cell_value =
        crate::utils::excel::read_cell_value(workbook, &cell_addr.sheet, &cell_addr.cell)?;
    Ok(cell_value)
}
