use serde::{Deserialize, Serialize};

lazy_static::lazy_static! {
    pub static ref CLIENT: reqwest::Client = {
        use reqwest::header::HeaderValue;
        let mut default_headers = reqwest::header::HeaderMap::new();
        default_headers.insert("Client-Id", HeaderValue::from_static(crate::dotenv_vars::TWITCH_CLIENT_ID));
        default_headers.insert("Authorization", HeaderValue::from_static(crate::dotenv_vars::TWITCH_AUTH_TOKEN));

        reqwest::Client::builder().default_headers(default_headers)
            .build()
            .unwrap()
    };
}

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

        while pagination.is_some() && length < max_length {
            // let result: TwitchUsers =
            // let mut users = Self::new(pagination).await?;
            // length += users.data.len();
            // pagination = users.pagination;
        }

        // Temp to allow compilation
        Ok(Self {
            total: 0,
            data: vec![],
            pagination: None,
        })
    }

    pub async fn new() -> anyhow::Result<Self> {
        Self::new_with_len(100).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Datum {
    pub from_id: String,
    pub from_login: String,
    pub from_name: String,
    pub to_id: String,
    pub to_login: String,
    pub to_name: String,
    pub followed_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub cursor: String,
}
