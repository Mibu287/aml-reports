use crate::codes::utils::search_for_code;

const ACCOUNT_TYPE_CODES: [(&str, &str); 9] = [
    ("CURRE", "TK thanh toán"),
    ("SAVIN", "TK tiết kiệm"),
    ("TERMD", "Tiền gửi có kỳ hạn"),
    ("INVES", "TK đầu tư"),
    ("SECUR", "TK chứng khoán"),
    ("SUSPE", "TK treo"),
    ("CREDI", "TK thẻ tín dụng"),
    ("LENDI", "TK vay"),
    ("CHECK", "TK séc"),
];

pub trait AccountTypeCode {
    fn to_account_type_code(&self) -> anyhow::Result<String>;
}

impl AccountTypeCode for String {
    fn to_account_type_code(&self) -> anyhow::Result<String> {
        match self.as_str() {
            "" => Ok(String::new()),
            _ => {
                let code = search_for_code(&ACCOUNT_TYPE_CODES, &self);
                if code.is_empty() {
                    Err(anyhow::anyhow!("Loại tài khoản không hợp lệ: {}", self))
                } else {
                    Ok(code)
                }
            }
        }
    }
}

impl AccountTypeCode for Option<String> {
    fn to_account_type_code(&self) -> anyhow::Result<String> {
        match self {
            Some(value) => value.to_account_type_code(),
            None => Ok(String::new()),
        }
    }
}
