use rand::{seq::SliceRandom, thread_rng, Rng};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

lazy_static::lazy_static! {
    pub static ref CLIENT: reqwest::Client = {
        use reqwest::header::HeaderValue;
        let mut default_headers = reqwest::header::HeaderMap::new();
        default_headers.insert("Client-Id", HeaderValue::from_str(&crate::creds::CREDENTIALS.lock().client_id).unwrap());
        default_headers.insert("Authorization", HeaderValue::from_str(&format!("Bearer {}", crate::creds::CREDENTIALS.lock().auth_token)).unwrap());

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

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPool {
    pub users: Vec<TwitchUser>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwitchUser {
    pub name: String,
    pub uid: String,
    pub is_mod: bool,
    pub is_vip: bool,
    pub is_sub: bool,
}

pub struct Badges {
    inner: Vec<Badge>,
}

impl Badges {
    pub fn from_user(user: &TwitchUser) -> Self {
        let uid = &crate::creds::CREDENTIALS.lock().user_id;

        let mut badges = Vec::new();

        if uid == &user.uid {
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
            Self::Vip => write!(f, "vip/1"),
            Self::Moderator => write!(f, "moderator/1"),
        }
    }
}

impl std::fmt::Display for Badges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "badges=")?;

        for (i, badge) in self.inner.iter().enumerate() {
            write!(f, "{}", badge)?;

            if i != self.inner.len() - 1 {
                write!(f, ",")?;
            }
        }

        write!(f, ";")?;

        Ok(())
    }
}

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn rand_new() -> Self {
        let mut rng = thread_rng();

        Self::new(rng.gen(), rng.gen(), rng.gen())
    }
}

impl std::fmt::UpperHex for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02X}", self.r)?;
        write!(f, "{:02X}", self.g)?;
        write!(f, "{:02X}", self.b)?;

        Ok(())
    }
}

impl TwitchUser {
    pub fn send_message(&self, message: impl AsRef<str>) -> String {
        let msg = message.as_ref();

        let badges = Badges::from_user(self);

        let mut message = format!(
            "@badge-info={};",
            if self.is_sub { "subscriber/22" } else { "" }
        );

        message.push_str(badges.to_string().as_str());

        message.push_str("client-nonce=6090b7621f1bf7bdcc46777cd522bca1;");

        let color = Color::rand_new();

        message.push_str(&format!("color=#{:X};", color));

        message.push_str(&format!("display-name={};", self.name));

        message.push_str("emotes=;first-msg=0;flags=;id=aedfa462-66b6-4a2b-b94d-afb01d0631f9;");

        message.push_str(&format!("mod={};", if self.is_mod { "1" } else { "0" }));

        message.push_str("returning-chatter=0;");

        message.push_str(const_format::concatcp!(
            "room-id=",
            env!("TWITCH_USER_ID"),
            ";"
        ));

        message.push_str(&format!(
            "subscriber={};",
            if self.is_sub { "1" } else { "0" }
        ));

        let current_time = {
            use std::time::{SystemTime, UNIX_EPOCH};

            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis()
        };

        message.push_str(&format!("tmi-sent-ts={};", current_time));

        message.push_str("turbo=0;");

        message.push_str(&format!("user-id={};", self.uid));

        message.push_str("user-type= :");

        message.push_str(&format!(
            "{}!{}@{}.tmi.twitch.tv PRIVMSG #{} :{}",
            self.name, self.name, self.name, self.name, msg
        ));

        message
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

    pub fn send_message(&self, message: impl AsRef<str>) -> String {
        let mut rng = rand::thread_rng();
        let user = self.users.choose(&mut rng).unwrap();

        user.send_message(message)
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
