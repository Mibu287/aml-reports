use crate::codes::utils::search_for_code;

const CORPORATE_TYPES: [(&str, &str); 7] = [
    ("1", "Công ty TNHH Một thành viên"),
    ("2", "Công ty TNHH Hai thành viên trở lên"),
    ("3", "Công ty cổ phần"),
    ("4", "Công ty hợp danh"),
    ("5", "Doanh nghiệp tư nhân"),
    ("6", "Tổ chức phi lợi nhuận"),
    ("999", "Khác (tự nhập)"),
];

pub trait CorporateTypeCode {
    fn to_corporate_type_code(&self) -> anyhow::Result<String>;
}

impl CorporateTypeCode for String {
    fn to_corporate_type_code(&self) -> anyhow::Result<String> {
        match self.as_str() {
            "" => Ok(String::new()),
            _ => {
                let code = search_for_code(&CORPORATE_TYPES, &self);
                if code.is_empty() {
                    Err(anyhow::anyhow!(
                        "Loại hình doanh nghiệp không hợp lệ: {}",
                        self
                    ))
                } else {
                    Ok(code)
                }
            }
        }
    }
}

impl CorporateTypeCode for Option<String> {
    fn to_corporate_type_code(&self) -> anyhow::Result<String> {
        match self {
            Some(value) => value.to_corporate_type_code(),
            None => Ok(String::new()),
        }
    }
}
