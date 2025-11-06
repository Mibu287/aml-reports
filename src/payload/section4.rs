use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Section4 {
    #[serde(rename = "loai_bao_cao")]
    pub report_type: Option<ReportType>,

    #[serde(rename = "thong_tin_giao_dich")]
    pub transaction_info: Option<TransactionInfo>,

    #[serde(rename = "phan_tich")]
    pub analysis: Option<Analysis>,

    #[serde(rename = "ket_luan")]
    pub conclusions: Option<Vec<ConclusionEntry>>,

    #[serde(rename = "ngay_phat_hien")]
    pub detection_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ReportType {
    #[serde(rename = "dieu_khoan_bao_cao")]
    pub clauses: Option<Vec<Clause>>,

    #[serde(rename = "dau_hieu_dang_ngo")]
    pub suspicious_indicators: Option<Vec<SuspiciousIndicator>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Clause {
    #[serde(rename = "ma_dieu_khoan")]
    pub code: Option<String>,
    #[serde(rename = "mo_ta")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SuspiciousIndicator {
    #[serde(rename = "ma_dau_hieu")]
    pub code: Option<String>,
    #[serde(rename = "mo_ta")]
    pub description: Option<String>,
    #[serde(rename = "noi_dung_khac_dang_ngo")]
    pub other_content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TransactionInfo {
    #[serde(rename = "hien_trang_giao_dich")]
    pub status: Option<String>,

    #[serde(rename = "thoi_gian_giao_dich")]
    pub time_range: Option<TimeRange>,

    #[serde(rename = "so_tien_giao_dich")]
    pub amounts: Option<Vec<AmountEntry>>,

    #[serde(rename = "tong_tien_giao_dich_quy_doi")]
    pub total_converted_amount: Option<f64>,

    #[serde(rename = "dong_tien")]
    pub money_flows: Option<Vec<MoneyFlow>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TimeRange {
    #[serde(rename = "tu_ngay")]
    pub from: Option<String>,
    #[serde(rename = "den_ngay")]
    pub to: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AmountEntry {
    #[serde(rename = "loai_tien")]
    pub currency: Option<String>,
    #[serde(rename = "so_tien")]
    pub amount: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MoneyFlow {
    pub id: Option<String>,
    #[serde(rename = "ten_doi_tuong")]
    pub subject_name: Option<String>,
    #[serde(rename = "so_dinh_danh")]
    pub identification: Option<String>,
    #[serde(rename = "so_tai_khoan")]
    pub account_number: Option<String>,
    #[serde(rename = "ten_ngan_hang")]
    pub bank_name: Option<String>,
    #[serde(rename = "ma_ngan_hang")]
    pub bank_code: Option<String>,
    #[serde(rename = "tong_so_tien_quy_doi_vao")]
    pub total_converted_in: Option<String>,
    #[serde(rename = "tong_so_giao_dich_vao")]
    pub total_transactions_in: Option<String>,
    #[serde(rename = "tong_so_tien_quy_doi_ra")]
    pub total_converted_out: Option<String>,
    #[serde(rename = "tong_so_giao_dich_ra")]
    pub total_transactions_out: Option<String>,
    #[serde(rename = "dong_tien_vao")]
    pub inflows: Option<Vec<FlowEntryIn>>,
    #[serde(rename = "dong_tien_ra")]
    pub outflows: Option<Vec<FlowEntryOut>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FlowEntryIn {
    #[serde(rename = "ho_ten_nguon")]
    pub source_name: Option<String>,
    #[serde(rename = "so_dinh_danh_nguon")]
    pub source_id: Option<String>,
    #[serde(rename = "so_tai_khoan_nguon")]
    pub source_account: Option<String>,
    #[serde(rename = "ten_ngan_hang_nguon")]
    pub source_bank_name: Option<String>,
    #[serde(rename = "ma_ngan_hang_nguon")]
    pub source_bank_code: Option<String>,
    #[serde(rename = "tong_so_tien")]
    pub total_amount: Option<String>,
    #[serde(rename = "tong_so_tien_quy_doi")]
    pub total_converted: Option<String>,
    #[serde(rename = "tong_so_giao_dich")]
    pub total_transactions: Option<String>,
    #[serde(rename = "giao_dich_tu_ngay")]
    pub tx_from: Option<String>,
    #[serde(rename = "giao_dich_den_ngay")]
    pub tx_to: Option<String>,
    #[serde(rename = "loai_tien")]
    pub currency: Option<String>,
    #[serde(rename = "noi_dung")]
    pub content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct FlowEntryOut {
    #[serde(rename = "ho_ten_dich")]
    pub dest_name: Option<String>,
    #[serde(rename = "so_dinh_danh_dich")]
    pub dest_id: Option<String>,
    #[serde(rename = "so_tai_khoan_dich")]
    pub dest_account: Option<String>,
    #[serde(rename = "ten_ngan_hang_dich")]
    pub dest_bank_name: Option<String>,
    #[serde(rename = "ma_ngan_hang_dich")]
    pub dest_bank_code: Option<String>,
    #[serde(rename = "tong_so_tien")]
    pub total_amount: Option<String>,
    #[serde(rename = "tong_so_tien_quy_doi")]
    pub total_converted: Option<String>,
    #[serde(rename = "tong_so_giao_dich")]
    pub total_transactions: Option<String>,
    #[serde(rename = "giao_dich_tu_ngay")]
    pub tx_from: Option<String>,
    #[serde(rename = "giao_dich_den_ngay")]
    pub tx_to: Option<String>,
    #[serde(rename = "loai_tien")]
    pub currency: Option<String>,
    #[serde(rename = "noi_dung")]
    pub content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Analysis {
    #[serde(rename = "phan_tich_chi_tiet")]
    pub detail: Option<String>,
    #[serde(rename = "co_so_phap_ly")]
    pub legal_bases: Option<Vec<LegalBasis>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct LegalBasis {
    #[serde(rename = "loai_bao_cao")]
    pub report_type: Option<String>,
    #[serde(rename = "so_thong_bao")]
    pub notice_number: Option<String>,
    #[serde(rename = "co_so_nghi_ngo")]
    pub basis: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ConclusionEntry {
    #[serde(rename = "ma_toi_pham")]
    pub crime_code: Option<String>,
    #[serde(rename = "mo_ta")]
    pub description: Option<String>,
    #[serde(rename = "noi_dung_khac_toi_pham_dang_ngo")]
    pub other_content: Option<String>,
}
