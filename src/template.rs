use crate::utils::excel::CellAddress;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::{Read, Seek},
    sync::LazyLock,
};

const TEMPLATE_FILE: &str = "report_template.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Table {
    pub sheet: String,
    #[serde(rename = "dòng tiêu đề")]
    pub header_row: u32,
    #[serde(rename = "cột")]
    pub columns: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ExcelParam {
    Address(CellAddress),
    Value(String),
    Table(Table),
    Mapping(HashMap<String, String>),
}

pub fn load_template() -> anyhow::Result<HashMap<String, ExcelParam>> {
    let template = std::fs::read_to_string(TEMPLATE_FILE)?;
    let parsed_result: HashMap<String, ExcelParam> = serde_json::from_str(&template)?;
    Ok(parsed_result)
}

pub static REPORT_TEMPLATE: LazyLock<HashMap<String, ExcelParam>> =
    LazyLock::new(|| load_template().expect("Failed to load report template"));

pub fn cell_value_from_key(
    key: &str,
    workbook: &mut calamine::Xlsx<impl Seek + Read>,
) -> anyhow::Result<String> {
    let cell_addr = match REPORT_TEMPLATE
        .get(key)
        .expect(format!("Cell `{}` not found", key).as_str())
    {
        ExcelParam::Address(addr) => addr,
        ExcelParam::Value(val) => return Ok(val.clone()),
        ExcelParam::Table(_) => {
            return Err(anyhow::anyhow!(
                "Expected cell address for key `{}`, found table definition",
                key
            ));
        }
        ExcelParam::Mapping(_) => {
            return Err(anyhow::anyhow!(
                "Expected cell address for key `{}`, found mapping definition",
                key
            ));
        }
    };

    let cell_value =
        crate::utils::excel::read_cell_value(workbook, &cell_addr.sheet, &cell_addr.cell)?;
    Ok(cell_value)
}

pub fn table_config_from_key(key: &str) -> anyhow::Result<Table> {
    match REPORT_TEMPLATE
        .get(key)
        .expect(format!("Table `{}` not found", key).as_str())
    {
        ExcelParam::Table(table) => Ok(table.clone()),
        _ => Err(anyhow::anyhow!(
            "Expected table definition for key `{}`",
            key
        )),
    }
}

pub fn mapping_from_key(key: &str) -> anyhow::Result<HashMap<String, String>> {
    match REPORT_TEMPLATE
        .get(key)
        .expect(format!("Mapping `{}` not found", key).as_str())
    {
        ExcelParam::Mapping(mapping) => Ok(mapping.clone()),
        _ => Err(anyhow::anyhow!(
            "Expected mapping definition for key `{}`",
            key
        )),
    }
}
