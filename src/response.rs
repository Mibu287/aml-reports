use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorResponse {
    pub status: u32,
    pub message: String,
}
