use std::time::{Duration, SystemTime};

use parking_lot::Mutex;

pub struct Credentials {
    pub client_id: &'static str,
    pub client_secret: &'static str,
    pub user_id: &'static str,
    pub auth_token: &'static str,
    pub refresh_token: &'static str,
}

pub static CREDENTIALS: Mutex<Credentials> = Mutex::new(Credentials {
    client_id: env!("TWITCH_CLIENT_ID"),
    client_secret: env!("TWITCH_CLIENT_SECRET"),
    user_id: env!("TWITCH_USER_ID"),
    auth_token: env!("TWITCH_AUTH_TOKEN"),
    refresh_token: env!("TWITCH_REFRESH_TOKEN"),
});

impl Credentials {
    pub async fn expires_in(&self) -> anyhow::Result<SystemTime> {
        use crate::twitch_api::CLIENT;

        let response: serde_json::Value = CLIENT
            .get("https://id.twitch.tv/oauth2/validate")
            .await?
            .json()
            .await?;

        let expires_in = response["expires_in"].as_u64().ok_or_else(|| {
            anyhow::anyhow!("Could not parse expires_in from response: {:?}", response)
        })?;

        let expires_in_dur = Duration::from_secs(expires_in);

        let now = SystemTime::now() + expires_in_dur;

        Ok(now)
    }

    pub async fn remain_30(&self) -> anyhow::Result<bool> {
        let now = SystemTime::now();
        // Default to now, meaning that it will trigger a refresh
        let expires_in = self.expires_in().await.unwrap_or(now);

        let diff = expires_in.duration_since(now)?;

        Ok(diff < Duration::from_secs(30 * 60))
    }

    pub async fn refresh(&mut self) -> anyhow::Result<bool> {
        unimplemented!()
    }
}
