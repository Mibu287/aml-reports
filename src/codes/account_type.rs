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
    fn to_account_type_code(&self) -> String;
}

impl AccountTypeCode for String {
    fn to_account_type_code(&self) -> String {
        search_for_code(&ACCOUNT_TYPE_CODES, &self)
    }
}

impl AccountTypeCode for Option<String> {
    fn to_account_type_code(&self) -> String {
        match self {
            Some(value) => value.to_account_type_code(),
            None => String::new(),
        }
    }
}
