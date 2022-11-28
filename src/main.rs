use std::{fs::File, io::Read};

use actix_files::NamedFile;
use actix_web::{web, HttpRequest, Result};
use parking_lot::Mutex;
use tracing_subscriber::fmt::format::FmtSpan;

use irc::handle_ws;
use twitch_api::UserPool;

use crate::creds::Credentials;

mod creds;

pub static USERS: Mutex<UserPool> = Mutex::new(UserPool { users: Vec::new() });
pub static MESSAGES: Mutex<Vec<String>> = Mutex::new(vec![]);

// TODO: In release builds, include all files from chat frontend in binary

#[macro_use]
extern crate tracing;

#[macro_export]
macro_rules! api_url {
    ($url:literal) => {{
        use const_format::formatcp;

        const URL: &str = formatcp!($url, user_id = env!("TWITCH_USER_ID"));

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

    use creds::CREDENTIALS;

    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::FULL)
        .with_max_level(tracing::Level::INFO)
        .init();

    let creds = {
        use std::fs::{create_dir_all, File};

        let mut creds = CREDENTIALS.lock();

        let dir = directories::ProjectDirs::from("com", "jewelexx", "FauxChat")
            .unwrap_or_else(|| unimplemented!());

        let data_dir = dir.data_dir();

        if !data_dir.exists() {
            create_dir_all(data_dir)?;
        }

        let creds_path = data_dir.join("credentials.toml");

        let creds_raw: Credentials = {
            if creds_path.exists() {
                let mut file_contents = String::new();

                File::open(creds_path)?.read_to_string(&mut file_contents)?;

                toml::from_str(&file_contents)?
            } else {
                let client_id = env!("TWITCH_CLIENT_ID");
                let client_secret = env!("TWITCH_CLIENT_SECRET");
                let user_id = env!("TWITCH_USER_ID");
                let auth_token = env!("TWITCH_AUTH_TOKEN");
                let refresh_token = env!("TWITCH_REFRESH_TOKEN");
            }
        };

        if creds.remain_30().await? {
            creds.refresh().await?;
        }

        creds
    };

    let pool = UserPool::get().await?;

    *USERS.lock() = pool;

    // A file containing one message per line
    // TODO: Add ability to pass custom directory
    let msgs_path = std::env::current_dir().unwrap().join("messages.txt");

    let mut msgs_file = File::open(msgs_path)?;

    let mut msgs_str = String::new();

    msgs_file.read_to_string(&mut msgs_str)?;

    let msgs: Vec<String> = msgs_str.lines().map(String::from).collect();

    *MESSAGES.lock() = msgs;

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
