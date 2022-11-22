use rayon::prelude::*;
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

#[derive(Debug, Serialize, Deserialize)]
pub struct TwitchVips {
    pub data: Vec<VipDatum>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VipDatum {
    pub user_id: String,
    pub user_name: String,
    pub user_login: String,
}

pub struct UserPool {
    pub users: Vec<TwitchUser>,
}
pub struct TwitchUser {
    pub name: String,
    pub is_mod: bool,
    pub is_vip: bool,
    pub is_sub: bool,
}

impl UserPool {
    pub async fn get() -> anyhow::Result<Self> {
        let vips: TwitchVips = CLIENT
            .get(crate::api_url!(
                "channels/vips/?broadcaster_id={user_id}&first=100"
            ))
            .send()
            .await?
            .json()
            .await?;

        let mods: TwitchVips = CLIENT
            .get("https://api.twitch.tv/helix/moderation/moderators/")
            .send()
            .await?
            .json()
            .await?;

        // TODO: Add ability to download all subs
        let subs: TwitchVips = CLIENT
            .get(crate::api_url!(
                "subscriptions?broadcaster_id={user_id}&first=100"
            ))
            .send()
            .await?
            .json()
            .await?;

        let users = TwitchUsers::new().await?;

        let pool: UserPool = users
            .data
            .par_iter()
            .map(|user| {
                let mut pooled_user: TwitchUser = TwitchUser {
                    name: user.from_name.clone(),
                    is_mod: false,
                    is_vip: false,
                    is_sub: false,
                };

                if vips.data.par_iter().any(|vip| vip.user_id == user.from_id) {
                    pooled_user.is_vip = true;
                }

                if mods
                    .data
                    .par_iter()
                    .any(|moderator| moderator.user_id == user.from_id)
                {
                    pooled_user.is_mod = true;
                }

                if subs.data.par_iter().any(|sub| sub.user_id == user.from_id) {
                    pooled_user.is_sub = true;
                }

                pooled_user
            })
            .fold(
                || UserPool { users: vec![] },
                |mut pool, user| {
                    pool.users.push(user);
                    pool
                },
            )
            .collect()?;

        Ok(pool)
    }
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
