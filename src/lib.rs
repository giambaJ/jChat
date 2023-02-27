use std::collections::VecDeque;

use parking_lot::Mutex;
use twitch_api::UserPool;

use crate::creds::Credentials;

pub mod command;
pub mod creds;
pub mod irc;
pub mod twitch_api;

#[macro_use]
extern crate tracing;

pub static USERS: Mutex<UserPool> = Mutex::new(UserPool { users: Vec::new() });

lazy_static::lazy_static! {
    pub static ref MESSAGES: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());
}

#[macro_export]
macro_rules! api_url {
    ($url:literal) => {{
        use const_format::formatcp;

        const URL: &str = formatcp!($url, user_id = env!("TWITCH_USER_ID"));

        formatcp!("https://api.twitch.tv/helix/{}", URL)
    }};
}

pub async fn init_creds() -> anyhow::Result<()> {
    use std::{fs::File, io::Read};

    let creds_path = Credentials::get_path()?;

    let mut creds: Credentials = {
        if creds_path.exists() {
            let mut file_contents = String::new();

            File::open(creds_path)?.read_to_string(&mut file_contents)?;

            toml::from_str(&file_contents)?
        } else {
            let client_id = env!("TWITCH_CLIENT_ID").to_string();
            let client_secret = env!("TWITCH_CLIENT_SECRET").to_string();
            let user_id = env!("TWITCH_USER_ID").to_string();
            let auth_token = env!("TWITCH_AUTH_TOKEN").to_string();
            let refresh_token = env!("TWITCH_REFRESH_TOKEN").to_string();

            let creds = Credentials {
                client_id,
                client_secret,
                user_id,
                auth_token,
                refresh_token,
            };

            creds.save()?;

            creds
        }
    };

    if creds.remain_30().await? {
        creds.refresh().await?;
    }

    *creds::CREDENTIALS.lock() = creds;

    Ok(())
}
