use std::collections::VecDeque;

use parking_lot::Mutex;
use twitch_api::UserPool;

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
