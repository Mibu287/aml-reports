use crate::payload::entities::{Individual, Organization};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Section3 {
	#[serde(rename = "ca_nhan_lien_quan")]
	pub related_individuals: Option<Vec<Individual>>,

	#[serde(rename = "to_chuc_lien_quan")]
	pub related_organizations: Option<Vec<Organization>>,

	#[serde(rename = "thong_tin_khac_bo_sung")]
	pub additional_info: Option<String>,
}
