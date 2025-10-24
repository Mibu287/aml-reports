use crate::payload::entities::{BeneficialOwners, Individual, Organization};
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
