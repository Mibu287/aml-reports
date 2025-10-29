const BANK_ACCOUNT_TYPE_CODES: [(&str, &str); 9] = [
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

pub trait BankAccountTypeCode {
    fn to_bank_account_type_code(&self) -> Option<&'static str>;
    fn to_bank_account_type_code_owned(&self) -> String {
        self.to_bank_account_type_code()
            .unwrap_or_default()
            .to_string()
    }
}

impl BankAccountTypeCode for String {
    fn to_bank_account_type_code(&self) -> Option<&'static str> {
        let bank_account_type_code =
            BANK_ACCOUNT_TYPE_CODES
                .into_iter()
                .find_map(|(type_code, type_name)| {
                    if type_name.eq_ignore_ascii_case(self) {
                        Some(type_code)
                    } else {
                        None
                    }
                });

        bank_account_type_code
    }
}
