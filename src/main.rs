use std::{collections::VecDeque, fs::File, io::Read, path::PathBuf};

use actix_files::NamedFile;
use actix_web::{web, HttpRequest, Result};
use clap::Parser;
use tracing_subscriber::fmt::format::FmtSpan;

use fauxchat::*;

use irc::handle_ws;
pub use twitch_api::UserPool;

// TODO: In release builds, include all files from chat frontend in binary

#[derive(Debug, Parser)]
struct CmdArgs {
    #[clap(short, long)]
    messages_file: Option<PathBuf>,
}

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

#[actix_web::get("/credentials.js")]
async fn credentials() -> Result<String> {
    let creds = creds::CREDENTIALS.lock();

    let client_id = &creds.client_id;
    let api_token = &creds.auth_token;

    let file = format!(
        r#"
const client_id = "{client_id}";
const credentials = "{api_token}";
        "#
    );

    Ok(file)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use actix_web::{App, HttpServer};

    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::FULL)
        .with_max_level(tracing::Level::INFO)
        .init();

    let args = CmdArgs::parse();

    std::thread::spawn(|| loop {
        use std::io;

        let mut buf = String::new();

        if io::stdin().read_line(&mut buf).is_ok() {
            MESSAGES.lock().push_back(buf);
        }
    });

    init_creds().await?;

    // Must be initialized after credentials
    lazy_static::initialize(&twitch_api::CLIENT);

    {
        let pool = UserPool::get().await?;

        *USERS.lock() = pool;

        // A file containing one message per line
        let msgs_path = {
            let cwd = std::env::current_dir().unwrap();

            if let Some(path) = args.messages_file {
                cwd.join(path)
            } else {
                cwd.join("messages.txt")
            }
        };

        let mut msgs_file = File::open(msgs_path)?;

        let mut msgs_str = String::new();

        msgs_file.read_to_string(&mut msgs_str)?;

        let msgs: VecDeque<String> = msgs_str.lines().map(String::from).collect();

        *MESSAGES.lock() = msgs;
    }

    HttpServer::new(|| {
        App::new()
            .service(twitch)
            .service(credentials)
            .route("/ws/", web::get().to(handle_ws))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
