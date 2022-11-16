use serde::{Deserialize, Serialize};

lazy_static::lazy_static! {
    pub static ref CLIENT: reqwest::Client = {
        use reqwest::header::HeaderValue;
        let mut default_headers = reqwest::header::HeaderMap::new();
        default_headers.insert("Client-Id", HeaderValue::from_static(crate::dotenv_vars::TWITCH_CLIENT_ID));
        default_headers.insert("Authorization", HeaderValue::from_str(&format!("Bearer {}", crate::dotenv_vars::TWITCH_AUTH_TOKEN)).unwrap());

        reqwest::Client::builder().default_headers(default_headers)
            .build()
            .unwrap()
    };
}

// Must retrieve list of followers, subscribers, mods, vips, etc. and match against the list of users in the channel

pub struct UserPool {
    pub users: Vec<TwitchUser>,
}

pub struct TwitchUser {
    pub name: String,
    pub is_mod: bool,
    pub is_vip: bool,
    pub is_subscriber: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwitchUsers {
    pub total: i64,
    pub data: Vec<Datum>,
    pub pagination: Option<Pagination>,
}

impl TwitchUsers {
    pub async fn new_with_len(max_length: usize) -> anyhow::Result<Self> {
        let mut total;
        let mut length = 0;
        let mut data = vec![];

        loop {
            let result: TwitchUsers = CLIENT
                .get("https://api.twitch.tv/helix/users/follows")
                .query(&[
                    (
                        "first",
                        match max_length - length {
                            remaining if remaining > 100 => 100,
                            remaining => remaining,
                        }
                        .to_string()
                        .as_str(),
                    ),
                    ("to_id", crate::dotenv_vars::TWITCH_USER_ID),
                ])
                .send()
                .await?
                .json()
                .await?;

            // Not good to set it every single time but it's fine for now
            total = result.total;
            length += result.data.len();
            data.extend(result.data);

            if result.pagination.is_none() || length >= max_length {
                break;
            }
        }

        Ok(Self {
            total,
            data,
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
