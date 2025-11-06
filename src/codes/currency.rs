use crate::codes::utils::search_for_code;

const CURRENCY_CODES: [(&'static str, &'static str); 9] = [
    ("VND", "VND - Việt Nam Đồng"),
    ("USD", "USD - United States Dollar"),
    ("AUD", "AUD - Australian Dollar"),
    ("CNY", "CNY - Yuan Renminbi"),
    ("EUR", "EUR - Euro"),
    ("GBP", "GBP - Pound Sterling (United Kingdom Pound)"),
    ("JPY", "JPY - Yen"),
    ("KRW", "KRW - Republic of Korean Won"),
    ("SGD", "SGD - Singapore Dollar"),
];

pub trait CurrencyCode {
    fn to_currency_code(&self) -> anyhow::Result<String>;
}

impl CurrencyCode for String {
    fn to_currency_code(&self) -> anyhow::Result<String> {
        match self.as_str() {
            "" => Ok(String::new()),
            currency_name => {
                let currency_code = search_for_code(&CURRENCY_CODES, currency_name);
                if currency_code.is_empty() {
                    Err(anyhow::anyhow!(
                        "Loại tiền tệ không hợp lệ: {}",
                        currency_name
                    ))
                } else {
                    Ok(currency_code)
                }
            }
        }
    }
}

impl CurrencyCode for Option<String> {
    fn to_currency_code(&self) -> anyhow::Result<String> {
        match self {
            Some(currency_name) => currency_name.to_currency_code(),
            None => Ok(String::new()),
        }
    }
}
