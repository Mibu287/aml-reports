use crate::codes::utils::search_for_code;

pub const OCCUPATION_CODES: [(&'static str, &'static str); 13] = [
    ("1", "Công chức/viên chức"),
    ("2", "Học sinh/sinh viên"),
    ("3", "Giáo viên"),
    ("4", "Nội trợ"),
    ("5", "Tiểu thương"),
    ("6", "Kỹ sư"),
    ("7", "Công nhân"),
    ("8", "Nông dân"),
    ("9", "Lao động tự do"),
    ("10", "Nhân viên văn phòng"),
    ("11", "Hưu trí"),
    ("12", "Bác sĩ"),
    ("999", "Khác (tự nhập)"),
];

pub trait OccupationCode {
    fn to_occupation_code(&self) -> anyhow::Result<String>;
}

impl OccupationCode for String {
    fn to_occupation_code(&self) -> anyhow::Result<String> {
        match self.as_str() {
            "" => Ok(String::new()),
            _ => {
                let code = search_for_code(&OCCUPATION_CODES, &self);
                if code.is_empty() {
                    Err(anyhow::anyhow!("Nghề nghiệp không hợp lệ: {}", self))
                } else {
                    Ok(code)
                }
            }
        }
    }
}

impl OccupationCode for Option<String> {
    fn to_occupation_code(&self) -> anyhow::Result<String> {
        match self {
            Some(occupation) => occupation.to_occupation_code(),
            None => Ok(String::new()),
        }
    }
}
