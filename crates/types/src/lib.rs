use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    pub service: String,
    pub version: String,
}
