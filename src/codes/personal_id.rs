use crate::codes::utils::search_for_code;

const PERSONAL_ID_CODES: [(&'static str, &'static str); 11] = [
    ("101", "CMTND"),
    ("100", "CCCD"),
    ("103", "Hộ chiếu"),
    ("102", "Định danh cá nhân"),
    ("197", "Thị thực nhập cảnh"),
    ("199", "Giấy tờ khác"),
    ("104", "Giấy Chứng minh sỹ quan quân đội nhân dân"),
    ("105", "Giấy CMND của Quân nhân chuyên nghiệp"),
    ("106", "Giấy chứng minh CAND"),
    ("198", "Giấy tờ có giá trị đi lại quốc tế/ thẻ cư trú"),
    ("107", "Thẻ căn cước"),
];

pub trait PersonalIdCode {
    fn to_personal_id_code(&self) -> anyhow::Result<String>;
}

impl PersonalIdCode for String {
    fn to_personal_id_code(&self) -> anyhow::Result<String> {
        match self.as_str() {
            "" => Ok(String::new()),
            _ => {
                let code = search_for_code(&PERSONAL_ID_CODES, self);
                if code.is_empty() {
                    Err(anyhow::anyhow!(
                        "Loại giấy tờ tùy thân không hợp lệ: {}",
                        self
                    ))
                } else {
                    Ok(code)
                }
            }
        }
    }
}

impl PersonalIdCode for Option<String> {
    fn to_personal_id_code(&self) -> anyhow::Result<String> {
        match self {
            Some(id_type) => id_type.to_personal_id_code(),
            None => Ok(String::new()),
        }
    }
}
