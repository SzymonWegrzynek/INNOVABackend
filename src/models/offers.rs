use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct InsertOffer {
    pub title: String,
    pub description: Option<String>,
    pub link: String,
    pub image: String,
}

#[derive(Serialize, Deserialize)]
pub struct Offer {
    pub offer_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub link: String,
    pub image: Vec<u8>,
}
