use serde::{Deserialize, Serialize};

use crate::payload::form::Form;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorResponse {
    pub status: u32,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SuccessResponse {
    #[serde(flatten)]
    pub form: Form,
}
