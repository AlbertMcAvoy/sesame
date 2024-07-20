use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct ErrorResult {
    pub status_code: u16,
    pub msg: String
}