use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TwitchUsers {
    pub total: i64,
    pub data: Vec<Datum>,
    pub pagination: Option<Pagination>,
}

impl TwitchUsers {
    pub async fn new_with_len(max_length: usize) -> anyhow::Result<Self> {
        let mut length = 0;
        let mut pagination: Option<Pagination> = None;
    }

    pub async fn new() -> anyhow::Result<Self> {
        Self::new_with_len(1000).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Datum {
    pub from_id: String,
    pub from_login: String,
    pub from_name: String,
    pub to_id: String,
    pub to_login: To,
    pub to_name: To,
    pub followed_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub cursor: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum To {
    #[serde(rename = "sapphicjewl")]
    Sapphicjewl,
}
