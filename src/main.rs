use std::{fs::File, io::Read};

use actix_files::NamedFile;
use actix_web::{web, HttpRequest, Result};
use tracing_subscriber::fmt::format::FmtSpan;

use irc::handle_ws;
use twitch_api::UserPool;

use crate::creds::CREDENTIALS;

mod creds;

lazy_static::lazy_static! {
    pub static ref MESSAGES: Vec<String> = {
        // A file containing one message per line
        // TODO: Add ability to pass custom directory
        let msgs_path = std::env::current_dir().unwrap().join("messages.txt");

        let mut msgs_file = File::open(msgs_path)?;

        let mut msgs: String = String::new();

        msgs_file.read_to_string(&mut msgs)?;

        msgs.lines().collect()
    };
}

#[macro_use]
extern crate tracing;

#[macro_export]
macro_rules! api_url {
    ($url:literal) => {{
        use const_format::formatcp;

        const URL: &str = formatcp!($url, user_id = $crate::creds::CREDENTIALS.user_id);

        formatcp!("https://api.twitch.tv/helix/{}", URL)
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
        .with_max_level(tracing::Level::INFO)
        .init();

    if CREDENTIALS.remain_30().await? {
        CREDENTIALS.refresh().await?;
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
