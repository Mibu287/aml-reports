use crate::codes::utils::search_for_code;

const DOCUMENT_TYPES: [(&'static str, &'static str); 7] = [
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
    fn validate_document_type(&self) -> anyhow::Result<String>;
}

impl DocumentType for String {
    fn to_document_type(&self) -> anyhow::Result<String> {
        match self.as_str() {
            "" => Ok(String::new()),
            _ => {
                let code = search_for_code(&DOCUMENT_TYPES, &self);
                if code.is_empty() {
                    Err(anyhow::anyhow!("Loại tài liệu không hợp lệ: {}", self))
                } else {
                    Ok(code)
                }
            }
        }
    }

    fn validate_document_type(&self) -> anyhow::Result<String> {
        let check = DOCUMENT_TYPES
            .into_iter()
            .map(|d| d.0)
            .filter(|code| *code == self.as_str())
            .collect::<Vec<_>>();

        if check.len() == 0 {
            Err(anyhow::anyhow!(
                "Loại tài liệu đính kèm không hợp lệ: {}. Danh sách loại tài liệu hợp lệ như sau: {}. {}",
                self,
                DOCUMENT_TYPES
                    .into_iter()
                    .fold(String::new(), |result, (code, name)| {
                        match result.as_str() {
                            "" => format!("{}: {}", code, name),
                            _ => format!("{}, {}: {}", result, code, name),
                        }
                    }),
                "Cần đặt tên file có tiền tố tương ứng. Ví dụ Bảng kê: STM_CIF100_202502.xlsx"
            ))
        } else {
            Ok(self.clone())
        }
    }
}

impl DocumentType for Option<String> {
    fn validate_document_type(&self) -> anyhow::Result<String> {
        match self {
            Some(document_name) => document_name.validate_document_type(),
            None => Ok(String::new()),
        }
    }

    fn to_document_type(&self) -> anyhow::Result<String> {
        match self {
            Some(document_name) => document_name.to_document_type(),
            None => Ok(String::new()),
        }
    }
}
