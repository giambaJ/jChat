use rand::seq::SliceRandom;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

lazy_static::lazy_static! {
    pub static ref CLIENT: reqwest::Client = {
        use reqwest::header::HeaderValue;
        let mut default_headers = reqwest::header::HeaderMap::new();
        default_headers.insert("Client-Id", HeaderValue::from_static(crate::creds::CREDENTIALS.client_id));
        default_headers.insert("Authorization", HeaderValue::from_str(&format!("Bearer {}", crate::creds::CREDENTIALS.auth_token)).unwrap());

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

#[derive(Debug)]
pub struct UserPool {
    pub users: Vec<TwitchUser>,
}

#[derive(Debug)]
pub struct TwitchUser {
    pub name: String,
    pub uid: String,
    pub is_mod: bool,
    pub is_vip: bool,
    pub is_sub: bool,
}

const BASE_MESSAGE: &str = "@badge-info=subscriber/1;badges=broadcaster/1,subscriber/3012;client-nonce=6090b7621f1bf7bdcc46777cd522bca1;color=#29DE7A;display-name=sapphicjewl;emotes=;first-msg=0;flags=;id=aedfa462-66b6-4a2b-b94d-afb01d0631f9;mod=0;returning-chatter=0;room-id=538134305;subscriber=1;tmi-sent-ts=1668563455712;turbo=0;user-id=538134305;user-type= :sapphicjewl!sapphicjewl@sapphicjewl.tmi.twitch.tv PRIVMSG #sapphicjewl :monkaS\r\n";

pub struct Badges {
    inner: Vec<Badge>,
}

impl Badges {
    pub fn from_user(user: &TwitchUser) -> Self {
        let uid = crate::CREDENTIALS.user_id;

        let mut badges = Vec::new();

        if uid == user.uid {
            badges.push(Badge::Broadcaster);
        }

        if user.is_mod {
            badges.push(Badge::Moderator);
        }

        if user.is_vip {
            badges.push(Badge::Vip);
        }

        if user.is_sub {
            badges.push(Badge::Subscriber);
        }

        Self { inner: badges }
    }
}

pub enum Badge {
    Broadcaster,
    Subscriber,
    Moderator,
    Vip,
}

impl std::fmt::Display for Badge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Broadcaster => write!(f, "broadcaster/1"),
            Self::Subscriber => write!(f, "subscriber/3012"),
            Self::Moderator => write!(f, "moderator/1"),
            Self::Vip => write!(f, "vip/1"),
        }
    }
}

impl std::fmt::Display for Badges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "badges=")?;

        for badge in &self.inner {
            write!(f, "{},", badge)?;
        }

        write!(f, ";")?;

        Ok(())
    }
}

impl TwitchUser {
    pub fn send_message(&self, message: impl AsRef<str>) {
        let msg = message.as_ref();
    }
}

impl UserPool {
    #[instrument]
    pub async fn get() -> anyhow::Result<Self> {
        let vips: TwitchVips = CLIENT
            .get(crate::api_url!(
                "channels/vips?broadcaster_id={user_id}&first=100"
            ))
            .await?
            .json()
            .await?;

        let mods: TwitchVips = CLIENT
            .get(crate::api_url!(
                "moderation/moderators?broadcaster_id={user_id}&first=100"
            ))
            .await?
            .json()
            .await?;

        // TODO: Add ability to download all subs
        let subs: TwitchVips = CLIENT
            .get(crate::api_url!(
                "subscriptions?broadcaster_id={user_id}&first=100"
            ))
            .await?
            .json()
            .await?;

        let users = TwitchUsers::new().await?;

        let users = users
            .data
            .par_iter()
            .map(|user| {
                let mut pooled_user: TwitchUser = TwitchUser {
                    name: user.from_name.clone(),
                    uid: user.from_id.clone(),
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
            .collect::<Vec<TwitchUser>>();

        Ok(UserPool { users })
    }

    pub fn send_message(&self, message: &str) {
        let mut rng = rand::thread_rng();
        let user = self.users.choose(&mut rng).unwrap();

        info!("{} sent: {}", user.name, message)
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
                .get(crate::api_url!("users/follows?to_id={user_id}&first=100"))
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
