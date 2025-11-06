use chrono::Datelike;

pub trait AgeRangeCode {
    fn to_age_range_code(&self) -> Option<String>;
}

impl AgeRangeCode for String {
    fn to_age_range_code(&self) -> Option<String> {
        let parse_result = chrono::NaiveDate::parse_from_str(self.trim().as_ref(), "%d/%m/%Y");
        let dob = match parse_result {
            Ok(d) => d,
            Err(_) => return None,
        };

        let age = chrono::Utc::now().year() as i32 - dob.year() as i32;

        match age {
            i32::MIN..0 => return None,
            0..20 => "1".to_string(),
            20..30 => "2".to_string(),
            30..40 => "3".to_string(),
            40..50 => "4".to_string(),
            50..=i32::MAX => "5".to_string(),
        }
        .into()
    }
}

impl AgeRangeCode for Option<String> {
    fn to_age_range_code(&self) -> Option<String> {
        match self {
            Some(age_range) => age_range.to_age_range_code(),
            None => None,
        }
    }
}
