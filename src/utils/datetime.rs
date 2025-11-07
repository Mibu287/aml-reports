use chrono::Datelike;

pub trait ConvertDateFormat {
    fn convert_date_format(
        &self,
        from_format: &str,
        to_format: &str,
    ) -> anyhow::Result<Option<String>>;
    fn convert_date_vn_to_iso(&self) -> anyhow::Result<Option<String>>;
}

impl ConvertDateFormat for &str {
    fn convert_date_format(
        &self,
        from_format: &str,
        to_format: &str,
    ) -> anyhow::Result<Option<String>> {
        let formatted_date = chrono::NaiveDate::parse_from_str(self.trim().as_ref(), from_format)
            .map_err(|_| anyhow::anyhow!("Ngày '{}' không theo định dạng '{}'", self, from_format))?
            .format(to_format)
            .to_string();

        Ok(Some(formatted_date))
    }

    fn convert_date_vn_to_iso(&self) -> anyhow::Result<Option<String>> {
        let from_format = "%d/%m/%Y";
        let to_format = "%Y-%m-%d";

        let date = chrono::NaiveDate::parse_from_str(self.trim().as_ref(), from_format)
            .map_err(|_| anyhow::anyhow!("Ngày '{}' không theo định dạng 'dd/mm/yyyy'", self))?;

        if date.year() < 1900 || date.year() > 2099 {
            return Err(anyhow::anyhow!(
                "Ngày {} nằm ngoài khoảng thời gian hợp lý (01/01/1900 -> 31/12/2099)",
                self
            ));
        }

        let formatted_date = date.format(to_format).to_string();
        Ok(Some(formatted_date))
    }
}

impl ConvertDateFormat for String {
    fn convert_date_format(
        &self,
        from_format: &str,
        to_format: &str,
    ) -> anyhow::Result<Option<String>> {
        (self as &str).convert_date_format(from_format, to_format)
    }

    fn convert_date_vn_to_iso(&self) -> anyhow::Result<Option<String>> {
        (self as &str).convert_date_vn_to_iso()
    }
}

impl ConvertDateFormat for Option<String> {
    fn convert_date_format(
        &self,
        from_format: &str,
        to_format: &str,
    ) -> anyhow::Result<Option<String>> {
        match self {
            Some(s) => s.convert_date_format(from_format, to_format),
            None => Ok(None),
        }
    }

    fn convert_date_vn_to_iso(&self) -> anyhow::Result<Option<String>> {
        match self {
            Some(s) => s.convert_date_vn_to_iso(),
            None => Ok(None),
        }
    }
}
