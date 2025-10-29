const BANK_ACCOUNT_STATUS_CODES: [(&str, &str); 5] = [
    ("ACTIV", "Đang hoạt động"),
    ("CLOSE", "Đã đóng"),
    ("BLOCK", "Bị phong tỏa"),
    ("IDLES", "Không hoạt động/ Ngủ đông"),
    ("HOLDS", "Đang treo"),
];

pub trait BankAccountStatusCode {
    fn to_bank_account_status_code(&self) -> Option<&'static str>;
    fn to_bank_account_status_code_owned(&self) -> String {
        self.to_bank_account_status_code()
            .unwrap_or_default()
            .to_string()
    }
}

impl BankAccountStatusCode for String {
    fn to_bank_account_status_code(&self) -> Option<&'static str> {
        let status_code =
            BANK_ACCOUNT_STATUS_CODES
                .into_iter()
                .find_map(|(status_code, status_name)| {
                    if status_name.eq_ignore_ascii_case(self) {
                        Some(status_code)
                    } else {
                        None
                    }
                });

        status_code
    }
}
