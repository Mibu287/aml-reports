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
        let account_type_code = ACCOUNT_TYPE_CODES
            .into_iter()
            .find_map(|(type_code, type_name)| {
                if type_name.eq_ignore_ascii_case(self) {
                    Some(type_code.to_string())
                } else {
                    None
                }
            })
            .unwrap_or_default();

        account_type_code
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
