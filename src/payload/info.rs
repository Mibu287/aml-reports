use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GeneralInfo {
    #[serde(rename = "ngay_bao_cao")]
    pub report_date: String,
    #[serde(rename = "so_bao_cao")]
    pub report_number: String,
    #[serde(rename = "sua_doi_bo_sung")]
    pub amendment_supplement: AmendmentSupplement,
    #[serde(rename = "ten_doi_tuong_bao_cao")]
    pub reporting_entity_name: String,
    #[serde(rename = "ma_doi_tuong_bao_cao")]
    pub reporting_entity_code: String,
    #[serde(rename = "mau_bao_cao")]
    pub report_form: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AmendmentSupplement {
    #[serde(rename = "loai_thay_doi")]
    pub change_type: i32,
    #[serde(rename = "so_bao_cao")]
    pub report_number: String,
    #[serde(rename = "ngay_bao_cao")]
    pub report_date: String,
}
