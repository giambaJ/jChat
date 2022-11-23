pub struct Credentials {
    pub client_id: &'static str,
    pub client_secret: &'static str,
    pub user_id: &'static str,
    pub auth_token: &'static str,
    pub refresh_token: &'static str,
}

pub const CREDENTIALS: Credentials = Credentials {
    client_id: env!("TWITCH_CLIENT_ID"),
    client_secret: env!("TWITCH_CLIENT_SECRET"),
    user_id: env!("TWITCH_USER_ID"),
    auth_token: env!("TWITCH_AUTH_TOKEN"),
    refresh_token: env!("TWITCH_REFRESH_TOKEN"),
};

impl Credentials {
    pub async fn expires_in(&self) -> anyhow::Result<u64> {
        use crate::twitch_api::CLIENT;

        let response: serde_json::Value = CLIENT
            .get("https://id.twitch.tv/oauth2/validate")
            .await?
            .json()
            .await?;

        let expires_in = response["expires_in"].as_u64().ok_or_else(|| {
            anyhow::anyhow!("Could not parse expires_in from response: {:?}", response)
        })?;

        Ok(expires_in)
    }
}
