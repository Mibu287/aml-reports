pub trait ConvertDateFormat {
    fn convert_date_format(&self, from_format: &str, to_format: &str) -> Option<String>;
    fn convert_date_vn_to_iso(&self) -> Option<String>;
}

impl ConvertDateFormat for &str {
    fn convert_date_format(&self, from_format: &str, to_format: &str) -> Option<String> {
        chrono::NaiveDate::parse_from_str(self.trim().as_ref(), from_format)
            .ok()
            .map(|d| d.format(to_format).to_string())
    }

    fn convert_date_vn_to_iso(&self) -> Option<String> {
        self.convert_date_format("%d/%m/%Y", "%Y-%m-%d")
    }
}

impl ConvertDateFormat for String {
    fn convert_date_format(&self, from_format: &str, to_format: &str) -> Option<String> {
        (self as &str).convert_date_format(from_format, to_format)
    }

    fn convert_date_vn_to_iso(&self) -> Option<String> {
        (self as &str).convert_date_vn_to_iso()
    }
}

impl ConvertDateFormat for Option<String> {
    fn convert_date_format(&self, from_format: &str, to_format: &str) -> Option<String> {
        match self {
            Some(s) => s.convert_date_format(from_format, to_format),
            None => None,
        }
    }

    fn convert_date_vn_to_iso(&self) -> Option<String> {
        match self {
            Some(s) => s.convert_date_vn_to_iso(),
            None => None,
        }
    }
}
