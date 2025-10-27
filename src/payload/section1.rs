use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Section1 {
    #[serde(rename = "doi_tuong_bao_cao")]
    pub reporting_entity: ReportingEntity,
    #[serde(rename = "nguoi_chiu_trach_nhiem")]
    pub responsible_person: ResponsiblePerson,
    #[serde(rename = "nguoi_lap_bao_cao")]
    pub report_preparer: ReportPreparer,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ReportingEntity {
    #[serde(rename = "ten_doi_tuong_bao_cao")]
    pub name: Option<String>,
    #[serde(rename = "ma_doi_tuong_bao_cao")]
    pub code: Option<String>,
    #[serde(rename = "dia_chi")]
    pub address: Option<Address>,
    #[serde(rename = "dia_diem_phat_sinh")]
    pub transaction_location: TransactionLocation,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Address {
    #[serde(rename = "so_nha")]
    pub street_address: String,
    #[serde(rename = "dien_thoai")]
    pub phone: String,
    #[serde(rename = "quan_huyen")]
    pub district: String,
    #[serde(rename = "tinh_thanh")]
    pub city_province: String,
    #[serde(rename = "quoc_gia")]
    pub country: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TransactionLocation {
    #[serde(rename = "ten_diem_phat_sinh_giao_dich")]
    pub transaction_point_name: String,
    #[serde(rename = "so_nha")]
    pub street_address: String,
    #[serde(rename = "dien_thoai")]
    pub phone: String,
    #[serde(rename = "quan_huyen")]
    pub district: String,
    #[serde(rename = "tinh_thanh")]
    pub city_province: String,
    #[serde(rename = "quoc_gia")]
    pub country: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ResponsiblePerson {
    #[serde(rename = "ho_ten")]
    pub full_name: String,
    #[serde(rename = "dien_thoai_noi_lam_viec")]
    pub work_phone: String,
    #[serde(rename = "dien_thoai_di_dong")]
    pub mobile_phone: String,
    #[serde(rename = "chuc_vu")]
    pub position: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ReportPreparer {
    #[serde(rename = "ho_ten")]
    pub full_name: String,
    #[serde(rename = "dien_thoai_noi_lam_viec")]
    pub work_phone: String,
    #[serde(rename = "dien_thoai_di_dong")]
    pub mobile_phone: String,
    #[serde(rename = "bo_phan_cong_tac")]
    pub department: String,
}
