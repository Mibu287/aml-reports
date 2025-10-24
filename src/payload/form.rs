use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Form {
    pub id: Option<String>,
    #[serde(rename = "str_internal_number")]
    pub internal_number: String,
    #[serde(rename = "str_type")]
    pub report_type: String,
    pub creation_status: String,
    pub payload: Payload,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Section2 {
    #[serde(rename = "ca_nhan_thuc_hien_giao_dich")]
    pub individuals: Option<Vec<Individual>>,

    #[serde(rename = "to_chuc_thuc_hien_giao_dich")]
    pub organizations: Option<Vec<Organization>>,

    #[serde(rename = "chu_so_huu_huong_loi")]
    pub beneficial_owners: Option<BeneficialOwners>,

    #[serde(rename = "thong_tin_khac_bo_sung")]
    pub additional_info: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Individual {
    pub id: Option<i64>,
    #[serde(rename = "khach_hang_hien_huu")]
    pub existing_customer: Option<String>,
    #[serde(rename = "ho_ten")]
    pub full_name: Option<String>,
    #[serde(rename = "ngay_sinh")]
    pub date_of_birth: Option<String>,
    #[serde(rename = "do_tuoi")]
    pub age: Option<String>,
    #[serde(rename = "gioi_tinh")]
    pub gender: Option<String>,
    #[serde(rename = "quoc_tich")]
    pub nationality: Option<String>,
    #[serde(rename = "nghe_nghiep")]
    pub occupation: Option<Occupation>,
    #[serde(rename = "chuc_vu")]
    pub position: Option<String>,
    #[serde(rename = "dia_chi_thuong_tru")]
    pub permanent_address: Option<AddrSimple>,
    #[serde(rename = "noi_o_hien_tai")]
    pub current_address: Option<AddrSimple>,
    #[serde(rename = "thong_tin_dinh_danh")]
    pub identifications: Option<Vec<Identification>>,
    #[serde(rename = "so_dien_thoai")]
    pub phone_number: Option<String>,
    #[serde(rename = "trinh_do_van_hoa")]
    pub education_level: Option<String>,
    pub email: Option<String>,
    #[serde(rename = "tai_khoan")]
    pub accounts: Option<Vec<Account>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Organization {
    pub id: Option<i64>,
    #[serde(rename = "khach_hang_hien_huu")]
    pub existing_customer: Option<String>,
    #[serde(rename = "ten_to_chuc")]
    pub name: Option<String>,
    #[serde(rename = "ten_tieng_nuoc_ngoai")]
    pub foreign_name: Option<String>,
    #[serde(rename = "ten_viet_tat")]
    pub short_name: Option<String>,
    #[serde(rename = "loai_hinh_to_chuc")]
    pub organization_type: Option<CodeDesc>,
    #[serde(rename = "dia_chi")]
    pub address: Option<AddrSimple>,
    #[serde(rename = "giay_phep_thanh_lap")]
    pub establishment_license: Option<License>,
    #[serde(rename = "ma_so_doanh_nghiep")]
    pub enterprise_code: Option<EnterpriseCode>,
    #[serde(rename = "nganh_nghe_kinh_doanh")]
    pub business_sector: Option<String>,
    #[serde(rename = "so_dien_thoai")]
    pub phone_number: Option<String>,
    pub website: Option<String>,
    #[serde(rename = "tai_khoan")]
    pub accounts: Option<Vec<Account>>,
    #[serde(rename = "nguoi_dai_dien")]
    pub representatives: Option<Vec<Representative>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BeneficialOwners {
    #[serde(rename = "chu_so_huu_khac")]
    pub other_owners: Option<Vec<PersonShort>>,
    #[serde(rename = "lien_ket_cshhl_ca_nhan")]
    pub individual_links: Option<Vec<IndividualLink>>,
    #[serde(rename = "lien_ket_cshhl_to_chuc")]
    pub organization_links: Option<Vec<OrganizationLink>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PersonShort {
    pub id: Option<i64>,
    #[serde(rename = "ho_ten")]
    pub full_name: Option<String>,
    #[serde(rename = "ngay_sinh")]
    pub date_of_birth: Option<String>,
    #[serde(rename = "do_tuoi")]
    pub age: Option<String>,
    #[serde(rename = "gioi_tinh")]
    pub gender: Option<String>,
    #[serde(rename = "quoc_tich")]
    pub nationality: Option<String>,
    #[serde(rename = "nghe_nghiep")]
    pub occupation: Option<Occupation>,
    #[serde(rename = "chuc_vu")]
    pub position: Option<String>,
    #[serde(rename = "dia_chi_thuong_tru")]
    pub permanent_address: Option<AddrSimple>,
    #[serde(rename = "noi_o_hien_tai")]
    pub current_address: Option<AddrSimple>,
    #[serde(rename = "thong_tin_dinh_danh")]
    pub identifications: Option<Vec<Identification>>,
    #[serde(rename = "so_dien_thoai")]
    pub phone_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IndividualLink {
    #[serde(rename = "ten")]
    pub name: Option<String>,
    #[serde(rename = "so_dinh_danh")]
    pub identification_number: Option<String>,
    pub id: Option<i64>,
    #[serde(rename = "co_chinh_chu")]
    pub is_principal: Option<bool>,
    #[serde(rename = "nhom_huong_loi")]
    pub benefit_group: Option<GroupBenefits>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrganizationLink {
    pub id: Option<i64>,
    #[serde(rename = "ten")]
    pub name: Option<String>,
    #[serde(rename = "so_dinh_danh")]
    pub identification_number: Option<String>,
    #[serde(rename = "nhom_huong_loi")]
    pub benefit_group: Option<GroupBenefits>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupBenefits {
    #[serde(rename = "nhom_thong_tin")]
    pub main_group: Option<Vec<PersonRef>>,
    #[serde(rename = "nhom_thong_tin_khac")]
    pub other_group: Option<Vec<PersonRef>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PersonRef {
    #[serde(rename = "ho_ten")]
    pub full_name: Option<String>,
    #[serde(rename = "so_dinh_danh")]
    pub identification_number: Option<String>,
    pub id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Occupation {
    #[serde(rename = "ma_nghe_nghiep")]
    pub occupation_code: Option<String>,
    #[serde(rename = "mo_ta")]
    pub description: Option<String>,
    #[serde(rename = "noi_dung")]
    pub content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddrSimple {
    #[serde(rename = "so_nha")]
    pub street_address: Option<String>,
    #[serde(rename = "quan_huyen")]
    pub district: Option<String>,
    #[serde(rename = "tinh_thanh")]
    pub city_province: Option<String>,
    #[serde(rename = "quoc_gia")]
    pub country: Option<String>,
    #[serde(rename = "dien_thoai")]
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Identification {
    #[serde(rename = "loai_dinh_danh")]
    pub id_type: Option<String>,
    #[serde(rename = "so_dinh_danh")]
    pub id_number: Option<String>,
    #[serde(rename = "ngay_cap")]
    pub issue_date: Option<String>,
    #[serde(rename = "ngay_het_han")]
    pub expiry_date: Option<String>,
    #[serde(rename = "co_quan_cap")]
    pub issuing_authority: Option<String>,
    #[serde(rename = "noi_cap")]
    pub place_of_issue: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    #[serde(rename = "so_tai_khoan")]
    pub account_number: Option<String>,
    #[serde(rename = "ngan_hang")]
    pub bank: Option<Bank>,
    #[serde(rename = "loai_tien")]
    pub currency_type: Option<String>,
    #[serde(rename = "loai_tai_khoan")]
    pub account_type: Option<String>,
    #[serde(rename = "ngay_mo")]
    pub open_date: Option<String>,
    #[serde(rename = "trang_thai")]
    pub status: Option<String>,
    #[serde(rename = "nguoi_duoc_uy_quyen")]
    pub authorized_persons: Option<Vec<PersonRef>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bank {
    pub ma_ngan_hang: Option<String>,
    pub ten_ngan_hang: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CodeDesc {
    pub ma_loai_hinh: Option<String>,
    pub mo_ta: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct License {
    pub so_giay_phep: Option<String>,
    pub ngay_cap: Option<String>,
    pub noi_cap: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnterpriseCode {
    pub ma_so: Option<String>,
    pub ngay_cap: Option<String>,
    pub noi_cap: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Representative {
    pub id: Option<i64>,
    pub ho_ten: Option<String>,
    pub ngay_sinh: Option<String>,
    pub nghe_nghiep: Option<Occupation>,
    pub chuc_vu: Option<String>,
    #[serde(rename = "dia_chi_thuong_tru")]
    pub permanent_address: Option<AddrSimple>,
    #[serde(rename = "noi_o_hien_tai")]
    pub current_address: Option<AddrSimple>,
    pub so_dien_thoai: Option<String>,
    pub quoc_tich: Option<String>,
    #[serde(rename = "thong_tin_dinh_danh")]
    pub identifications: Option<Vec<Identification>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AmendmentSupplement {
    #[serde(rename = "loai_thay_doi")]
    pub change_type: i32,
    #[serde(rename = "so_bao_cao")]
    pub report_number: String,
    #[serde(rename = "ngay_bao_cao")]
    pub report_date: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Section1 {
    #[serde(rename = "doi_tuong_bao_cao")]
    pub reporting_entity: ReportingEntity,
    #[serde(rename = "nguoi_chiu_trach_nhiem")]
    pub responsible_person: ResponsiblePerson,
    #[serde(rename = "nguoi_lap_bao_cao")]
    pub report_preparer: ReportPreparer,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Section3 {}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Section4 {}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Section5 {}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Section6 {}
