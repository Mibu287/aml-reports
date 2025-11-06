use crate::codes::utils::search_for_code;

const ACCOUNT_STATUS_CODES: [(&str, &str); 5] = [
    ("ACTIV", "Đang hoạt động"),
    ("CLOSE", "Đã đóng"),
    ("BLOCK", "Bị phong tỏa"),
    ("IDLES", "Không hoạt động/ Ngủ đông"),
    ("HOLDS", "Đang treo"),
];

pub trait AccountStatusCode {
    fn to_account_status_code(&self) -> anyhow::Result<String>;
}

impl AccountStatusCode for String {
    fn to_account_status_code(&self) -> anyhow::Result<String> {
        match self.as_str() {
            "" => Ok(String::new()),
            status => {
                let status_code = search_for_code(&ACCOUNT_STATUS_CODES, status);
                if status_code.is_empty() {
                    Err(anyhow::anyhow!(
                        "Trạng thái tài khoản không hợp lệ: {}",
                        status
                    ))
                } else {
                    Ok(status_code)
                }
            }
        }
    }
}

impl AccountStatusCode for Option<String> {
    fn to_account_status_code(&self) -> anyhow::Result<String> {
        match self {
            Some(status) => status.to_account_status_code(),
            None => Ok(String::new()),
        }
    }
}
