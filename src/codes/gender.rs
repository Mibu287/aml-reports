const GENDER_CODES: [(&'static str, &'static str); 3] =
    [("Nam", "male"), ("Nữ", "female"), ("Khác", "other")];

pub trait GenderCode {
    fn to_gender_code(&self) -> String;
}

impl GenderCode for String {
    fn to_gender_code(&self) -> String {
        GENDER_CODES
            .into_iter()
            .find_map(|(gender_name, gender_code)| {
                if gender_name.eq_ignore_ascii_case(self) {
                    Some(gender_code.to_string())
                } else {
                    None
                }
            })
            .unwrap_or_default()
    }
}

impl GenderCode for Option<String> {
    fn to_gender_code(&self) -> String {
        match self {
            Some(gender) => gender.to_gender_code(),
            None => Default::default(),
        }
    }
}
