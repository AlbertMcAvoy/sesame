use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthError {
    pub msg: String
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.msg)
    }
}