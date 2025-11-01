use crate::codes::utils::search_for_code;

const GENDER_CODES: [(&'static str, &'static str); 3] =
    [("male", "Nam"), ("female", "Nữ"), ("other", "Khác")];

pub trait GenderCode {
    fn to_gender_code(&self) -> anyhow::Result<String>;
}

impl GenderCode for String {
    fn to_gender_code(&self) -> anyhow::Result<String> {
        match self.as_str() {
            "" => Ok(String::new()),
            _ => {
                let code = search_for_code(&GENDER_CODES, &self);
                if code.is_empty() {
                    Err(anyhow::anyhow!("Giới tính không hợp lệ: {}", self))
                } else {
                    Ok(code)
                }
            }
        }
    }
}

impl GenderCode for Option<String> {
    fn to_gender_code(&self) -> anyhow::Result<String> {
        match self {
            Some(gender) => gender.to_gender_code(),
            None => Ok(String::new()),
        }
    }
}
