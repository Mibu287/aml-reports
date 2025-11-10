use crate::codes::utils::search_for_code;

const AMENDMENT_TYPES: [(&'static str, &'static str); 3] =
    [("0", "Không"), ("1", "Bổ sung"), ("2", "Thay thế")];

pub trait AmendmentTypeCode {
    fn to_amendment_type_code(&self) -> anyhow::Result<String>;
}

impl AmendmentTypeCode for String {
    fn to_amendment_type_code(&self) -> anyhow::Result<String> {
        match self.as_str() {
            "" => Ok(String::new()),
            _ => {
                let code = search_for_code(&AMENDMENT_TYPES, &self);
                if code.is_empty() {
                    Err(anyhow::anyhow!(
                        "Loại sửa đổi không hợp lệ: {}. {}: {:?}.",
                        self,
                        "Loại sửa đổi bổ sung hợp lệ bao gồm:",
                        AMENDMENT_TYPES.into_iter().map(|t| t.1).collect::<Vec<_>>()
                    ))
                } else {
                    Ok(code)
                }
            }
        }
    }
}

impl AmendmentTypeCode for Option<String> {
    fn to_amendment_type_code(&self) -> anyhow::Result<String> {
        match self {
            Some(value) => value.to_amendment_type_code(),
            None => Ok(String::new()),
        }
    }
}
