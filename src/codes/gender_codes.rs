const GENDER_CODES: [(&'static str, &'static str); 3] =
    [("Nam", "male"), ("Nữ", "female"), ("Khác", "other")];

pub trait GenderCode {
    fn to_gender_code(&self) -> Option<&'static str>;
    fn to_gender_code_owned(&self) -> String {
        self.to_gender_code().unwrap_or_default().to_string()
    }
}

impl GenderCode for String {
    fn to_gender_code(&self) -> Option<&'static str> {
        GENDER_CODES
            .into_iter()
            .find_map(|(gender_name, gender_code)| {
                if gender_name.eq_ignore_ascii_case(self) {
                    Some(gender_code)
                } else {
                    None
                }
            })
    }
}
