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
    fn to_occupation_code(&self) -> Option<&'static str>;
    fn to_occupation_code_owned(&self) -> String {
        self.to_occupation_code().unwrap_or_default().to_string()
    }
}

impl OccupationCode for String {
    fn to_occupation_code(&self) -> Option<&'static str> {
        let occupation_code = OCCUPATION_CODES.into_iter().find_map(|(code, occupation)| {
            if occupation.eq_ignore_ascii_case(self) {
                Some(code)
            } else {
                None
            }
        });

        occupation_code
    }
}
