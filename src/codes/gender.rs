use crate::codes::utils::search_for_code;

const GENDER_CODES: [(&'static str, &'static str); 3] =
    [("Nam", "male"), ("Nữ", "female"), ("Khác", "other")];

pub trait GenderCode {
    fn to_gender_code(&self) -> String;
}

impl GenderCode for String {
    fn to_gender_code(&self) -> String {
        search_for_code(&GENDER_CODES, self)
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
