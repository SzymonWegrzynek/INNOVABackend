use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub id: usize,
    pub role: String,
    pub exp: usize,
}

#[derive(Serialize, Deserialize)]
pub struct AuthenticationToken {
    pub id: usize,
    pub role: String,
}

#[derive(Serialize, Deserialize)]
pub struct ExtractToken {
    pub role: String,
}
