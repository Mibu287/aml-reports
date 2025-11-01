use crate::codes::utils::search_for_code;

const DOCUMENT_TYPES: [(&str, &str); 7] = [
    ("STM", "Bảng kê"),
    ("FLW", "Minh họa dòng tiền"),
    ("REL", "Minh họa mối quan hệ khách hàng"),
    ("TRX", "Chứng từ giao dịch"),
    ("CIF", "Hồ sơ khách hàng"),
    ("ACC", "Hồ sơ mở tài khoản"),
    ("OTH", "Tài liệu khác"),
];

pub trait DocumentType {
    fn to_document_type(&self) -> anyhow::Result<String>;
}

impl DocumentType for String {
    fn to_document_type(&self) -> anyhow::Result<String> {
        match self.as_str() {
            "" => Ok(String::new()),
            _ => {
                let code = search_for_code(&DOCUMENT_TYPES, self);
                if code.is_empty() {
                    Err(anyhow::anyhow!(
                        "Loại tài liệu đính kèm không hợp lệ: {}",
                        self
                    ))
                } else {
                    Ok(code)
                }
            }
        }
    }
}

impl DocumentType for Option<String> {
    fn to_document_type(&self) -> anyhow::Result<String> {
        match self {
            Some(document_name) => document_name.to_document_type(),
            None => Ok(String::new()),
        }
    }
}
