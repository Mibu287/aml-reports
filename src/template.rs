use crate::utils::excel::CellAddress;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{Read, Seek},
    sync::LazyLock,
};

const TEMPLATE_FILE: &str = "report_template.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum CellAddressOrValue {
    Address(CellAddress),
    Value(String),
}

pub fn load_template() -> anyhow::Result<HashMap<String, CellAddressOrValue>> {
    let template = std::fs::read_to_string(TEMPLATE_FILE)?;
    let parsed_result: HashMap<String, CellAddressOrValue> = serde_json::from_str(&template)?;
    Ok(parsed_result)
}

pub static REPORT_TEMPLATE: LazyLock<HashMap<String, CellAddressOrValue>> =
    LazyLock::new(|| load_template().expect("Failed to load report template"));

pub fn cell_value_from_key(
    key: &str,
    workbook: &mut calamine::Xlsx<impl Seek + Read>,
) -> anyhow::Result<String> {
    let cell_addr = match REPORT_TEMPLATE
        .get(key)
        .expect(format!("Cell `{}` not found", key).as_str())
    {
        CellAddressOrValue::Address(addr) => addr,
        CellAddressOrValue::Value(val) => return Ok(val.clone()),
    };

    let cell_value =
        crate::utils::excel::read_cell_value(workbook, &cell_addr.sheet, &cell_addr.cell)?;
    Ok(cell_value)
}
