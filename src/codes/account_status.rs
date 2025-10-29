use crate::codes::utils::search_for_code;

const ACCOUNT_STATUS_CODES: [(&str, &str); 5] = [
    ("ACTIV", "Đang hoạt động"),
    ("CLOSE", "Đã đóng"),
    ("BLOCK", "Bị phong tỏa"),
    ("IDLES", "Không hoạt động/ Ngủ đông"),
    ("HOLDS", "Đang treo"),
];

pub trait AccountStatusCode {
    fn to_account_status_code(&self) -> String;
}

impl AccountStatusCode for String {
    fn to_account_status_code(&self) -> String {
        search_for_code(&ACCOUNT_STATUS_CODES, self)
    }
}

impl AccountStatusCode for Option<String> {
    fn to_account_status_code(&self) -> String {
        match self {
            Some(status) => status.to_account_status_code(),
            None => Default::default(),
        }
    }
}
