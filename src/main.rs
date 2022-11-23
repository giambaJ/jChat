use std::{fs::File, io::Read};

use actix_files::NamedFile;
use actix_web::{web, HttpRequest, Result};
use tracing_subscriber::fmt::format::FmtSpan;

use crate::{irc::handle_ws, twitch_api::UserPool};

pub struct Credentials {
    pub client_id: &'static str,
    pub client_secret: &'static str,
    pub user_id: &'static str,
    pub auth_token: &'static str,
}

pub const CREDENTIALS: Credentials = Credentials {
    client_id: env!("TWITCH_CLIENT_ID"),
    client_secret: env!("TWITCH_CLIENT_SECRET"),
    user_id: env!("TWITCH_USER_ID"),
    auth_token: env!("TWITCH_AUTH_TOKEN"),
};

#[macro_use]
extern crate tracing;

#[macro_export]
macro_rules! api_url {
    ($url:literal) => {{
        use const_format::formatcp;

        const URL: &str = formatcp!($url, user_id = $crate::CREDENTIALS.user_id);

        const_format::formatcp!("https://api.twitch.tv/helix/{}", URL)
    }};
}

mod irc;
mod twitch_api;

// User follows reference: https://dev.twitch.tv/docs/api/reference#get-users-follows
// And to get user id in the first place: https://dev.twitch.tv/docs/api/reference#get-users

#[actix_web::get("/twitch/{filename:.*}")]
async fn twitch(req: HttpRequest) -> Result<NamedFile> {
    let base_path = std::env::current_dir().unwrap();
    let path = {
        let query = req.match_info().query("filename");

        if query.is_empty() {
            "index.html"
        } else {
            query
        }
    };

    let qualified_path = base_path.join("chat").join(path);

    Ok(NamedFile::open_async(qualified_path).await?)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use actix_web::{App, HttpServer};

    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::FULL)
        .init();

    let user_pool = UserPool::get().await?;

    // A file containing one message per line
    // TODO: Add ability to pass custom directory
    let msgs_path = std::env::current_dir().unwrap().join("messages.txt");

    let mut msgs_file = File::open(msgs_path)?;

    let mut msgs: String = String::new();

    msgs_file.read_to_string(&mut msgs)?;

    for msg in msgs.lines() {
        user_pool.send_message(msg);
    }

    HttpServer::new(|| {
        App::new()
            .service(twitch)
            .route("/ws/", web::get().to(handle_ws))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
