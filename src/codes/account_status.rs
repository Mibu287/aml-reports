const BANK_ACCOUNT_STATUS_CODES: [(&str, &str); 5] = [
    ("ACTIV", "Đang hoạt động"),
    ("CLOSE", "Đã đóng"),
    ("BLOCK", "Bị phong tỏa"),
    ("IDLES", "Không hoạt động/ Ngủ đông"),
    ("HOLDS", "Đang treo"),
];

pub trait BankAccountStatusCode {
    fn to_account_status_code(&self) -> String;
}

impl BankAccountStatusCode for String {
    fn to_account_status_code(&self) -> String {
        let status_code = BANK_ACCOUNT_STATUS_CODES
            .into_iter()
            .find_map(|(status_code, status_name)| {
                if status_name.eq_ignore_ascii_case(self) {
                    Some(status_code.to_string())
                } else {
                    None
                }
            })
            .unwrap_or_default();

        status_code
    }
}

impl BankAccountStatusCode for Option<String> {
    fn to_account_status_code(&self) -> String {
        match self {
            Some(status) => status.to_account_status_code(),
            None => Default::default(),
        }
    }
}
