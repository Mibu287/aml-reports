use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Attachment {
    #[serde(rename = "strId")]
    pub str_id: Option<i64>,
    pub status: Option<String>,
    #[serde(rename = "attachmentType")]
    pub attachment_type: Option<String>,
    #[serde(rename = "pageCount")]
    pub page_count: Option<i32>,
    pub description: Option<String>,
    #[serde(rename = "fileName")]
    pub file_name: Option<String>,
    #[serde(rename = "fileType")]
    pub file_type: Option<String>,
    #[serde(rename = "fileSize")]
    pub file_size: Option<i64>,
    pub file: Option<()>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Section6 {
    #[serde(flatten)]
    pub attachments: Vec<Attachment>,
}
