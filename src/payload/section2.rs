use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BeneficialOwners {
    #[serde(rename = "chu_so_huu_khac")]
    pub other_owners: Option<Vec<PersonShort>>,
    #[serde(rename = "lien_ket_cshhl_ca_nhan")]
    pub individual_links: Option<Vec<IndividualLink>>,
    #[serde(rename = "lien_ket_cshhl_to_chuc")]
    pub organization_links: Option<Vec<OrganizationLink>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct OrganizationLink {
    pub id: Option<i64>,
    #[serde(rename = "ten")]
    pub name: Option<String>,
    #[serde(rename = "so_dinh_danh")]
    pub identification_number: Option<String>,
    #[serde(rename = "nhom_huong_loi")]
    pub benefit_group: Option<GroupBenefits>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GroupBenefits {
    #[serde(rename = "nhom_thong_tin")]
    pub main_group: Option<Vec<PersonRef>>,
    #[serde(rename = "nhom_thong_tin_khac")]
    pub other_group: Option<Vec<PersonRef>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PersonRef {
    #[serde(rename = "ho_ten")]
    pub full_name: Option<String>,
    #[serde(rename = "so_dinh_danh")]
    pub identification_number: Option<String>,
    pub id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Occupation {
    #[serde(rename = "ma_nghe_nghiep")]
    pub occupation_code: Option<String>,
    #[serde(rename = "mo_ta")]
    pub description: Option<String>,
    #[serde(rename = "noi_dung")]
    pub content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Bank {
    pub ma_ngan_hang: Option<String>,
    pub ten_ngan_hang: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CodeDesc {
    pub ma_loai_hinh: Option<String>,
    pub mo_ta: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct License {
    pub so_giay_phep: Option<String>,
    pub ngay_cap: Option<String>,
    pub noi_cap: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct EnterpriseCode {
    pub ma_so: Option<String>,
    pub ngay_cap: Option<String>,
    pub noi_cap: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
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
