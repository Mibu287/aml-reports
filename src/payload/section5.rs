use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Section5 {
	#[serde(rename = "cong_viec_da_xu_ly")]
	pub processed_tasks: Option<Vec<ProcessedTask>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ProcessedTask {
	#[serde(rename = "ma_cong_viec")]
	pub code: Option<String>,
	#[serde(rename = "mo_ta")]
	pub description: Option<String>,
	#[serde(rename = "cong_van")]
	pub documents: Option<Vec<Document>>,
	#[serde(rename = "noi_dung")]
	pub other_content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Document {
	#[serde(rename = "loai_cong_van")]
	pub doc_type: Option<String>,
	#[serde(rename = "so_cong_van")]
	pub doc_number: Option<String>,
	#[serde(rename = "ngay_cong_van")]
	pub doc_date: Option<String>,
	#[serde(rename = "don_vi")]
	pub unit: Option<String>,
}
