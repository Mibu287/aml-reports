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
    fn to_personal_id_code(&self) -> Option<&'static str>;
    fn to_personal_id_code_owned(&self) -> String {
        self.to_personal_id_code().unwrap_or_default().to_string()
    }
}

impl PersonalIdCode for String {
    fn to_personal_id_code(&self) -> Option<&'static str> {
        let personal_id_code = PERSONAL_ID_CODES.into_iter().find_map(|(code, id_type)| {
            if id_type.eq_ignore_ascii_case(self) {
                Some(code)
            } else {
                None
            }
        });

        personal_id_code
    }
}
