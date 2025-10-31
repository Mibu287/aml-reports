use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::payload::{
    info::GeneralInfo, section1::Section1, section2::Section2, section3::Section3,
    section4::Section4, section5::Section5, section6::Section6,
};

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Default)]
pub enum CreationStatus {
    #[default]
    #[serde(rename = "DANG_NHAP_LIEU")]
    InProgress,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Form {
    pub id: Option<i64>,
    #[serde(rename = "str_internal_number")]
    pub internal_number: String,
    #[serde(rename = "str_type")]
    pub report_type: String,
    pub creation_status: CreationStatus,
    pub payload: Payload,
    #[serde(flatten)]
    pub others: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Payload {
    #[serde(rename = "Thong_tin_chung")]
    pub general_info: GeneralInfo,
    #[serde(rename = "Phan_1")]
    pub section_1: Section1,
    #[serde(rename = "Phan_2")]
    pub section_2: Section2,
    #[serde(rename = "Phan_3")]
    pub section_3: Section3,
    #[serde(rename = "Phan_4")]
    pub section_4: Section4,
    #[serde(rename = "Phan_5")]
    pub section_5: Section5,
}
