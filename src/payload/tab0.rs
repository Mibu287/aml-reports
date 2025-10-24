use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tab0 {
    pub id: Option<String>,
    #[serde(rename = "str_internal_number")]
    pub internal_number: String,
    #[serde(rename = "str_type")]
    pub report_type: String,
    pub creation_status: String,
    pub payload: Payload,
}

#[derive(Debug, Serialize, Deserialize)]
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
    #[serde(rename = "Phan_6")]
    pub section_6: Section6,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct AmendmentSupplement {
    #[serde(rename = "loai_thay_doi")]
    pub change_type: i32,
    #[serde(rename = "so_bao_cao")]
    pub report_number: String,
    #[serde(rename = "ngay_bao_cao")]
    pub report_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Section1 {
    #[serde(rename = "doi_tuong_bao_cao")]
    pub reporting_entity: ReportingEntity,
    #[serde(rename = "nguoi_chiu_trach_nhiem")]
    pub responsible_person: ResponsiblePerson,
    #[serde(rename = "nguoi_lap_bao_cao")]
    pub report_preparer: ReportPreparer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportingEntity {
    #[serde(rename = "ten_doi_tuong_bao_cao")]
    pub name: String,
    #[serde(rename = "ma_doi_tuong_bao_cao")]
    pub code: String,
    #[serde(rename = "dia_chi")]
    pub address: Address,
    #[serde(rename = "dia_diem_phat_sinh")]
    pub transaction_location: TransactionLocation,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Section2 {}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Section3 {}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Section4 {}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Section5 {}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Section6 {}
